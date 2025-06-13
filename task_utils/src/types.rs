use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};

use anyhow::{Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: i32,
    pub status: Status,
    pub desc: String,
    #[serde(with = "my_date_format")]
    pub create_at: DateTime<Local>,
    #[serde(with = "my_date_format")]
    pub update_at: DateTime<Local>,
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

pub type OpResult =  Result<()>;

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

        println!("Added task:\n {:#?}", task);

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

        println!("Updated description for task:\n {:#?}", task.desc);

        Ok(())
    }

    pub fn delete_task(&mut self, id: i32) -> OpResult {
        let index = self.list
            .iter()
            .position(|t| t.id == id)
            .unwrap();

        let removed_task= self.list.remove(index);

        println!("Removed task:\n {:#?}", removed_task);

        Ok(())
    }

    pub fn list_task(&self, status: &[Status]) -> OpResult {
        let filtered_tasks: Vec<&Task> = self.list
            .iter()
            .filter(|t| status.contains(&t.status))
            .collect();

        println!("Task list:\n {:#?}", filtered_tasks);
        
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

        println!("Task updated to:\n {:#?}", task.status);

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