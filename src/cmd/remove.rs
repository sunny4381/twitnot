use clap::ArgMatches;

use crate::db::Db;
use crate::error::Error;
use crate::config::Config;

pub fn execute_remove(args: &ArgMatches) -> Result<(), Error> {
    let config = Config::load("default")?;
    let db = Db::open(&config.database_file)?;

    let screen_name = args.value_of("screen_name").unwrap();
    let opt_user = db.get_user_by_screen_name(screen_name)?;
    if opt_user.is_none() {
        return Ok(());
    }

    let user = opt_user.unwrap();
    db.begin_transaction()?;
    db.delete_user(user.id)?;
    db.delete_tweets_by_user_id(user.id)?;
    db.commit()?;

    Ok(())
}
