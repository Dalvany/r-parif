//! rparif errors
use core::fmt;
use std::error;

use chrono::ParseError;
use json::Error as JsonError;
use reqwest::Error as RequestError;

#[derive(Debug)]
pub enum RParifError {
    /// Error from reqwest lib
    RequestError(RequestError),
    /// Error from json lib
    JsonError(JsonError),
    /// Date error from chrono lib
    DateParseError(ParseError),
    /// String can't be converted into enum value
    /// it contains the wrong token
    UnkownEnumValue(String),
    /// Raised when json response contains an unexpected type
    WrongJsonType {
        /// Expected type (array, object, string, ...etc)
        expected: String,
        /// Actual JSON
        json: String,
    },
    /// Raised when string date can't be converted.  
    /// Accepted string are `hier`, `jour` and `demain`
    UnexpectedDate(String),
    /// Raised when the API call return status code other than 2XX
    CallError {
        /// URL that raise the error
        url: String,
        /// HTTP body of AirParif API error
        body: String,
        /// HTTP status code
        status: u16,
    },
    /// Raised when key doesn't exists in json response
    MissingJsonKey {
        /// Name of the missing key in JSON
        key: String,
        /// Actual JSON
        json: String,
    },
}

impl fmt::Display for RParifError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use RParifError::*;

        match self {
            RequestError(err) => err.fmt(f),
            JsonError(err) => err.fmt(f),
            DateParseError(err) => err.fmt(f),
            UnkownEnumValue(token) => {
                write!(f, "Error parsing enum valu : unexpected value {}", token)
            }
            MissingJsonKey { ref key, ref json } => write!(f, "Missing key {} in {}", key, json),
            UnexpectedDate(value) => write!(
                f,
                "Wrong date : expect on of 'hier', 'jour', 'demain' but got {}",
                value
            ),
            WrongJsonType {
                ref expected,
                json: ref actual,
            } => write!(
                f,
                "Unexpected type value in JSON : expected an {} but got {}",
                expected, actual
            ),
            CallError {
                ref url,
                ref body,
                ref status,
            } => write!(
                f,
                "Unexpected HTTP response : url={}, status={}, body={:?}, ",
                url, status, body
            ),
        }
    }
}

impl error::Error for RParifError {
    fn description(&self) -> &str {
        use RParifError::*;

        match self {
            RequestError(..) => "Error calling HTTP API",
            JsonError(..) => "Error parsing JSON response",
            DateParseError(..) => "Error parsing date",
            UnkownEnumValue(..) => "Error parsing enum value",
            UnexpectedDate(..) => "Wrong date : expect on of 'hier', 'jour', 'demain'",
            WrongJsonType { .. } => "Unexpected type value in JSON",
            CallError { .. } => "Unexpected HTTP response",
            MissingJsonKey { .. } => "Missing key in json",
        }
    }
}

#[doc(hidden)]
impl From<RequestError> for RParifError {
    fn from(err: RequestError) -> Self {
        RParifError::RequestError(err)
    }
}

#[doc(hidden)]
impl From<JsonError> for RParifError {
    fn from(err: JsonError) -> Self {
        RParifError::JsonError(err)
    }
}

#[doc(hidden)]
impl From<ParseError> for RParifError {
    fn from(err: ParseError) -> Self {
        RParifError::DateParseError(err)
    }
}
