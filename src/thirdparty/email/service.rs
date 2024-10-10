use std::{
    env, fs,
    path::{Path, PathBuf},
};

use lettre::{
    error::Error,
    message::{header::ContentType, Mailbox},
    transport::smtp::{
        authentication::Credentials,
        client::{Tls, TlsParameters},
    },
    Address, Message, SmtpTransport, Transport,
};
use once_cell::sync::Lazy;

#[derive(Debug)]
struct MailerConfig {
    host: String,
    username: String,
    password: String,
}

impl MailerConfig {
    pub fn new() -> Result<Self, dotenv::Error> {
        let host = dotenv::var("MAILER_HOST").unwrap_or_else(|_| String::from("smtp.gmail.com"));
        let username = dotenv::var("MAILER_EMAIL").unwrap();
        let password = dotenv::var("MAILER_PASSWORD").unwrap();

        Ok(Self {
            host,
            username,
            password,
        })
    }
}

pub struct SendMesageProps {
    pub email: String,
    pub subject: String,
    pub message: String,
    pub content_type: ContentType,
    pub username: String,
}

pub struct MailerService {
    transport: SmtpTransport,
    current_dir: PathBuf,
}

impl MailerService {
    pub fn new() -> Result<Self, Error> {
        let config = MailerConfig::new().unwrap();
        let transport = SmtpTransport::relay(config.host.as_str())
            .unwrap()
            .credentials(Credentials::new(config.username, config.password))
            .port(587)
            .tls(Tls::Required(
                TlsParameters::builder(config.host).build().unwrap(),
            ))
            .build();

        let current_dir = env::current_dir().unwrap();

        Ok(Self {
            transport,
            current_dir,
        })
    }

    async fn send(&self, props: SendMesageProps) -> bool {
        let builder = Message::builder()
            .from(Mailbox::new(
                Some(String::from("Gamer Hub")),
                env::var("MAILER_EMAIL")
                    .expect("MAILER_EMAIL must be set")
                    .parse::<Address>()
                    .unwrap(),
            ))
            .to(Mailbox::new(
                Some(props.username),
                props.email.parse::<Address>().unwrap(),
            ))
            .subject(props.subject)
            .header(props.content_type)
            .body(props.message);

        if let Err(_) = builder {
            return false;
        }

        match self.transport.send(&builder.unwrap()) {
            Ok(_) => true,
            Err(err) => {
                println!("failed to send email: {:?}", err);
                false
            }
        }
    }

    pub async fn send_email_verification(
        &self,
        email: String,
        username: String,
        token: String,
    ) -> bool {
        let content = fs::read_to_string(Path::new(
            &self
                .current_dir
                .join("src/thirdparty/email/assets/email_verification.html"),
        ))
        .unwrap();

        self.send(SendMesageProps {
            email,
            subject: String::from("Confirm your email"),
            message: content.replace(
                "[TARGET_URL]",
                format!(
                    "{}/user/verify?token={}",
                    env::var("PUBLIC_APP_URL")
                        .unwrap_or(String::from("http://localhost:3000"))
                        .as_str(),
                    token.as_str()
                )
                .as_str(),
            ),
            content_type: ContentType::TEXT_HTML,
            username,
        })
        .await
    }
}

pub static MAILER: Lazy<MailerService> = Lazy::new(|| MailerService::new().unwrap());
