extern crate base64;
extern crate chrono;
extern crate docopt;
extern crate dotenv;
extern crate encoding;
extern crate env_logger;
extern crate lettre;
extern crate lettre_email;
extern crate mime;
extern crate reqwest;
extern crate rustc_serialize;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate sqlite3;
extern crate time;

mod cmd;
mod config;
mod db;
mod error;
mod twitter;

use std::io::{self, Write};

use docopt::Docopt;
use dotenv::dotenv;

use cmd::execute;
use error::Error;

const USAGE: &'static str = r#"
Tweet Monitor & Notification.

Usage:
  twitnot init <consumer-key> [--secret=<consumer-secret>] [--db=<database-file>] [--gmail-username=<gmail-username>] [--gmail-password=<gmail-password>]
  twitnot add <screen-name>
  twitnot list [<screen-name>] [--max=<max-count>]
  twitnot remove <screen-name>
  twitnot check-updates [--screen-name=<screen-name>]
  twitnot (-h | --help)
Options:
  -h, --help     Show this screen.
  --secret=<consumer-secret> Specify consucmer secret.
  --screen-name=<screen-name> Specify screen name.
  --db=<databaase-file> Specify database file.
  --max=<max-count> Specify max count of tweet [default is 10].
"#;

#[derive(Debug, Deserialize)]
pub struct Args {
    flag_secret: Option<String>,
    flag_gmail_username: Option<String>,
    flag_gmail_password: Option<String>,
    flag_screen_name: Option<String>,
    flag_db: Option<String>,
    flag_max: Option<u32>,
    arg_consumer_key: Option<String>,
    arg_screen_name: Option<String>,
    cmd_init: bool,
    cmd_list: bool,
    cmd_add: bool,
    cmd_remove: bool,
    cmd_check_updates: bool,
}

fn main() {
    env_logger::init();
    dotenv().ok();

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    match execute(&args) {
        Ok(_) => (),
        Err(ref e) => abort(e),
    };
}

pub fn abort(e: &Error) {
    writeln!(&mut io::stderr(), "{}", e).unwrap();
    ::std::process::exit(1)
}
