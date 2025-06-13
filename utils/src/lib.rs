pub mod types;

use types::*;
use std::str::FromStr;

use anyhow::{Result, anyhow};

pub fn parse_command(args: &[String]) -> Result<CommandType> {
    assert!(args.len() >= 2);

    let command_type = args[1].as_str();
    
    match command_type {
        "add" => Ok(CommandType::Add {
            desc: args[2].clone(),
        }),
        "edit" => Ok(CommandType::Edit {
            id: args[2].parse::<i32>()?,
            desc: args[3].clone(),
        }),
        "list" => Ok(CommandType::List {
            status: parse_status(args.get(2).unwrap_or(&String::from("all")), true)?,
        }),
        "delete" => Ok(CommandType::Delete {
            id: args[2].parse::<i32>()?,
        }),
        "mark" => Ok(CommandType::Mark {
            id: args[2].parse::<i32>()?,
            status: parse_status(&args[2], false)?,
        }),
        unknown_command => Err(anyhow!("Got incorrect command: {unknown_command}"))
    }
}

pub fn parse_status(status: &str, allow_all: bool) -> Result<CommandStatus>{
    match CommandStatus::from_str(status) {
        Ok(CommandStatus::All) => {
            if !allow_all {
                Err(anyhow!("Status not allowed"))
            }
            else {
                Ok(CommandStatus::All)
            }
        },
        Ok(any) => Ok(any),
        Err(_) => Err(anyhow!("Failed to parse status {status}")),
    }   
}

#[cfg(test)]
mod command_tests {
    use super::*;

    #[test]
    fn parse_add() -> Result<()> {
        let command_args = ["_".to_string(), "add".to_string(), "description".to_string()];
        let expected_result = CommandType::Add {
            desc: String::from("description"),
        };

        assert_eq!(expected_result, parse_command(&command_args)?);

        Ok(())
    }

    #[test]
    fn parse_edit() -> Result<()> {
        let command_args = ["_".to_string(), "edit".to_string(), "1".to_string(), "description".to_string()];
        let expected_result = CommandType::Edit {
            id: 1,
            desc: String::from("description"),
        };

        assert_eq!(expected_result, parse_command(&command_args)?);

        Ok(())
    }

    #[test]
    fn parse_delete() -> Result<()> {
        let command_args = ["_".to_string(), "delete".to_string(), "1".to_string()];
        let expected_result = CommandType::Delete {
            id: 1,
        };

        assert_eq!(expected_result, parse_command(&command_args)?);

        Ok(())
    }

    #[test]
    fn parse_list() -> Result<()> {
        let command_args = ["_".to_string(), "list".to_string(), "done".to_string()];
        let expected_result = CommandType::List {
            status: CommandStatus::Done,
        };

        assert_eq!(expected_result, parse_command(&command_args)?);

        Ok(())
    }

    #[test]
    fn parse_mark() -> Result<()> {
        let command_args = ["_".to_string(), "mark".to_string(), "1".to_string(), "todo".to_string()];
        let expected_result = CommandType::Mark {
            id: 1,
            status: CommandStatus::Todo,
        };

        assert_eq!(expected_result, parse_command(&command_args)?);

        Ok(())
    }
}