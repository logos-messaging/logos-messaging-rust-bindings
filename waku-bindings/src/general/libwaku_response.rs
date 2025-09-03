use crate::general::waku_decode::WakuDecode;
use crate::general::Result;
use std::convert::TryFrom;
use std::str;
use waku_sys::{RET_ERR, RET_MISSING_CALLBACK, RET_OK};

#[derive(Debug, Clone, Default, PartialEq)]
pub enum LibwakuResponse {
    Success(Option<String>),
    Failure(String),
    MissingCallback,
    #[default]
    Undefined,
}

impl TryFrom<(u32, &str)> for LibwakuResponse {
    type Error = String;

    fn try_from((ret_code, response): (u32, &str)) -> std::result::Result<Self, Self::Error> {
        let opt_value = Some(response.to_string()).filter(|s| !s.is_empty());
        match ret_code {
            RET_OK => Ok(LibwakuResponse::Success(opt_value)),
            RET_ERR => Ok(LibwakuResponse::Failure(format!(
                "waku error: {}",
                response
            ))),
            RET_MISSING_CALLBACK => Ok(LibwakuResponse::MissingCallback),
            _ => Err(format!("undefined return code {}", ret_code)),
        }
    }
}

/// Used in cases where the FFI call doesn't return additional information in the
/// callback. Instead, it returns RET_OK, RET_ERR, etc.
pub(crate) fn handle_no_response(code: i32, result: LibwakuResponse) -> Result<()> {
    if result == LibwakuResponse::Undefined && code as u32 == RET_OK {
        // Some functions will only execute the callback on error
        return Ok(());
    }

    match result {
        LibwakuResponse::Success(_) => Ok(()),
        LibwakuResponse::Failure(v) => Err(v),
        LibwakuResponse::MissingCallback => panic!("callback is required"),
        LibwakuResponse::Undefined => panic!(
            "undefined ffi state: code({}) was returned but callback was not executed",
            code
        ),
    }
}

/// Used in cases where the FFI function returns a code (RET_OK, RET_ERR, etc) plus additional
/// information, i.e. LibwakuResponse
pub(crate) fn handle_response<F: WakuDecode>(code: i32, result: LibwakuResponse) -> Result<F> {
    match result {
        LibwakuResponse::Success(v) => WakuDecode::decode(&v.unwrap_or_default()),
        LibwakuResponse::Failure(v) => Err(v),
        LibwakuResponse::MissingCallback => panic!("callback is required"),
        LibwakuResponse::Undefined => panic!(
            "undefined ffi state: code({}) was returned but callback was not executed",
            code
        ),
    }
}
