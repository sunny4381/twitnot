use std::convert::TryFrom;
use chrono::Utc;
use rusqlite::{self, NO_PARAMS, Connection, params};

use crate::error::Error;

pub mod models;
mod query;

pub struct Db {
    conn: Connection,
}

impl Db {
    pub fn open(database_file: &str) -> Result<Db, Error> {
        let conn = Connection::open(database_file)?;

        conn.execute(query::ENABLE_FOREIGN_KEY, NO_PARAMS)?;
        conn.execute(query::CREATE_USERS_TABLE, NO_PARAMS)?;
        conn.execute(query::CREATE_TWEETS_TABLE, NO_PARAMS)?;
        conn.execute(query::CREATE_INDEX_USER_ID_ON_TWEETS, NO_PARAMS)?;
        conn.execute(query::CREATE_INDEX_CREATED_AT_ON_TWEETS, NO_PARAMS)?;

        Ok(Db { conn: conn })
    }

    pub fn begin_transaction(&self) -> Result<(), Error> {
        self.conn.execute(query::BEGIN_TRANSACTION, NO_PARAMS)?;
        Ok(())
    }

    pub fn commit(&self) -> Result<(), Error> {
        self.conn.execute(query::COMMIT_TRANSACTION, NO_PARAMS)?;
        Ok(())
    }

    pub fn get_all_users(&self) -> Result<Vec<models::User>, Error> {
        let mut stmt = self.conn.prepare(query::GET_ALL_USERS)?;
        let iter = stmt.query_map(NO_PARAMS, |row| Ok(models::User::try_from(row)?))?;

        let users: Result<Vec<models::User>, rusqlite::Error> = iter.collect();
        Ok(users?)
    }

    pub fn get_user_by_screen_name(&self, screen_name: &str) -> Result<Option<models::User>, Error> {
        let mut stmt = self.conn.prepare(query::GET_USER_BY_SCREEN_NAME)?;
        let mut iter = stmt.query_map(params![screen_name], |row| Ok(models::User::try_from(row)?))?;

        Ok(match iter.next() {
            Some(result) => Some(result?),
            _ => None
        })
    }

    pub fn get_user_by_row_id(&self, row_id: i64) -> Result<Option<models::User>, Error> {
        let mut stmt = self.conn.prepare(query::GET_USER_BY_ROW_ID)?;
        let mut iter = stmt.query_map(&[row_id], |row| Ok(models::User::try_from(row)?))?;

        Ok(match iter.next() {
            Some(result) => Some(result?),
            _ => None
        })
    }

    pub fn insert_user(&self, screen_name: &str) -> Result<models::User, Error> {
        let mut stmt = self.conn.prepare(query::INSERT_USER)?;
        let changes = stmt.execute(params![screen_name, &Utc::now()])?;
        if changes == 0 {
            return Err(Error::ModelError("insert user error"));
        }

        let row_id = self.conn.last_insert_rowid();
        let opt_user = self.get_user_by_row_id(row_id)?;
        opt_user.ok_or_else(|| Error::ModelError("insert user error"))
    }

    pub fn delete_user(&self, id: i32) -> Result<(), Error> {
        let mut stmt = self.conn.prepare(query::DELETE_USER)?;
        let changes = stmt.execute(params![id])?;
        if changes == 0 {
            return Err(Error::ModelError("delete user error"));
        }

        Ok(())
    }

    pub fn get_tweets_by_user_id(&self, user_id: i32, limit: i32) -> Result<Vec<models::Tweet>, Error> {
        let mut stmt = self.conn.prepare(query::GET_TWEETS_BY_USER_ID)?;
        let iter = stmt.query_map(params![user_id, limit], |row| {
            Ok(models::Tweet::try_from(row)?)
        })?;

        let tweets: Result<Vec<models::Tweet>, rusqlite::Error> = iter.collect();
        Ok(tweets?)
    }

    pub fn get_tweet(&self, id: i64) -> Result<Option<models::Tweet>, Error> {
        let mut stmt = self.conn.prepare(query::GET_TWEET)?;
        let mut iter = stmt.query_map(&[id], |row| {
            Ok(models::Tweet::try_from(row)?)
        })?;

        Ok(match iter.next() {
            Some(result) => Some(result?),
            _ => None
        })
    }

    pub fn get_tweet_by_row_id(&self, row_id: i64) -> Result<Option<models::Tweet>, Error> {
        let mut stmt = self.conn.prepare(query::GET_TWEET_BY_ROW_ID)?;
        let mut iter = stmt.query_map(&[row_id], |row| {
            Ok(models::Tweet::try_from(row)?)
        })?;

        Ok(match iter.next() {
            Some(result) => Some(result?),
            _ => None
        })
    }

    pub fn insert_tweet(&self, tweet: &models::Tweet) -> Result<models::Tweet, Error> {
        let mut stmt = self.conn.prepare(query::INSERT_TWEET)?;
        let changes = stmt.execute(params![
            tweet.id, tweet.user_id, tweet.user_name.as_str(), &tweet.created_at, tweet.text.as_str(), tweet.retweets, tweet.raw_json.as_str()
        ])?;
        if changes == 0 {
            return Err(Error::ModelError("insert tweet error"));
        }

        let row_id = self.conn.last_insert_rowid();
        let opt_tweet = self.get_tweet_by_row_id(row_id)?;
        opt_tweet.ok_or_else(|| Error::ModelError("insert tweet error"))
    }

    pub fn delete_tweets_by_user_id(&self, user_id: i32) -> Result<(), Error> {
        let mut stmt = self.conn.prepare(query::DELETE_TWEETS_BY_USER_ID)?;
        let changes = stmt.execute(params![user_id])?;
        if changes == 0 {
            return Err(Error::ModelError("delete error. no users are found"));
        }

        Ok(())
    }
}
