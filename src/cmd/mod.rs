mod add;
mod init;
mod list;
mod check_updates;
mod remove;

use super::Args;
use self::add::execute_add;
use self::init::execute_init;
use self::list::execute_list;
use self::check_updates::execute_check_updates;
use self::remove::execute_remove;
use crate::error::Error;

pub fn execute(args: &Args) -> Result<(), Error> {
    if args.cmd_init {
        return execute_init(args);
    } else if args.cmd_list {
        return execute_list(args);
    } else if args.cmd_add {
        return execute_add(args);
    } else if args.cmd_remove {
        return execute_remove(args);
    } else if args.cmd_check_updates {
        return execute_check_updates(args);
    }

    return Err(Error::UnknownCommandError);
}
