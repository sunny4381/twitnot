pub const ENABLE_FOREIGN_KEY: &'static str = r#"
PRAGMA foreign_keys = ON;
"#;

pub const CREATE_USERS_TABLE: &'static str = r#"
CREATE TABLE IF NOT EXISTS users (
        id          INTEGER PRIMARY KEY AUTOINCREMENT,
        screen_name VARCHAR NOT NULL UNIQUE,
        created_at  DATETIME NOT NULL
);
"#;

pub const CREATE_TWEETS_TABLE: &'static str = r#"
CREATE TABLE IF NOT EXISTS tweets (
        id          INTEGER PRIMARY KEY,
        user_id     INTEGER NOT NULL,
        user_name   TEXT,
        created_at  DATETIME NOT NULL,
        text        TEXT,
        retweets    INTEGER,
        raw_json    TEXT
);
"#;

pub const CREATE_INDEX_USER_ID_ON_TWEETS: &'static str = r#"
CREATE INDEX IF NOT EXISTS index_user_id ON tweets (
    user_id
)
"#;

pub const CREATE_INDEX_CREATED_AT_ON_TWEETS: &'static str = r#"
CREATE INDEX IF NOT EXISTS index_created_at ON tweets (
    created_at
)
"#;

pub const GET_ALL_USERS: &'static str = r#"
SELECT * FROM users
"#;

pub const GET_USER_BY_SCREEN_NAME: &'static str = r#"
SELECT * FROM users WHERE screen_name=$1
"#;

pub const GET_USER_BY_ROW_ID: &'static str = r#"
SELECT * FROM users WHERE ROWID=$1
"#;

pub const INSERT_USER: &'static str = r#"
INSERT INTO users(screen_name, created_at) VALUES ($1, $2)
"#;

pub const GET_TWEETS_BY_USER_ID: &'static str = r#"
SELECT * FROM tweets WHERE user_id=$1 ORDER BY created_at desc LIMIT $2
"#;

pub const GET_TWEET: &'static str = r#"
SELECT * FROM tweets WHERE id=$1
"#;

pub const GET_TWEET_BY_ROW_ID: &'static str = r#"
SELECT * FROM tweets WHERE ROWID=$1
"#;

pub const INSERT_TWEET: &'static str = r#"
INSERT INTO tweets(id,user_id,user_name,created_at,text,retweets,raw_json) VALUES ($1,$2,$3,$4,$5,$6,$7)
"#;
