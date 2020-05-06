use super::Args;
use db::Db;
use error::Error;
use config::Config;

pub fn execute_remove(args: &Args) -> Result<(), Error> {
    let config = Config::load("default")?;
    let db = Db::open(&config.database_file)?;

    let screen_name = args.arg_screen_name.clone().unwrap();
    let opt_user = db.get_user_by_screen_name(&screen_name)?;
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
