use ::Args;
use config::Config;
use error::Error;
use db::Db;

pub fn execute_list(args: &Args) -> Result<(), Error> {
    let config = try!(Config::load("default"));
    let db = try!(Db::open(&config.database_file));
    
    if let Some(ref screen_name) = args.arg_screen_name {
        // list tweets
        if let Some(ref user) = try!(db.get_user_by_screen_name(&screen_name)) {
            let tweets = try!(db.get_tweets_by_user_id(user.id, args.flag_max.unwrap_or(20) as i32));
            for tweet in tweets {
                println!("{}\t{}\t{}", tweet.id, tweet.created_at, tweet.text);
            }
        }
    } else {
        // list users
        let users: Vec<_> = try!(db.get_all_users());
        for user in users {
            println!("{}", user.screen_name);
        }
    }

    Ok(())
}
