use std::error::Error as StdError;
use std::fmt;

use modifier::Modifier;
use {Response};

pub use err::Error;
pub use hyper::Error as HttpError;
pub use hyper::error::Result as HttpResult;

/// The type of Errors inside and when using Iron.
///
/// IronError informs its receivers of two things:
///
/// * What went wrong
/// * What to do about it
///
/// The `error` field is responsible for informing receivers of which
/// error occured, and receivers may also modify the error field by layering
/// it (building up a cause chain).
///
/// The `response` field provides a tangible action to be taken if this error
/// is not otherwise handled.
#[derive(Debug)]
pub struct IronError {
    /// The underlying error
    ///
    /// This can be layered and will be logged at the end of an errored
    /// request.
    pub error: Box<Error + Send>,

    /// What to do about this error.
    ///
    /// This Response will be used when the error-handling flow finishes.
    pub response: Response
}

impl IronError {
    /// Create a new IronError from an error and a modifier.
    pub fn new<E: Error, M: Modifier<Response>>(e: E, m: M) -> IronError {
        IronError {
            error: Box::new(e),
            response: Response::with(m)
        }
    }
}

impl fmt::Display for IronError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Display::fmt(&*self.error, f)
    }
}

impl StdError for IronError {
    fn description(&self) -> &str {
        self.error.description()
    }

    fn cause(&self) -> Option<&StdError> {
        self.error.cause()
    }
}

// Some errors for common use

/// NotFound Error.
///
/// For use when the cause of an error is that some resource is not found.
///
/// Optionally contains another Error with more detail.
#[derive(Debug, Default)]
pub struct NotFound(pub Option<Box<Error + Send>>);

impl StdError for NotFound {
    fn description(&self) -> &str {
        "Not Found"
    }

    fn cause(&self) -> Option<&StdError> {
        self.0.as_ref().and_then(|e| e.cause())
    }
}

impl fmt::Display for NotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Debug::fmt(self, f)
    }
}

