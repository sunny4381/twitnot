use chrono::{DateTime, UTC};
use sqlite3::ResultRow;

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub screen_name: String,
    pub created_at: DateTime<UTC>,
}

#[derive(Debug)]
pub struct Tweet {
    pub id: i64,
    pub user_id: i32,
    pub user_name: String,
    pub created_at: DateTime<UTC>,
    pub text: String,
    pub retweets: i32,
    pub raw_json: String,
}

impl<'a, 'res, 'row> From<&'a ResultRow<'res, 'row>> for User {
    fn from(row: &'a ResultRow<'res, 'row>) -> User {
        User {
            id: row.column_int(0),
            screen_name: row.column_text(1).unwrap(),
            created_at: row.column_text(2).unwrap().parse::<DateTime<UTC>>().unwrap(),
        }
    }
}

impl<'a, 'res, 'row> From<&'a ResultRow<'res, 'row>> for Tweet {
    fn from(row: &'a ResultRow<'res, 'row>) -> Tweet {
        Tweet {
            id: row.column_int64(0),
            user_id: row.column_int(1),
            user_name: row.column_text(2).unwrap(),
            created_at: row.column_text(3).unwrap().parse::<DateTime<UTC>>().unwrap(),
            text: row.column_text(4).unwrap(),
            retweets: row.column_int(5),
            raw_json: row.column_text(6).unwrap(),
        }
    }
}
