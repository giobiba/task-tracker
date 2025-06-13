use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};

use anyhow::{Result};

use core::fmt::Display;

use cli_table::{Table};

use display_tasks::{display_task_list, display_status, display_date,};

#[derive(Serialize, Deserialize, Debug, Table)]
pub struct Task {
    #[table(title = "ID")]
    pub id: i32,

    #[table(title = "Description")]
    pub desc: String,

    #[table(title = "Status", display_fn="display_status")]
    pub status: Status,

    #[table(title = "Created at", display_fn = "display_date")]
    #[serde(with = "my_date_format")]
    pub create_at: DateTime<Local>,

    #[table(title = "Last Update", display_fn = "display_date")]
    #[serde(with = "my_date_format")]
    pub update_at: DateTime<Local>,
}

mod display_tasks{
    use super::*;
    use cli_table::{WithTitle};

    pub fn display_status(status: &Status) -> impl Display {
        match status {
            Status::Todo => "To do",
            Status::Progress => "In Progress",
            Status::Done => "Done",
        }
    }

    pub fn display_date(date: &DateTime<Local>) -> impl Display {
        date.format("%Y-%m-%d %H:%M:%S")
    }

    pub fn display_task_list(task_list: Vec<&Task>) -> impl Display
    {
        task_list.with_title().display().unwrap()
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskList {
    pub list: Vec<Task>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Status {
    Todo,
    Progress,
    Done
}

pub type OpResult = Result<()>;

impl TaskList {
    #![allow(dead_code)]
    fn new() -> Self {
        Self { list: vec![] }
    }

    pub fn get_task(&self, id: i32) -> &Task {
        self.list
            .iter()
            .filter(|t| t.id == id)
            .next()
            .unwrap()
    }

    pub fn get_tasks_with_status(&self, status: &[Status]) -> Vec<&Task> {
        let filtered_tasks: Vec<&Task> = self.list
            .iter()
            .filter(|t| status.contains(&t.status))
            .collect();

        filtered_tasks
    }

    pub fn add_task(&mut self, desc: String) -> OpResult {
        let biggest_id = self.list
            .iter()
            .map(|t| t.id)
            .max()
            .unwrap_or_default();

        let now = Local::now();

        let task = Task {
            id: biggest_id + 1,
            status: Status::Todo,
            desc: desc,
            create_at: now,
            update_at: now,
        };

        println!("Task Added:");
        println!("{}", display_task_list(vec![&task]));

        self.list.push(task);


        Ok(())
    }

    pub fn edit_task(&mut self, id: i32, desc: String) -> OpResult {
        let task = self.list
            .iter_mut()
            .filter(|t| t.id == id)
            .next()
            .unwrap();

        task.desc = desc;
        task.update_at = Local::now();

        println!("Description updated to: {}", task.desc);

        Ok(())
    }

    pub fn delete_task(&mut self, id: i32) -> OpResult {
        let index = self.list
            .iter()
            .position(|t| t.id == id)
            .unwrap();

        let removed_task= self.list.remove(index);

        println!("Removed task: {}", removed_task.desc);

        Ok(())
    }

    pub fn list_task(&self, status: &[Status]) -> OpResult {
        let filtered_tasks: Vec<&Task> = self.list
            .iter()
            .filter(|t| status.contains(&t.status))
            .collect();
        
        if filtered_tasks.len() > 0 {
            println!("{}", display_task_list(filtered_tasks));
        }
        else {
            println!("No tasks with that status");
        }


        Ok(())
    }

    pub fn mark_task(&mut self, id: i32, status: Status) -> OpResult {
        let task = self.list
            .iter_mut()
            .filter(|t| t.id == id)
            .next()
            .unwrap();

        task.status = status;
        task.update_at = Local::now();

        println!("Moved task {} to {}", task.desc, display_status(&task.status));

        Ok(())
    }
}

mod my_date_format {
    use chrono::{DateTime, Local};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &str = "%+";

    pub fn serialize<S>(date: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
        where D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DateTime::parse_from_str(&s, FORMAT)
            .map(Into::into)
            .map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tasklist_tests {
    use super::*;

    #[test]
    fn add_task() -> Result<()> {
        let task_list = &mut TaskList::new();

        task_list.add_task(String::from("test"))?;

        assert_eq!(task_list.list.len(), 1);
        assert_eq!(task_list.get_task(1).desc, "test");

        Ok(())
    }

    #[test]
    fn edit_task() -> Result<()> {
        let task_list = &mut TaskList::new();

        task_list.add_task(String::from("test"))?;
        task_list.edit_task(1, String::from("another test"))?;

        assert_eq!(task_list.list.len(), 1);
        assert_eq!(task_list.get_task(1).desc, "another test");

        Ok(())
    }

    #[test]
    fn delete_task() -> Result<()> {
        let task_list = &mut TaskList::new();

        task_list.add_task(String::from("test"))?;
        task_list.delete_task(1).unwrap();

        assert_eq!(task_list.list.len(), 0);

        Ok(())
    }

    #[test]
    fn mark_task() -> Result<()> {
        let task_list = &mut TaskList::new();

        task_list.add_task(String::from("test"))?;
        task_list.add_task(String::from("test2"))?;

        task_list.mark_task(1, Status::Done)?;

        assert!(task_list.get_tasks_with_status(&[Status::Todo]).contains(&task_list.get_task(2)));
        assert!(task_list.get_tasks_with_status(&[Status::Done]).contains(&task_list.get_task(1)));

        Ok(())
    }
}