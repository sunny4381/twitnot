mod cmd;
mod config;
mod db;
mod error;
mod twitter;

use std::io::{self, Write};

use clap::clap_app;
use dotenv::dotenv;
// use serde_derive::Deserialize;

use crate::cmd::execute;
use crate::error::Error;

// const USAGE: &'static str = r#"
// Tweet Monitor & Notification.

// Usage:
//   twitnot init <consumer-key> [--secret=<consumer-secret>] [--db=<database-file>] [--gmail-username=<gmail-username>] [--gmail-password=<gmail-password>]
//   twitnot add <screen-name>
//   twitnot list [<screen-name>] [--max=<max-count>]
//   twitnot remove <screen-name>
//   twitnot check-updates [--screen-name=<screen-name>]
//   twitnot (-h | --help)
// Options:
//   -h, --help     Show this screen.
//   --secret=<consumer-secret> Specify consucmer secret.
//   --screen-name=<screen-name> Specify screen name.
//   --db=<databaase-file> Specify database file.
//   --max=<max-count> Specify max count of tweet [default is 10].
// "#;

// #[derive(Debug, Deserialize)]
// pub struct Args {
//     flag_secret: Option<String>,
//     flag_gmail_username: Option<String>,
//     flag_gmail_password: Option<String>,
//     flag_screen_name: Option<String>,
//     flag_db: Option<String>,
//     flag_max: Option<u32>,
//     arg_consumer_key: Option<String>,
//     arg_screen_name: Option<String>,
//     cmd_init: bool,
//     cmd_list: bool,
//     cmd_add: bool,
//     cmd_remove: bool,
//     cmd_check_updates: bool,
// }

fn main() {
    env_logger::init();
    dotenv().ok();

    let args = clap_app!(twitnot =>
        (version: "0.1.0")
        (author: "NAKANO Hideo <pinarello.marvel@gmail.com>")
        (about: "Twitter timeline update notification")
        (@subcommand init =>
            (about: "initialize environment")
            (@arg consumer_key: +required "consumer key")
            (@arg consumer_secret: --secret +takes_value "consumer secret")
            (@arg database_file: --db +takes_value "database file")
            (@arg gmail_username: --gmail_username +takes_value "gmail username")
            (@arg gmail_password: --gmail_password +takes_value "gmail password")
        )
        (@subcommand add =>
            (about: "add screen name to watch updates")
            (@arg screen_name: +required "screen name")
        )
        (@subcommand list =>
            (about: "list tweets collected currently")
            (@arg screen_name: "screen name")
            (@arg max_count: --max +takes_value "max count of tweet [default is 10]")
        )
        (@subcommand remove =>
            (about: "remove screen name watching updates")
            (@arg screen_name: +required "screen name")
        )
        (@subcommand check_update =>
            (about: "remove screen name watching updates")
            (@arg screen_name: "screen name")
        )
    ).get_matches();

    match execute(&args) {
        Ok(_) => (),
        Err(ref e) => abort(e),
    };
}

pub fn abort(e: &Error) {
    writeln!(&mut io::stderr(), "{}", e).unwrap();
    ::std::process::exit(1)
}
