use super::{service, structs::CreateUserProps, validate::RegisterUser};
use crate::helpers::jwt::TokenValue;
use crate::helpers::response::{send_response_body, send_validation_error_response, RespBodyProps};
use crate::models::user::UserStatus;
use crate::modules::wallet::service as wallet_service;
use crate::thirdparty::email::service::MAILER;
use crate::AppState;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{post, web, HttpResponse, Responder};
use bcrypt::{hash, DEFAULT_COST};
use tokio::spawn;
use validator::Validate;

pub fn user_handler(conf: &mut web::ServiceConfig) {
    conf.service(
        web::scope("/user").service(
            web::scope("")
                .wrap(Governor::new(
                    &GovernorConfigBuilder::default()
                        .requests_per_minute(10)
                        .burst_size(5)
                        .finish()
                        .unwrap(),
                ))
                .service(register),
        ),
    );
}

#[post("/register")]
async fn register(body: web::Json<RegisterUser>, data: web::Data<AppState>) -> impl Responder {
    if let Err(err) = body.validate() {
        return HttpResponse::BadRequest().json(send_validation_error_response(err));
    }

    let RegisterUser {
        username,
        email,
        password,
    } = body.into_inner();

    let exists = service::find_unique(&*data.client, &username, &email).await;
    if exists.len() > 0 {
        for (_, existing_username, existing_email) in exists {
            if existing_email == email {
                return HttpResponse::Conflict().json(send_response_body(
                    RespBodyProps {
                        code: 409,
                        message: String::from("email already exists"),
                        data: None,
                        errors: None,
                    },
                    None,
                ));
            }
            if existing_username == username {
                return HttpResponse::Conflict().json(send_response_body(
                    RespBodyProps {
                        code: 409,
                        message: String::from("username already exists"),
                        data: None,
                        errors: None,
                    },
                    None,
                ));
            }
        }
    }

    let hashed_password = hash(password, DEFAULT_COST);
    if let Err(_) = hashed_password {
        return HttpResponse::InternalServerError().json(send_response_body(
            RespBodyProps {
                code: 500,
                message: String::from("internal server error"),
                data: None,
                errors: None,
            },
            None,
        ));
    }

    let start_transaction = data.client.begin().await;
    if let Err(_) = start_transaction {
        return HttpResponse::BadGateway().json(send_response_body(
            RespBodyProps {
                code: 502,
                message: String::from("failed to open connection to database"),
                data: None,
                errors: None,
            },
            None,
        ));
    }

    let mut transaction = start_transaction.unwrap();
    let inserted_user = service::create_one(
        &mut *transaction,
        CreateUserProps {
            username: username.replace(" ", "-"),
            email: email.clone(),
            password: hashed_password.unwrap(),
            is_verified: false,
            status: UserStatus::ACTIVE,
        },
    )
    .await;

    match inserted_user {
        Ok(id) => {
            if let Err(_) = wallet_service::create_wallet(&mut *transaction, id).await {
                let _ = transaction.rollback().await;
                return HttpResponse::BadGateway().json(send_response_body(
                    RespBodyProps {
                        code: 502,
                        message: String::from("failed to create wallet"),
                        data: None,
                        errors: None,
                    },
                    None,
                ));
            }
        }
        Err(_) => {
            let _ = transaction.rollback().await;
            return HttpResponse::BadGateway().json(send_response_body(
                RespBodyProps {
                    code: 502,
                    message: String::from("failed to create user"),
                    data: None,
                    errors: None,
                },
                None,
            ));
        }
    }

    if let Err(_) = transaction.commit().await {
        return HttpResponse::BadGateway().json(send_response_body(
            RespBodyProps {
                code: 502,
                message: String::from("failed to commit transaction"),
                data: None,
                errors: None,
            },
            None,
        ));
    }

    spawn(async move {
        if !MAILER
            .send_email_verification(
                email.clone(),
                username,
                data.jwt_handler
                    .create_token(
                        TokenValue {
                            id: inserted_user.unwrap().to_string(),
                            is_verified: false,
                            is_admin: false,
                            discord_data: None,
                        },
                        3600,
                    )
                    .unwrap(),
            )
            .await
        {
            println!("failed to send email verification to {}", email);
        }
    });

    HttpResponse::Created().json(send_response_body(
        RespBodyProps {
            code: 201,
            message: String::from("Created"),
            data: None,
            errors: None,
        },
        None,
    ))
}
