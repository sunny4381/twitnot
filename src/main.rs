mod cmd;
mod config;
mod db;
mod error;
mod twitter;

use std::io::{self, Write};

use clap::clap_app;
use dotenv::dotenv;

use crate::cmd::execute;
use crate::error::Error;

fn main() {
    dotenv().ok();
    env_logger::init();

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
