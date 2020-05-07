use clap::ArgMatches;

use crate::config::Config;
use crate::error::Error;
use crate::db::Db;

pub fn execute_list(args: &ArgMatches) -> Result<(), Error> {
    let config = Config::load("default")?;
    let db = Db::open(&config.database_file)?;

    if let Some(screen_name) = args.value_of("screen_name") {
        // list tweets
        if let Some(ref user) = db.get_user_by_screen_name(screen_name)? {
            let max_count: i32 = args.value_of("max_count").map(|s| s.parse().unwrap_or(20)).unwrap_or(20);
            let tweets = db.get_tweets_by_user_id(user.id, max_count)?;
            for tweet in tweets {
                println!("{}\t{}\t{}", tweet.id, tweet.created_at, tweet.text);
            }
        }
    } else {
        // list users
        let users: Vec<_> = db.get_all_users()?;
        for user in users {
            println!("{}", user.screen_name);
        }
    }

    Ok(())
}
