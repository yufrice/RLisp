use combine::ParseError;
use failure::{Backtrace, Context, Fail, ResultExt};
use std::fmt::{self, Display};

#[derive(Debug, Fail)]
enum ErrorKind {
    #[fail(display = "{}", err)]
    ParseError { err: String },
    #[fail(display = "{}", err)]
    CompileError { err: String },
}

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}
