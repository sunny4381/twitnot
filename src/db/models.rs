use std::convert::TryFrom;

use chrono::{DateTime, Utc};
use rusqlite::Row;

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub screen_name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct Tweet {
    pub id: i64,
    pub user_id: i32,
    pub user_name: String,
    pub created_at: DateTime<Utc>,
    pub text: String,
    pub retweets: i32,
    pub raw_json: String,
}

impl<'a, 'stmt> TryFrom<&'a Row<'stmt>> for User {
    type Error = rusqlite::Error;

    fn try_from(row: &'a Row<'stmt>) -> Result<Self, Self::Error> {
        Ok(User {
            id: row.get(0)?,
            screen_name: row.get(1)?,
            created_at: row.get(2)?,
        })
    }
}

impl<'a> TryFrom<&'a Row<'_>> for Tweet {
    type Error = rusqlite::Error;

    fn try_from(row: &'a Row<'_>) -> Result<Self, Self::Error> {
        Ok(Tweet {
            id: row.get(0)?,
            user_id: row.get(1)?,
            user_name: row.get(2)?,
            created_at: row.get(3)?,
            text: row.get(4)?,
            retweets: row.get(5)?,
            raw_json: row.get(6)?,
        })
    }
}
