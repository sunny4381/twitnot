use std::str;

use base64;

use chrono::{DateTime, Utc};

use lettre_email::EmailBuilder;
use lettre::EmailTransport;
use lettre::SmtpTransport;
use lettre::smtp::authentication::Credentials;

use super::Args;
use db::Db;
use db::models::{User, Tweet};
use error::Error;
use config::Config;
use twitter::TwitterClient;

fn encode_subject(subject: &str) -> String {
    let mut slices = subject.as_bytes().chunks(3 * 14 + 1);
    let mut ret = String::new();
    loop {
        match slices.next() {
            None => break,
            Some(ref slice) => {
                let b64 = base64::encode(slice);
                if ret.len() > 0 {
                    ret.push_str("\r\n ");
                }
                ret.push_str(&format!("=?UTF-8?B?{}?=", b64));
            },
        }
    }
    
    ret
}

fn send_notification_mail(config: &Config, user: &User, tweet: &Tweet) -> Result<(), Error> {
    let url = format!("http://twitter.com/{}/status/{}", user.screen_name, tweet.id);
    let subject = &format!("【更新通知】{}", tweet.user_name);
    let text = format!("{}\n\nURL: {}", tweet.text, url);
    let mut email_builder = EmailBuilder::new()
        .from(config.notification_from_email.as_str())
        .subject(encode_subject(subject))
        .text(text);
    for to in &config.notification_tos {
        email_builder = email_builder.to(to.as_str());
    }
    let email = try!(email_builder.build());

    // let transport_builder = try!(SmtpTransportBuilder::new(("smtp.gmail.com", 587), ClientSecurity::Opportunistic));
    let transport_builder = try!(SmtpTransport::simple_builder("smtp.gmail.com"));
    let mut transport = transport_builder
        .credentials(Credentials::new(config.gmail_username.clone(), config.gmail_password.clone()))
        .smtp_utf8(false)
        .build();
    try!(transport.send(&email));
    Ok(())
}

fn check_updates(config: &Config, db: &Db, user: &User) -> Result<(), Error> {
    let client = try!(TwitterClient::new());
    let access_token = try!(client.get_access_token(&config.consumer_key, &config.consumer_secret));
    let tweets = try!(client.get_tweets(&access_token, &user.screen_name, None));

    let mut insert_count = 0;
    let mut notify_count = 0;
    for tweet in tweets {
        let exists = {
            let tw = try!(db.get_tweet(tweet.id as i64));
            tw.is_some()
        };
        if exists {
            continue;
        }

        let tw = try!(db.insert_tweet(&Tweet {
            id: tweet.id as i64,
            user_id: user.id,
            user_name: tweet.user_name,
            created_at: try!(DateTime::parse_from_str(&tweet.created_at, "%a %b %e %T %z %Y")).with_timezone(&Utc),
            text: tweet.text,
            retweets: if tweet.retweets { 1 } else { 0 },
            raw_json: tweet.raw_json }));
        insert_count += 1;

        let exists2 = {
            let tw = try!(db.get_tweet(tweet.retweeted_status_id as i64));
            tw.is_some()
        };
        if exists2 {
            continue;
        }

        try!(send_notification_mail(config, &user, &tw));
        notify_count += 1;
    }

    println!("{}: imported {} tweets and send {} mails", user.screen_name, insert_count, notify_count);
    Ok(())
}

pub fn execute_check_updates(args: &Args) -> Result<(), Error> {
    let config = try!(Config::load("default"));
    let db = try!(Db::open(&config.database_file));

    if let Some(ref screen_name) = args.flag_screen_name {
        let opt_users = try!(db.get_user_by_screen_name(screen_name));
        if opt_users.is_some() {
            try!(check_updates(&config, &db, &opt_users.unwrap()));
        }
    } else {
        let users = try!(db.get_all_users());
        for user in users {
            try!(check_updates(&config, &db, &user));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_subject1() {
        assert_eq!(encode_subject("日本語テスト"), "=?UTF-8?B?5pel5pys6Kqe44OG44K544OI?=");
    }

    #[test]
    fn test_encode_subject2() {
        assert_eq!(encode_subject("徳島ヴォルティス 公式の更新通知"), "=?UTF-8?B?5b6z5bO244O044Kp44Or44OG44Kj44K5IOWFrOW8j+OBruabtOaWsOmAmg==?=\r\n =?UTF-8?B?55+l?=");
    }

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
            user_name: String::from("徳島ヴォルティス公式"),
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
