use ac_viewer::{self, run};
use clap::Parser;
use std::process;

#[derive(Parser)]
#[clap(
    name = "ac_viewer",
    author = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION"),
    about = "AtCoder Rust Submissions Viewer"
)]
struct AppArg {
    #[clap(short = 'c', long = "contest_id")]
    contest_id: String,
    #[clap(short = 'p', long = "probrem_id")]
    problem_id: String,
}

#[tokio::main]
async fn main() {
    let arg = AppArg::parse();

    let result = run(arg.contest_id, arg.problem_id).await;

    if let Err(e) = result {
        match e.to_string().as_str() {
            "CTRL+C" => process::exit(0),
            _ => {
                println!("{}", e);
                process::exit(1)
            }
        };
    };
}
