pub mod opt;

use std::fs;
use std::error::Error;

use opt::CLIArgs;

pub fn grep_on_cli_arg(arg: &CLIArgs) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(&arg.filename)?;
    let ans = search(&arg.query, &content);
    let mut ret: String = "".to_string();
    for line in ans.iter() {
        ret.push_str(line);
        ret.push('\n');
    }
    let summary = format!("======Summary======\n{} lines found \n", ans.len());
    ret.push_str(&summary);
    return Result::Ok(ret);
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut ret = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            ret.push(line);
        }
    }
    return ret;
}

#[cfg(test)]
mod test_search {
    use super::*;

    #[test]
    fn one_line() {
        let query = "son";
        let content =
            "?
come,
my son,
it is your time";

        let expected = vec!["my son,"];

        assert_eq!(expected, search(query, content));
    }
}