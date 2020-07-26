pub mod opt;

use std::fs;
use std::error::Error;

use opt::CLIArgs;
use crate::opt::CaseType;

pub fn grep_on_cli_arg(arg: &CLIArgs) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(&arg.filename)?;
    let ans;
    if arg.case_sensitive == CaseType::SENSITIVE {
        ans = case_sensitive_search(&arg.query, &content);
    } else {
        ans = case_insensitive_search(&arg.query, &content);
    }

    let mut ret: String = "".to_string();
    for line in ans.iter() {
        ret.push_str(line);
        ret.push('\n');
    }
    let summary = format!("======Summary======\n{} lines found \n", ans.len());
    ret.push_str(&summary);
    return Result::Ok(ret);
}

fn case_insensitive_search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let lowerd_query = query.to_lowercase();
    let mut ret = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&lowerd_query) {
            ret.push(line);
        }
    }
    return ret;
}

fn case_sensitive_search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
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

        assert_eq!(expected, case_sensitive_search(query, content));
    }

    #[test]
    fn case_sensitive_one_line() {
        let content =
            "?
Come,
my Son,
it IS your time";

        let empty_ans: Vec<&str> = vec![];
        assert_eq!(empty_ans, case_sensitive_search("come", content));
        assert_eq!(vec!["Come,"], case_sensitive_search("Come", content));

        assert_eq!(empty_ans, case_sensitive_search("son", content));
        assert_eq!(vec!["my Son,"], case_sensitive_search("Son", content));

        assert_eq!(empty_ans, case_sensitive_search("Is", content));
        assert_eq!(vec!["it IS your time"], case_sensitive_search("IS", content));
    }

    #[test]
    fn case_insensitive_one_line() {
        let content =
            "?
Come,
my Son,
it IS your time";

        let empty_ans: Vec<&str> = vec![];
        let mut expected = vec!["Come,"];
        assert_eq!(expected, case_insensitive_search("come", content));
        assert_eq!(expected, case_insensitive_search("Come", content));

        expected = vec!["my Son,"];
        assert_eq!(expected, case_insensitive_search("son", content));
        assert_eq!(expected, case_insensitive_search("Son", content));

        expected = vec!["it IS your time"];
        assert_eq!(expected, case_insensitive_search("Is", content));
        assert_eq!(expected, case_insensitive_search("IS", content));
    }
}