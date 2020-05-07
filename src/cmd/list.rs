use super::Args;
use crate::config::Config;
use crate::error::Error;
use crate::db::Db;

pub fn execute_list(args: &Args) -> Result<(), Error> {
    let config = Config::load("default")?;
    let db = Db::open(&config.database_file)?;

    if let Some(ref screen_name) = args.arg_screen_name {
        // list tweets
        if let Some(ref user) = db.get_user_by_screen_name(&screen_name)? {
            let tweets = db.get_tweets_by_user_id(user.id, args.flag_max.unwrap_or(20) as i32)?;
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
