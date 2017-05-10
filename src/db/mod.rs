use chrono::UTC;

use error::Error;

use sqlite3::DatabaseConnection;
use sqlite3::access;
use sqlite3::StatementUpdate;
use sqlite3::SqliteError;
use sqlite3::SqliteErrorCode;

pub mod models;
mod query;

pub struct Db {
    conn: DatabaseConnection,
}

fn mk_err1(desc: &'static str) -> Error {
    mk_err2(SqliteErrorCode::SQLITE_ERROR, desc)
}

fn mk_err2(kind: SqliteErrorCode, desc: &'static str) -> Error {
    Error::from(SqliteError {
        kind: kind,
        desc: desc,
        detail: None,
    })
}

// fn mk_err3(kind: SqliteErrorCode, desc: &'static str, detail: Option<String>) -> Error {
//     Error::from(SqliteError {
//         kind: kind,
//         desc: desc,
//         detail: detail,
//     })
// }

impl Db {
    pub fn open(database_file: &str) -> Result<Db, Error> {
        let access = access::ByFilename { flags: Default::default(), filename: database_file };
        let mut conn = try!(DatabaseConnection::new(access));

        try!(conn.exec(query::ENABLE_FOREIGN_KEY));
        try!(conn.exec(query::CREATE_USERS_TABLE));
        try!(conn.exec(query::CREATE_TWEETS_TABLE));
        try!(conn.exec(query::CREATE_INDEX_USER_ID_ON_TWEETS));
        try!(conn.exec(query::CREATE_INDEX_CREATED_AT_ON_TWEETS));
        Ok(Db { conn: conn })
    }

    pub fn begin_transaction(&self) -> Result<(), Error> {
        let mut stmt = try!(self.conn.prepare(query::BEGIN_TRANSACTION));
        let mut results = stmt.execute();
        loop {
            match try!(results.step()) {
                None => break,
                Some(ref row) => (),
            }
        }
        Ok(())
    }

    pub fn commit(&self) -> Result<(), Error> {
        let mut stmt = try!(self.conn.prepare(query::COMMIT_TRANSACTION));
        let mut results = stmt.execute();
        loop {
            match try!(results.step()) {
                None => break,
                Some(ref row) => (),
            }
        }
        Ok(())
    }

    pub fn get_all_users(&self) -> Result<Vec<models::User>, Error> {
        let mut stmt = try!(self.conn.prepare(query::GET_ALL_USERS));
        let mut users = vec!();
        let mut results = stmt.execute();
        loop {
            match try!(results.step()) {
                None => break,
                Some(ref row) => users.push(models::User::from(row)),
            }
        }

        Ok(users)
    }

    pub fn get_user_by_screen_name(&self, screen_name: &str) -> Result<Option<models::User>, Error> {
        let mut stmt = try!(self.conn.prepare(query::GET_USER_BY_SCREEN_NAME));
        try!(stmt.bind_text(1, screen_name));
        let mut users = vec!();
        let mut results = stmt.execute();
        loop {
            match try!(results.step()) {
                None => break,
                Some(ref row) => {
                    users.push(models::User::from(row));
                    break;
                },
            }
        }

        Ok(users.pop())
    }

    pub fn get_user_by_row_id(&self, row_id: i64) -> Result<Option<models::User>, Error> {
        let mut stmt = try!(self.conn.prepare(query::GET_USER_BY_ROW_ID));
        try!(stmt.bind_int64(1, row_id));
        let mut users = vec!();
        let mut results = stmt.execute();
        loop {
            match try!(results.step()) {
                None => break,
                Some(ref row) => {
                    users.push(models::User::from(row));
                    break;
                }
            }
        }

        Ok(users.pop())
    }

