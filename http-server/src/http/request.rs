use super::{
    method::{Method, MethodError},
    query_string::QueryString,
    QueryStringValue,
};
use std::{
    convert::{From, TryFrom},
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    str::{from_utf8, Utf8Error},
};

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request = match from_utf8(buf) {
            Ok(request) => request,
            Err(err) => return Err(ParseError::InvalidEncoding),
        };

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidEncoding)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidEncoding)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidEncoding)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;

        if let Some(i) = path.find("?") {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
}

#[derive(Debug)]
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidMethod,
    InvalidProtocol,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidProtocol => "Invalid Protocol Error",
            Self::InvalidRequest => "Invalid Request Error",
            Self::InvalidEncoding => "Invalid Encoding Error",
            Self::InvalidMethod => "Invalid Method Error",
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "SuperError is here!")
    }
}

impl Error for ParseError {}
