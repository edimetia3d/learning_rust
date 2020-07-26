pub mod opt;

use std::fs;
use std::error::Error;

use opt::CLIArgs;

pub fn grep_on_cli_arg(arg: &CLIArgs) -> Result<String, Box<dyn Error>> {
    let filterd_str = fs::read_to_string(&arg.filename)?;
    return Result::Ok(filterd_str);
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    return vec![];
}

#[cfg(test)]
mod test_search {
    use super::*;

    #[test]
    fn one_line() {
        let query = "son";
        let content = "?\
        come,\
        my son,\
        it is your time";

        let expected = vec!["my son,"];

        assert_eq!(expected, search(query, content));
    }
}