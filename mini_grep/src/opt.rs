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

pub fn parse_args() -> Result<CLIArgs, String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return Result::Err(String::from("Not enough input"));
    }
    match args.len() {
        3 => Result::Ok(CLIArgs::new(args[1].clone(), args[2].clone())),
        _ => Result::Err(String::from("Unkonwn input"))
    }
}
