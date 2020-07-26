use std::env;

pub struct CLIArgs {
    pub query: String,
    pub filename: String,
}

impl CLIArgs {
    pub fn new(query: String, filename: String) -> CLIArgs {
        CLIArgs {
            query,
            filename,
        }
    }
}

fn do_parse_args(arg_vec: &Vec<String>) -> Result<CLIArgs, String> {
    if arg_vec.len() < 3 {
        return Result::Err(String::from("Not enough input"));
    }
    match arg_vec.len() {
        3 => Result::Ok(CLIArgs::new(arg_vec[1].clone(), arg_vec[2].clone())),
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
