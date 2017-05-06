use std::io::{self,BufRead,Write};

use super::Args;
use error::Error;
use config::Config;

fn prompt(label: &str) -> Result<(), Error> {
    print!("put your {}: ", label);
    try!(io::stdout().flush());
    return Ok(());
}

fn read_from_stdin(label: &str) -> Result<String, Error> {
    loop {
        try!(prompt(label));

        let stdin = io::stdin();
        let input = stdin.lock().lines().next();
        if input.is_none() {
            continue;
        }
        let line = try!(input.unwrap());
        if line.len() > 0 {
            return Ok(line);
        }
    }
}

pub fn execute_init(args: &Args) -> Result<(), Error> {
    let consumer_key = try!(match args.arg_consumer_key {
        Some(ref consumer_key) => Ok(consumer_key.clone()),
        _ => read_from_stdin("Consumer Key"),
    });
    let consumer_secret = try!(match args.flag_secret {
        Some(ref consumer_secret) => Ok(consumer_secret.clone()),
        _ => read_from_stdin("Consumer Secret"),
    });
    let database_file = try!(match args.flag_db {
        Some(ref db) => Ok(db.clone()),
        _ => Config::default_database_file(),
    });
    let gmail_username = try!(match args.flag_gmail_username {
        Some(ref gmail_username) => Ok(gmail_username.clone()),
        _ => read_from_stdin("Gmail Username"),
    });
    let gmail_password = try!(match args.flag_gmail_password {
        Some(ref gmail_password) => Ok(gmail_password.clone()),
        _ => read_from_stdin("Gmail Password"),
    });
    let notification_from_email = try!(read_from_stdin("From Email Address"));
    let notification_tos = try!(read_from_stdin("To Email Addresses(comma separated)"));
    let v2: Vec<String> = notification_tos.split(",").map(|item| item.trim()).map(String::from).collect();

    let config = Config {
        consumer_key: consumer_key,
        consumer_secret: consumer_secret,
        database_file: database_file,
        gmail_username: gmail_username,
        gmail_password: gmail_password,
        notification_from_email: notification_from_email,
        notification_tos: v2,
    };
    try!(config.save("default"));

    Ok(())
}
