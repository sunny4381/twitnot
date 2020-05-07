mod add;
mod init;
mod list;
mod check_updates;
mod remove;

use clap::ArgMatches;

use self::add::execute_add;
use self::init::execute_init;
use self::list::execute_list;
use self::check_updates::execute_check_updates;
use self::remove::execute_remove;
use crate::error::Error;

pub fn execute(args: &ArgMatches) -> Result<(), Error> {
    if let Some(args) = args.subcommand_matches("init") {
        return execute_init(args);
    } else if let Some(args) = args.subcommand_matches("list") {
        return execute_list(args);
    } else if let Some(args) = args.subcommand_matches("add") {
        return execute_add(args);
    } else if let Some(args) = args.subcommand_matches("remove") {
        return execute_remove(args);
    } else if let Some(args) = args.subcommand_matches("check_update") {
        return execute_check_updates(args);
    }

    return Err(Error::UnknownCommandError);
}
