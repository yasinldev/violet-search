/*
    This folder contains all the exceptions that can be thrown by the microservice library.
*/

#[derive(Debug)]
pub enum VioletSearchExceptions {
    VioletSearchTooManyRequestsException(String, Option<i32>), // (message, retry_after)
    VioletSearchInvalidRequestException(String), // (message)
    VioletSearchInvalidResponseException(String), // (message)
    VioletSearchInvalidParameterException(String), // (message)
    VioletSearchUndefinedException(String), // (message)
    VioletSearchGlobalException(String), // (message)
}

impl VioletSearchExceptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            VioletSearchExceptions::VioletSearchTooManyRequestsException(message, Some(retry_after)) => {
                write!(f, "VioletSearchTooManyRequestsException: {} Retry After: {} seconds", message, retry_after)
            },
            VioletSearchExceptions::VioletSearchInvalidRequestException(message) => {
                write!(f, "VioletSearchInvalidRequestException: {}", message)
            },
            VioletSearchExceptions::VioletSearchInvalidResponseException(message) => {
                write!(f, "VioletSearchInvalidResponseException: {}", message)
            },
            VioletSearchExceptions::VioletSearchInvalidParameterException(message) => {
                write!(f, "VioletSearchInvalidParameterException: {}", message)
            },
            VioletSearchExceptions::VioletSearchGlobalException(message) => {
                write!(f, "VioletSearchGlobalException: {}", message)
            },
            VioletSearchExceptions::VioletSearchUndefinedException(message) => {
                write!(f, "VioletSearchUndefinedException: {}", message)
            }
            _ => { panic!("VioletSearchExceptions::fmt() called with an invalid exception type.") }
        }
    }
}

impl VioletSearchExceptions {
    pub(crate) fn throw_exception(&self) -> ! {
        panic!("{:?}", self);
    }
}

pub fn throw_violet_search_exception(exception: VioletSearchExceptions) -> ! {
    exception.throw_exception();
}
