use ctap::FidoError;
use std::io;

pub type Fido2LuksResult<T> = Result<T, Fido2LuksError>;

#[derive(Debug, Fail)]
pub enum Fido2LuksError {
    #[fail(display = "unable to retrieve password: {}", cause)]
    AskPassError { cause: AskPassError },
    #[fail(display = "unable to read keyfile: {}", cause)]
    KeyfileError { cause: io::Error },
    #[fail(display = "authenticator error: {}", cause)]
    AuthenticatorError { cause: ctap::FidoError },
    #[fail(display = "no authenticator found, please ensure your device is plugged in")]
    NoAuthenticatorError,
    #[fail(display = "luks err")]
    LuksError {
        cause: libcryptsetup_rs::LibcryptErr,
    },
    #[fail(display = "no authenticator found, please ensure your device is plugged in")]
    IoError { cause: io::Error },
    #[fail(display = "supplied secret isn't valid for this device")]
    WrongSecret,
    #[fail(display = "not an utf8 string")]
    StringEncodingError { cause: FromUtf8Error },
}

impl Fido2LuksError {
    pub fn exit_code(&self) -> i32 {
        use Fido2LuksError::*;
        match self {
            AskPassError { .. } | KeyfileError { .. } => 2,
            AuthenticatorError { .. } => 3,
            NoAuthenticatorError => 4,
            WrongSecret => 5,
            _ => 1,
        }
    }
}

#[derive(Debug, Fail)]
pub enum AskPassError {
    #[fail(display = "unable to retrieve password: {}", _0)]
    IO(io::Error),
    #[fail(display = "provided passwords don't match")]
    Mismatch,
}

use libcryptsetup_rs::LibcryptErr;
use std::string::FromUtf8Error;
use Fido2LuksError::*;

impl From<FidoError> for Fido2LuksError {
    fn from(e: FidoError) -> Self {
        AuthenticatorError { cause: e }
    }
}

impl From<LibcryptErr> for Fido2LuksError {
    fn from(e: LibcryptErr) -> Self {
        LuksError { cause: e }
    }
}
impl From<io::Error> for Fido2LuksError {
    fn from(e: io::Error) -> Self {
        IoError { cause: e }
    }
}

impl From<FromUtf8Error> for Fido2LuksError {
    fn from(e: FromUtf8Error) -> Self {
        StringEncodingError { cause: e }
    }
}
