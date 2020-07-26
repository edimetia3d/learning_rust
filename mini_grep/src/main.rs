use std::process;
use mini_grep;

fn main() {
    let cli_arg = mini_grep::opt::parse_args().unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let filterd_str = mini_grep::grep_on_cli_arg(&cli_arg).unwrap_or_else(|err| {
        println!("Failed grep in file:{}\n{}", cli_arg.filename, err);
        process::exit(2);
    });
    println!("Content:\n {}", filterd_str);
}
