use std::{str::FromStr};

#[derive(Debug, PartialEq, )]
pub enum CommandStatus {
    Todo,
    Progress,
    Done, 
    All
}

impl FromStr for CommandStatus {
    type Err = ();

    fn from_str(input: &str) -> Result<CommandStatus, Self::Err> {
        match input {
            "todo"     => Ok(CommandStatus::Todo),
            "progress" => Ok(CommandStatus::Progress),
            "done"     => Ok(CommandStatus::Done),
            "all"     => Ok(CommandStatus::All),
            _          => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum CommandType {
    Add { 
        desc: String 
    },
    Edit {
        id: i32,
        desc: String,
    },
    Delete {
        id: i32,
    },
    List {
        status: CommandStatus,
    },
    Mark  {
        id: i32,
        status: CommandStatus,
    }
}
