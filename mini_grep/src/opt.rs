use std::env;

#[derive(PartialEq)]
pub enum CaseType {
    SENSITIVE,
    INSENSITIVE,
}
pub struct CLIArgs {
    pub query: String,
    pub filename: String,
    pub case_sensitive: CaseType
}

impl CLIArgs {
    pub fn new(query: String, filename: String, case_sensitive: CaseType) -> CLIArgs {
        CLIArgs {
            query,
            filename,
            case_sensitive,
        }
    }
}

fn do_parse_args(arg_vec: &Vec<String>) -> Result<CLIArgs, String> {
    if arg_vec.len() < 3 {
        return Result::Err(String::from("Not enough input"));
    }
    // insenstive by default
    let mut case_type = CaseType::INSENSITIVE;

    // if env seted, use sensitive
    let is_env_set = !env::var("GREP_CASE_SENSITIVE").is_err();
    if is_env_set {
        case_type = CaseType::SENSITIVE;
    }

    // whatever, user's input decide how program go
    match arg_vec.len() {
        3 => Result::Ok(CLIArgs::new(arg_vec[1].clone(), arg_vec[2].clone(), case_type)),
        4 => {
            if arg_vec[3].to_lowercase().contains("insensitive") {
                case_type = CaseType::INSENSITIVE;
            } else {
                case_type = CaseType::SENSITIVE;
            }
            Result::Ok(CLIArgs::new(arg_vec[1].clone(), arg_vec[2].clone(), case_type))
        }
        _ => Result::Err(String::from("Unkonwn input"))
    }
}

pub fn parse_args() -> Result<CLIArgs, String> {
    let arg_vec: Vec<String> = env::args().collect();
    return do_parse_args(&arg_vec);
}

#[cfg(test)]
mod tests {
    use crate::opt::*;

    #[test]
    #[should_panic]
    fn parse_empty() {
        let empty_vec: Vec<String> = Vec::new();
        do_parse_args(&empty_vec).unwrap();
    }
}
