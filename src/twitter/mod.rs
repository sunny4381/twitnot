use std::io::Read;
use std::vec::Vec;

use base64;
use reqwest;
use serde_json;

use error::Error;

pub const USER_AGENT: &'static str = "twitnot/0.1";
pub const TOKEN_URL: &'static str = "https://api.twitter.com/oauth2/token";

#[derive(Debug)]
pub struct TwitterClient {
    pub client: reqwest::blocking::Client,
}

#[derive(Debug)]
pub struct Tweet {
    pub id: u64,
    pub created_at: String,
    pub user_name: String,
    pub text: String,
    pub retweets: bool,
    pub retweeted_status_id: u64,
    pub raw_json: String,
}

impl TwitterClient {
    pub fn new() -> Result<TwitterClient, Error> {
        Ok(TwitterClient {
            client: reqwest::blocking::Client::new(),
        })
    }

    pub fn get_access_token(&self, consumer_key: &str, consumer_secret: &str) -> Result<String, Error> {
        let authorization = base64::encode(format!("{}:{}", consumer_key, consumer_secret).as_bytes());

        let builder = self.client.post(TOKEN_URL);
        let req = builder.form(&[("grant_type", "client_credentials")])
            .bearer_auth(authorization)
            .header(reqwest::header::USER_AGENT, USER_AGENT);
        let res = req.send()?;
        if !res.status().is_success() {
            return Err(Error::from(res))
        }

        let body: serde_json::Value = serde_json::from_reader(res)?;
        let access_token = String::from(body["access_token"].as_str().unwrap());
        Ok(access_token)
    }

    pub fn get_tweets(&self, access_token: &str, screen_name: &str, count: Option<u32>) -> Result<Vec<Tweet>, Error> {
        let mut builder = self.client.get("https://api.twitter.com/1.1/statuses/user_timeline.json");
        builder = builder.query(&[("screen_name", screen_name)]);
        if let Some(c) = count {
            builder = builder.query(&[("count", &format!("{}", c))]);
        }

        let req = builder.bearer_auth(access_token)
            .header(reqwest::header::USER_AGENT, USER_AGENT);
        let mut res = req.send()?;
        if !res.status().is_success() {
            return Err(Error::from(res))
        }

        let mut body = String::new();
        res.read_to_string(&mut body)?;

        let mut results: Vec<Tweet> = Vec::new();
        let timeline_json: serde_json::Value = serde_json::from_str(&body)?;
        match timeline_json.as_array() {
            Some(entries) => {
                for entry in entries {
                    let id = entry["id"].as_u64().unwrap_or(0);
                    let created_at = entry["created_at"].as_str().unwrap_or("");
                    let user_name = entry["user"]["name"].as_str().unwrap_or("");
                    let text = entry["text"].as_str().unwrap_or("");
                    let retweets = entry["retweeted_status"].is_null();
                    let retweeted_status_id = entry["retweeted_status"]["id"].as_u64().unwrap_or(0);

                    results.push(Tweet {
                        id: id,
                        created_at: String::from(created_at),
                        user_name: String::from(user_name),
                        text: String::from(text),
                        retweets: retweets,
                        retweeted_status_id: retweeted_status_id,
                        raw_json: entry.to_string(),
                    });
                }
            },
            _ => {},
        };

        Ok(results)
    }
}
