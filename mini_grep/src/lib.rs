pub mod opt;

use std::fs;
use std::error::Error;

use opt::CLIArgs;

pub fn grep_on_cli_arg(arg: &CLIArgs) -> Result<String, Box<dyn Error>> {
    let filterd_str = fs::read_to_string(&arg.filename)?;
    return Result::Ok(filterd_str);
}