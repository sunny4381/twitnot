use std::io::Write;
use std::process::Command;

use chrono::{DateTime, Utc};
use clap::ArgMatches;
use tempfile::NamedTempFile;

use crate::db::Db;
use crate::db::models::{User, Tweet};
use crate::error::Error;
use crate::config::Config;
use crate::twitter::TwitterClient;

fn send_notification_mail(config: &Config, user: &User, tweet: &Tweet) -> Result<(), Error> {
    let url = format!("http://twitter.com/{}/status/{}", user.screen_name, tweet.id);
    let subject = format!("【更新通知】{}", tweet.user_name);
    let text = format!("{}\n\nURL: {}", tweet.text, url);

    let mut tmp_file = NamedTempFile::new()?;
    tmp_file.write_all(text.as_bytes())?;

    let tmp_file_path = tmp_file.path().as_os_str();

    // let to_addresses: String = config.notification_tos.join(",");
    for to in &config.notification_tos {
        let exit_status = Command::new("gmail").
            arg("send").
            arg(tmp_file_path).
            arg("--subject").arg(subject.as_str()).
            arg("--to").arg(to.as_str()).
            spawn()?.wait()?;
        if !exit_status.success() {
            return Err(Error::CommandError("gmail command execution error"));
        }
    }

    Ok(())
}

fn check_updates(config: &Config, db: &Db, user: &User) -> Result<(), Error> {
    let client = TwitterClient::new();
    let access_token = client.get_access_token(&config.consumer_key, &config.consumer_secret)?;
    let tweets = client.get_tweets(&access_token, &user.screen_name, None)?;

    let mut insert_count = 0;
    let mut notify_count = 0;
    for tweet in tweets {
        let exists = {
            let tw = db.get_tweet(tweet.id as i64)?;
            tw.is_some()
        };
        if exists {
            continue;
        }

        let tw = db.insert_tweet(&Tweet {
            id: tweet.id as i64,
            user_id: user.id,
            user_name: tweet.user_name,
            created_at: DateTime::parse_from_str(&tweet.created_at, "%a %b %e %T %z %Y")?.with_timezone(&Utc),
            text: tweet.text,
            retweets: if tweet.retweets { 1 } else { 0 },
            raw_json: tweet.raw_json })?;
        insert_count += 1;

        let exists2 = {
            let tw = db.get_tweet(tweet.retweeted_status_id as i64)?;
            tw.is_some()
        };
        if exists2 {
            continue;
        }

        send_notification_mail(config, &user, &tw)?;
        notify_count += 1;
    }

    println!("{}: imported {} tweets and send {} mails", user.screen_name, insert_count, notify_count);
    Ok(())
}

pub fn execute_check_updates(args: &ArgMatches) -> Result<(), Error> {
    let config = Config::load("default")?;
    let db = Db::open(&config.database_file)?;

    if let Some(screen_name) = args.value_of("screen_name") {
        let opt_users = db.get_user_by_screen_name(screen_name)?;
        if opt_users.is_some() {
            check_updates(&config, &db, &opt_users.unwrap())?;
        }
    } else {
        let users = db.get_all_users()?;
        for user in users {
            check_updates(&config, &db, &user)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_notification_mail() {
        let config = Config::load("test").unwrap();
        let user = User {
            id: 0,
            screen_name: String::from("vortis_pr"),
            created_at: Utc::now(),
        };
        let tweet = Tweet {
            id: 0,
            user_id: 0,
            user_name: String::from("ヴォルティススタジアム"),
            created_at: Utc::now(),
            text: String::from("テスト"),
            retweets: 0,
            raw_json: String::from(""),
        };
        let result = send_notification_mail(&config, &user, &tweet);
        println!("{:?}", result);
        assert_eq!(true, result.is_ok());
    }
}
