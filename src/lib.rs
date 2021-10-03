use crate::tasks::{Task, TaskStatus, Tasks};
use chrono::Utc;
use std::fs::File;
use std::io::{Read, Result as IoResult, Write};
use std::path::PathBuf;

mod tasks;

pub fn add(task: String) -> IoResult<()> {
    let mut tasks_struct: Tasks = get_tasks();

    let new_task = Task {
        id: get_next_task_id(&tasks_struct.tasks),
        description: task,
        status: TaskStatus::Todo,
        updated_at: Utc::now(),
    };

    tasks_struct.tasks.push(new_task);

    save_tasks(tasks_struct)?;

    Ok(())
}

pub fn start(task_id: u32) -> IoResult<()> {
    let mut tasks_struct: Tasks = get_tasks();

    let i = tasks_struct
        .tasks
        .iter()
        .position(|t| t.id == task_id)
        .unwrap();

    let mut task_to_update = tasks_struct.tasks.remove(i);

    task_to_update.status = TaskStatus::InProgress;

    tasks_struct.tasks.push(task_to_update);

    save_tasks(tasks_struct)?;

    Ok(())
}

pub fn finish(task_id: u32) -> IoResult<()> {
    let mut tasks_struct: Tasks = get_tasks();

    let i = tasks_struct
        .tasks
        .iter()
        .position(|t| t.id == task_id)
        .unwrap();

    let mut task_to_update = tasks_struct.tasks.remove(i);

    task_to_update.status = TaskStatus::Done;
    task_to_update.updated_at = Utc::now();

    tasks_struct.tasks.push(task_to_update);

    save_tasks(tasks_struct)?;

    Ok(())
}

pub fn list() -> IoResult<()> {
    let tasks_struct: Tasks = get_tasks();

    println!("{}", tasks_struct);

    Ok(())
}

fn get_tasks() -> Tasks {
    let config_file_path = get_tasks_file_path();

    let mut file = File::open(&config_file_path).unwrap_or_else(|_err| {
        File::create(&config_file_path).unwrap();
        return File::open(&config_file_path).unwrap();
    });

    let mut buffer = String::new();

    file.read_to_string(&mut buffer).unwrap();

    let t = toml::from_str(&buffer);

    return match t {
        Ok(tasks) => tasks,
        Err(_error) => Tasks { tasks: Vec::new() },
    };
}

fn save_tasks(tasks: Tasks) -> IoResult<()> {
    let task_toml = toml::to_vec(&tasks).unwrap();
    let config_file_path = get_tasks_file_path();
    let mut task_file = File::create(&config_file_path)?;

    return task_file.write_all(&task_toml);
}

fn get_next_task_id(tasks: &Vec<Task>) -> u32 {
    let mut highest_id: u32 = 0;

    for task in tasks {
        if task.id > highest_id {
            highest_id = task.id;
        }
    }

    return highest_id + 1;
}

fn get_tasks_file_path() -> PathBuf {
    let mut config_file_path: PathBuf = dirs::config_dir().unwrap();
    config_file_path.push("tindalos_tasks");
    config_file_path.set_extension("toml");

    return config_file_path;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_next_lowest_unused_int_for_task_number() {
        let mut existing_tasks = Vec::new();

        let task = Task {
            id: 1,
            description: "buy some milk".to_string(),
            status: TaskStatus::InProgress,
            updated_at: Utc::now(),
        };

        existing_tasks.push(task);

        assert_eq!(2, get_next_task_id(&existing_tasks))
    }

    #[test]
    fn prints_a_task_in_nice_format() {
        let task_todo = Task {
            id: 33,
            description: "ring up john".to_string(),
            status: TaskStatus::Todo,
            updated_at: Utc::now(),
        };

        assert_eq!("#033: ring up john\n", format!("{}", task_todo));
    }
}
