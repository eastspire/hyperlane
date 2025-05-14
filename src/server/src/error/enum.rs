use super::*;

#[derive(Debug)]
pub enum ServerError {
    TcpBindError(String),
    HttpReadError(String),
    InvalidHttpRequest(RequestError),
    Unknown,
}
