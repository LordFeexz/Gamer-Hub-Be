use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct RespBodyProps {
    pub code: u16,
    pub message: String,
    pub data: Value,
}

// Struct untuk `PaginationRespProps`
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationRespProps {
    pub page: u32,
    pub limit: u32,
    pub total_data: u32,
    pub total_page: u32,
}

// Struct untuk `IRespBody`
#[derive(Debug, Serialize, Deserialize)]
pub struct RespBody {
    pub code: u16,
    pub message: String,
    pub status: String,
    pub data: Value,
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub total_data: Option<u32>,
    pub total_page: Option<u32>,
}

pub fn send_response_body(props: RespBodyProps, opts: Option<PaginationRespProps>) -> RespBody {
    let status = status_name()
        .get(&props.code)
        .unwrap_or(&"Unknown Status")
        .to_string();

    let (page, limit, total_data, total_page) = match opts {
        Some(pagination) => (
            Some(pagination.page),
            Some(pagination.limit),
            Some(pagination.total_data),
            Some(pagination.total_page),
        ),
        None => (None, None, None, None),
    };

    RespBody {
        code: props.code,
        message: props.message,
        status,
        data: props.data,
        page,
        limit,
        total_data,
        total_page,
    }
}

pub fn status_name() -> HashMap<u16, &'static str> {
    let mut map = HashMap::new();
    map.insert(100, "Continue");
    map.insert(101, "Switching Protocols");
    map.insert(102, "Processing");
    map.insert(103, "Early Hints");
    map.insert(200, "OK");
    map.insert(201, "Created");
    map.insert(202, "Accepted");
    map.insert(203, "Non Authoritative Info");
    map.insert(204, "No Content");
    map.insert(205, "Reset Content");
    map.insert(206, "Partial Content");
    map.insert(207, "Multi Status");
    map.insert(208, "Already Reported");
    map.insert(226, "IM Used");
    map.insert(300, "Multiple Choices");
    map.insert(301, "Moved Permanently");
    map.insert(302, "Found");
    map.insert(303, "See Other");
    map.insert(304, "Not Modified");
    map.insert(305, "Use Proxy");
    map.insert(307, "Temporary Redirect");
    map.insert(308, "Permanent Redirect");
    map.insert(400, "Bad Request");
    map.insert(401, "Unauthorized");
    map.insert(402, "Payment Required");
    map.insert(403, "Forbidden");
    map.insert(404, "Not Found");
    map.insert(405, "Method Not Allowed");
    map.insert(406, "Not Acceptable");
    map.insert(407, "Proxy Auth Required");
    map.insert(408, "Request Timeout");
    map.insert(409, "Conflict");
    map.insert(410, "Gone");
    map.insert(411, "Length Required");
    map.insert(412, "Precondition Failed");
    map.insert(413, "Request Entity Too Large");
    map.insert(414, "Request URI Too Long");
    map.insert(415, "Unsupported Media Type");
    map.insert(416, "Request Range Not Satisfiable");
    map.insert(417, "Expectation Failed");
    map.insert(418, "Teapot");
    map.insert(421, "Misdirected Request");
    map.insert(422, "Unprocessable Entity");
    map.insert(423, "Locked");
    map.insert(424, "Failed Dependency");
    map.insert(425, "Too Early");
    map.insert(426, "Upgrade Required");
    map.insert(429, "Too Many Requests");
    map.insert(431, "Request Header Fields Too Large");
    map.insert(451, "Unavailable For Legal Reasons");
    map.insert(500, "Internal Server Error");
    map.insert(501, "Not Implemented");
    map.insert(502, "Bad Gateway");
    map.insert(503, "Service Unavailable");
    map.insert(504, "Gateway Timeout");
    map.insert(505, "HTTP Version Not Supported");
    map.insert(506, "Variant Also Negotiates");
    map.insert(507, "Insufficient Storage");
    map.insert(508, "Loop Detected");
    map.insert(510, "Not Extended");
    map.insert(511, "Network Authentication Required");

    map
}
