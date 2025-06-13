pub mod types;

use utils::types::{CommandStatus, CommandType};
use types::{TaskList, Status};

use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io::prelude::*;

use anyhow::{Context, Result, anyhow};

pub fn open_configuration(path_to_config: &str) -> Result<TaskList> {
    let path = Path::new(path_to_config);

    if !path.exists() {
        File::create(path_to_config)?.write_all(b"{\"list\": []}").expect("Failed to write to file");
    }
    
    let file = File::open(path_to_config)?;
    let result: TaskList = serde_json::from_reader(file)
        .with_context(|| format!("Failed to parse JSON in '{}", path_to_config))?;

    Ok(result)
}

pub fn save_configuration(path_to_config: &str, tasks: &TaskList) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .append(false)
        .open(path_to_config)?;

    file.write(serde_json::to_string_pretty(tasks)?.as_bytes())?;

    Ok(())
}

pub fn run_command(tasks: &mut TaskList, command: CommandType) -> Result<()> {
    match command {
        CommandType::Add { desc } => tasks.add_task(desc),
        CommandType::Edit { id, desc } => tasks.edit_task(id, desc),
        CommandType::List { status } => tasks.list_task(&convert_to_multiple_status(status)),
        CommandType::Delete { id } => tasks.delete_task(id),
        CommandType::Mark { id, status } => tasks.mark_task(id, convert_to_single_status(status)?),
    }
}

pub fn convert_to_multiple_status(c_status: CommandStatus) -> Vec<Status> {
    match c_status {
        CommandStatus::Todo     => vec!(Status::Todo),
        CommandStatus::Progress => vec!(Status::Progress),
        CommandStatus::Done     => vec!(Status::Done),
        CommandStatus::All      => vec!(Status::Todo, Status::Progress, Status::Done),
    }
}

pub fn convert_to_single_status(c_status: CommandStatus) -> Result<Status> {
    match c_status {
        CommandStatus::Todo     => Ok(Status::Todo),
        CommandStatus::Progress => Ok(Status::Progress),
        CommandStatus::Done     => Ok(Status::Done),
        _                       => Err(anyhow!("Command Status not allowed"))
    }
}
