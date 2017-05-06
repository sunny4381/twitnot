use std::io::Read;
use std::vec::Vec;

use base64;
use hyper::Client;
use hyper::header::{ContentType, UserAgent, Authorization};
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use serde_json;
use url::form_urlencoded;

use error::Error;

pub const USER_AGENT: &'static str = "twitnot/0.1";

#[derive(Debug)]
pub struct TwitterClient {
    pub client: Client,
}

#[derive(Debug)]
pub struct Tweet {
    pub id: u64,
    pub created_at: String,
    pub user_name: String,
    pub text: String,
    pub retweets: bool,
    pub raw_json: String,
}

impl TwitterClient {
    pub fn new() -> Result<TwitterClient, Error> {
        let ssl = try!(NativeTlsClient::new().map_err(Error::NativeTlsError));

        Ok(TwitterClient {
            client: Client::with_connector(HttpsConnector::new(ssl)),
        })
    }

    pub fn get_access_token(&self, consumer_key: &str, consumer_secret: &str) -> Result<String, Error> {
        let token_body: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("grant_type", "client_credentials")
            .finish();

        let authorization = base64::encode(format!("{}:{}", consumer_key, consumer_secret).as_bytes());

        let req = self.client.post("https://api.twitter.com/oauth2/token")
            .body(&token_body)
            .header(ContentType("application/x-www-form-urlencoded".parse().unwrap()))
            .header(Authorization(format!("Basic {}", authorization)))
            .header(UserAgent(USER_AGENT.to_owned()));
    
        let res = try!(req.send());
        if !res.status.is_success() {
            return Err(Error::from(res))
        }

        let body: serde_json::Value = try!(serde_json::from_reader(res));
        let access_token = String::from(body["access_token"].as_str().unwrap());
        Ok(access_token)
    }

    pub fn get_tweets(&self, access_token: &str, screen_name: &str, count: Option<u32>) -> Result<Vec<Tweet>, Error> {
        let mut params = form_urlencoded::Serializer::new(String::new());
        params.append_pair("screen_name", screen_name);
        if let Some(c) = count {
            params.append_pair("count", &format!("{}", c));
        }
        let url = format!("https://api.twitter.com/1.1/statuses/user_timeline.json?{}", params.finish());
        let req = self.client.get(&url)
            .header(Authorization(format!("Bearer {}", access_token)))
            .header(UserAgent(USER_AGENT.to_owned()));
        let mut res = try!(req.send());
        if !res.status.is_success() {
            return Err(Error::from(res))
        }

        let mut body = String::new();
        try!(res.read_to_string(&mut body));

        let mut results: Vec<Tweet> = Vec::new();
        let timeline_json: serde_json::Value = try!(serde_json::from_str(&body));
        match timeline_json.as_array() {
            Some(entries) => {
                for entry in entries {
                    let id = entry["id"].as_u64().unwrap_or(0);
                    let created_at = entry["created_at"].as_str().unwrap_or("");
                    let user_name = entry["user"]["name"].as_str().unwrap_or("");
                    let text = entry["text"].as_str().unwrap_or("");
                    let retweets = entry["retweeted_status"].is_null();

                    results.push(Tweet {
                        id: id,
                        created_at: String::from(created_at),
                        user_name: String::from(user_name),
                        text: String::from(text),
                        retweets: retweets,
                        raw_json: entry.to_string(),
                    });
                }
            },
            _ => {},
        };

        Ok(results)
    }
}
