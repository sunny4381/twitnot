use chrono::{DateTime, Utc};

use super::Args;
use crate::db::Db;
use crate::db::models::{self, User};
use crate::error::Error;
use crate::config::Config;
use crate::twitter::TwitterClient;

fn retrieve_user(db: &Db, screen_name: &str) -> Result<User, Error> {
    let user1 = db.get_user_by_screen_name(screen_name);
    if user1.is_ok() {
        if let Some(user) = user1.unwrap() {
            return Ok(user);
        }
    }

    db.insert_user(screen_name)
}

pub fn execute_add(args: &Args) -> Result<(), Error> {
    let config = Config::load("default")?;

    let db = Db::open(&config.database_file)?;
    let client = TwitterClient::new()?;

    let screen_name = args.arg_screen_name.clone().unwrap();
    let user = retrieve_user(&db, &screen_name)?;

    let access_token = client.get_access_token(&config.consumer_key, &config.consumer_secret)?;
    let tweets = client.get_tweets(&access_token, &screen_name, None)?;

    let mut insert_count = 0;
    for tweet in tweets {
        let exists = {
            let tw = db.get_tweet(tweet.id as i64)?;
            tw.is_some()
        };
        if exists {
            continue;
        }

        db.insert_tweet(&models::Tweet {
            id: tweet.id as i64,
            user_id: user.id,
            user_name: tweet.user_name,
            created_at: DateTime::parse_from_str(&tweet.created_at, "%a %b %e %T %z %Y")?.with_timezone(&Utc),
            text: tweet.text,
            retweets: if tweet.retweets { 1 } else { 0 },
            raw_json: tweet.raw_json })?;
        insert_count += 1;
    }

    println!("imported {} tweets", insert_count);
    Ok(())
}
