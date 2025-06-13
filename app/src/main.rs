use std::env;

use utils::{parse_command};
use task_utils::{save_configuration, open_configuration, run_command};

use anyhow::{Result};

fn run() -> Result<()>  {
    // read the command
    let args: Vec<String> = env::args().collect();
    let command = parse_command(&args)?;

    // parse the json
    let path_to_config = "./config/task_list.json";
    let mut tasklist = open_configuration(path_to_config)?;

    // doing the changes
    run_command(&mut tasklist, command)?;

    // writing the output
    save_configuration(path_to_config, &tasklist)?;

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    }
}