    pub fn insert_user(&self, screen_name: &str) -> Result<models::User, Error> {
        let mut stmt = try!(self.conn.prepare(query::INSERT_USER));
        try!(stmt.bind_text(1, screen_name));
        try!(stmt.bind_text(2, &UTC::now().to_rfc3339()));
        let changes = try!(stmt.update(&[]));
        if changes == 0 {
            return Err(mk_err2(SqliteErrorCode::SQLITE_NOTFOUND, "insert error"));
        }

        let row_id = self.conn.last_insert_rowid();
        let opt_user = try!(self.get_user_by_row_id(row_id));
        opt_user.ok_or_else(|| mk_err2(SqliteErrorCode::SQLITE_NOTFOUND, "insert error"))
    }

    pub fn delete_user(&self, id: i32) -> Result<(), Error> {
        let mut stmt = try!(self.conn.prepare(query::DELETE_USER));
        try!(stmt.bind_int(1, id));
        let changes = try!(stmt.update(&[]));
        if changes == 0 {
            return Err(mk_err1("delete error"));
        }

        Ok(())
    }

    pub fn get_tweets_by_user_id(&self, user_id: i32, limit: i32) -> Result<Vec<models::Tweet>, Error> {
        let mut stmt = try!(self.conn.prepare(query::GET_TWEETS_BY_USER_ID));
        try!(stmt.bind_int(1, user_id));
        try!(stmt.bind_int(2, limit));
        let mut tweets = vec!();
        let mut results = stmt.execute();
        loop {
            match try!(results.step()) {
                None => break,
                Some(ref row) => tweets.push(models::Tweet::from(row)),
            }
        }

        Ok(tweets)
    }

    pub fn get_tweet(&self, id: i64) -> Result<Option<models::Tweet>, Error> {
        let mut stmt = try!(self.conn.prepare(query::GET_TWEET));
        try!(stmt.bind_int64(1, id));
        let mut tweets = vec!();
        let mut results = stmt.execute();
        loop {
            match try!(results.step()) {
                None => break,
                Some(ref row) => {
                    tweets.push(models::Tweet::from(row));
                    break;
                },
            }
        }

        Ok(tweets.pop())
    }

    pub fn get_tweet_by_row_id(&self, row_id: i64) -> Result<Option<models::Tweet>, Error> {
        let mut stmt = try!(self.conn.prepare(query::GET_TWEET_BY_ROW_ID));
        try!(stmt.bind_int64(1, row_id));
        let mut tweets = vec!();
        let mut results = stmt.execute();
        loop {
            match try!(results.step()) {
                None => break,
                Some(ref row) => {
                    tweets.push(models::Tweet::from(row));
                    break;
                },
            }
        }

        Ok(tweets.pop())
    }

    pub fn insert_tweet(&self, tweet: &models::Tweet) -> Result<models::Tweet, Error> {
        let mut stmt = try!(self.conn.prepare(query::INSERT_TWEET));
        try!(stmt.bind_int64(1, tweet.id));
        try!(stmt.bind_int(2, tweet.user_id));
        try!(stmt.bind_text(3, &tweet.user_name));
        try!(stmt.bind_text(4, &tweet.created_at.to_rfc3339()));
        try!(stmt.bind_text(5, &tweet.text));
        try!(stmt.bind_int(6, tweet.retweets));
        try!(stmt.bind_text(7, &tweet.raw_json));
        let changes = try!(stmt.update(&[]));
        if changes == 0 {
            return Err(mk_err2(SqliteErrorCode::SQLITE_NOTFOUND, "insert error"));
        }

        let row_id = self.conn.last_insert_rowid();
        let opt_tweet = try!(self.get_tweet_by_row_id(row_id));
        opt_tweet.ok_or_else(|| mk_err2(SqliteErrorCode::SQLITE_NOTFOUND, "insert error"))
    }

    pub fn delete_tweets_by_user_id(&self, user_id: i32) -> Result<(), Error> {
        let mut stmt = try!(self.conn.prepare(query::DELETE_TWEETS_BY_USER_ID));
        try!(stmt.bind_int(1, user_id));
        let changes = try!(stmt.update(&[]));
        if changes == 0 {
            return Err(mk_err1("delete error"));
        }

        Ok(())
    }
}
