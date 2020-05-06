use std::env;
// use std::error;
use std::io;
use std::io::Read;
use std::fmt;

use chrono;
use lettre;
use lettre_email;
use reqwest;
use serde_json;
use sqlite3;

#[derive(Debug)]
pub enum Error {
    EnvError(env::VarError),
    IoError(io::Error),
    ReqwestError(reqwest::Error),
    HttpError(reqwest::StatusCode, String),
    SerdeError(serde_json::error::Error),
    ConfigError(&'static str),
    SqliteError(sqlite3::SqliteError),
    ChronoParseError(chrono::ParseError),
    LettreEmailError(lettre_email::error::Error),
    LettreSmtpError(lettre::smtp::error::Error),
    UnknownCommandError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::EnvError(ref err) => write!(f, "IO error: {}", err),
            Error::IoError(ref err) => write!(f, "IO error: {}", err),
            // Error::HyperError(ref err) => write!(f, "Hyper error: {}", err),
            Error::ReqwestError(ref err) => write!(f, "Reqwest error: {}", err),
            Error::HttpError(ref status, ref msg) => write!(f, "HTTP error: {}\n{}", status, msg),
            Error::SerdeError(ref err) => write!(f, "Serde error: {}", err),
            // Error::NativeTlsError(ref err) => write!(f, "NativeTls error: {}", err),
            Error::ConfigError(msg) => write!(f, "Config error: {}", msg),
            Error::SqliteError(ref err) => write!(f, "Sqlite error: {}", err),
            Error::ChronoParseError(ref err) => write!(f, "Chrono Parse error: {}", err),
            Error::LettreEmailError(ref err) => write!(f, "Lettre Email error: {}", err),
            Error::LettreSmtpError(ref err) => write!(f, "Lettre Smtp error: {}", err),
            Error::UnknownCommandError => write!(f, "Unknown Command"),
        }
    }
}

// impl error::Error for Error {
//     fn description(&self) -> &str {
//         // 下層のエラーは両方ともすでに `Error` を実装しているので、
//         // それらの実装に従います。
//         match *self {
//             Error::EnvError(ref err) => err.description(),
//             Error::IoError(ref err) => err.description(),
//             // Error::HyperError(ref err) => err.description(),
//             Error::ReqwestError(ref err) => err.description(),
//             Error::HttpError(ref status, ref _msg) => status.canonical_reason().unwrap(),
//             Error::SerdeError(ref err) => err.description(),
//             // Error::NativeTlsError(ref err) => err.description(),
//             Error::ConfigError(msg) => msg,
//             Error::SqliteError(ref err) => err.description(),
//             Error::ChronoParseError(ref err) => err.description(),
//             Error::LettreEmailError(ref err) => err.description(),
//             Error::LettreSmtpError(ref err) => err.description(),
//             Error::UnknownCommandError => "unknown command",
//         }
//     }

//     fn cause(&self) -> Option<&error::Error> {
//         match *self {
//             // 注意：これらは両方とも `err` を、その具象型（`&io::Error` か
//             // `&num::ParseIntError` のいずれか）から、トレイトオブジェクト
//             // `&Error` へ暗黙的にキャストします。どちらのエラー型も `Error` を
//             // 実装しているので、問題なく動きます。
//             Error::EnvError(ref err) => Some(err),
//             Error::IoError(ref err) => Some(err),
//             // Error::HyperError(ref err) => Some(err),
//             Error::ReqwestError(ref err) => Some(err),
//             Error::HttpError(_, _) => None,
//             Error::SerdeError(ref err) => Some(err),
//             // Error::NativeTlsError(ref err) => Some(err),
//             Error::ConfigError(_) => None,
//             Error::SqliteError(ref err) => Some(err),
//             Error::ChronoParseError(ref err) => Some(err),
//             Error::LettreEmailError(ref err) => Some(err),
//             Error::LettreSmtpError(ref err) => Some(err),
//             Error::UnknownCommandError => None,
//         }
//     }
// }

impl From<env::VarError> for Error {
    fn from(err: env::VarError) -> Error {
        Error::EnvError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::ReqwestError(err)
    }
}

impl From<reqwest::blocking::Response> for Error {
    fn from(mut res: reqwest::blocking::Response) -> Error {
        let mut body = String::new();
        let result = res.read_to_string(&mut body);
        if result.is_ok() {
            Error::HttpError(res.status(), body)
        } else {
            Error::from(result.unwrap_err())
        }
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Error {
        Error::SerdeError(err)
    }
}

impl From<sqlite3::SqliteError> for Error {
    fn from(err: sqlite3::SqliteError) -> Error {
        Error::SqliteError(err)
    }
}

impl From<chrono::ParseError> for Error {
    fn from(err: chrono::ParseError) -> Error {
        Error::ChronoParseError(err)
    }
}

impl From<lettre_email::error::Error> for Error {
    fn from(err: lettre_email::error::Error) -> Error {
        Error::LettreEmailError(err)
    }
}

impl From<lettre::smtp::error::Error> for Error {
    fn from(err: lettre::smtp::error::Error) -> Error {
        Error::LettreSmtpError(err)
    }
}
