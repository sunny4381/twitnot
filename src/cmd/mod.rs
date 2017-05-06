mod import;
mod init;
mod list;
mod check_updates;

use super::Args;
use self::import::execute_import;
use self::init::execute_init;
use self::list::execute_list;
use self::check_updates::execute_check_updates;
use error::Error;

pub fn execute(args: &Args) -> Result<(), Error> {
    if args.cmd_init {
        return execute_init(args);
    } else if args.cmd_list {
        return execute_list(args);
    } else if args.cmd_import {
        return execute_import(args);
    } else if args.cmd_check_updates {
        return execute_check_updates(args);
    }

    return Err(Error::UnknownCommandError);
}
