use std::io::{self,BufRead,Write};

use clap::ArgMatches;

use crate::error::Error;
use crate::config::Config;

fn prompt(label: &str) -> Result<(), Error> {
    print!("put your {}: ", label);
    io::stdout().flush()?;
    return Ok(());
}

fn read_from_stdin(label: &str) -> Result<String, Error> {
    loop {
        prompt(label)?;

        let stdin = io::stdin();
        let input = stdin.lock().lines().next();
        if input.is_none() {
            continue;
        }
        let line = input.unwrap()?;
        if line.len() > 0 {
            return Ok(line);
        }
    }
}

pub fn execute_init(args: &ArgMatches) -> Result<(), Error> {
    let consumer_key = String::from(args.value_of("consumer_key").unwrap_or_else(|| panic!("specify consumer key")));
    let consumer_secret = match args.value_of("consumer_secret") {
        Some(consumer_secret) => String::from(consumer_secret),
        _ => read_from_stdin("Consumer Secret")?,
    };
    let database_file = match args.value_of("database_file") {
        Some(database_file) => String::from(database_file),
        _ => Config::default_database_file()?,
    };
    let notification_from_email = read_from_stdin("From Email Address")?;
    let notification_tos = read_from_stdin("To Email Addresses(comma separated)")?;
    let v2: Vec<String> = notification_tos.split(",").map(|item| item.trim()).map(String::from).collect();

    let config = Config {
        consumer_key: consumer_key,
        consumer_secret: consumer_secret,
        database_file: database_file,
        notification_from_email: notification_from_email,
        notification_tos: v2,
    };
    config.save("default")?;

    Ok(())
}
