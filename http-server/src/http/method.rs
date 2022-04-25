use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    DELETE,
    PATCH,
    PUT,
    CONNECT,
    TRACE,
    HEAD,
    OPTIONS,
}

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "DELETE" => Ok(Self::DELETE),
            "PATCH" => Ok(Self::PATCH),
            "PUT" => Ok(Self::PUT),
            "CONNECT" => Ok(Self::CONNECT),
            "TRACE" => Ok(Self::TRACE),
            "HEAD" => Ok(Self::HEAD),
            "OPTIONS" => Ok(Self::OPTIONS),
            _ => Err(MethodError),
        }
    }
}

pub struct MethodError;
