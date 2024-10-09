use std::time::{SystemTime, UNIX_EPOCH};
mod codegen;
use codegen::{generate_fl_code, remaining_time};
use clap::{Arg, Command};


fn main() {
    let matches = Command::new("fl-codegen")
        .version("1.0")
        .about("Generates a Family Link code and shows remaining time")
        .bin_name("fl-codegen")
        .arg(
            Arg::new("shared_code")
                .help("Shared code to generate Family Link code")
                .required(true)
                .index(1)
        )
        .arg(
            Arg::new("timestamp")
                .short('t')
                .long("timestamp")
                .help("Optional UNIX timestamp to use instead of current time")
                .num_args(1)
        )
        .get_matches();

    
    let shared_code = matches.get_one::<String>("shared_code").unwrap().clone();

    
    let curr_time_secs = matches
        .get_one::<String>("timestamp")
        .map(|t| t.parse().unwrap_or_else(|_| {
            eprint!("Error: Invalid timestamp specified");
            std::process::exit(1);
        }))
        .unwrap_or_else(|| SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());

    
    println!("Family Link Code: {}", generate_fl_code(shared_code, curr_time_secs));
    print!(
        "Time left: {:02}:{:02} ({} secs)",
        remaining_time(curr_time_secs) / 60,
        remaining_time(curr_time_secs) % 60,
        remaining_time(curr_time_secs)
    );
}