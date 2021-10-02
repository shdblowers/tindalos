use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    description: String,
    status: char, // 't' = to-do, 'p' = in progress, 'd' = done
}

#[derive(Serialize, Deserialize, Debug)]
struct Tasks {
    tasks: Vec<Task>,
}

pub fn add(task: String) -> std::io::Result<()> {
    let mut tasks_struct: Tasks = get_tasks();

    let new_task = Task {
        id: get_next_task_id(&tasks_struct.tasks),
        description: task,
        status: 't',
    };

    tasks_struct.tasks.push(new_task);

    save_tasks(tasks_struct)?;

    Ok(())
}

pub fn start(task_id: u32) -> std::io::Result<()> {
    let mut tasks_struct: Tasks = get_tasks();

    let i = tasks_struct
        .tasks
        .iter()
        .position(|t| t.id == task_id)
        .unwrap();

    let mut task_to_update = tasks_struct.tasks.remove(i);

    task_to_update.status = 'p';

    tasks_struct.tasks.push(task_to_update);

    save_tasks(tasks_struct)?;

    Ok(())
}

pub fn finish(task_id: u32) -> std::io::Result<()> {
    let mut tasks_struct: Tasks = get_tasks();

    let i = tasks_struct
        .tasks
        .iter()
        .position(|t| t.id == task_id)
        .unwrap();

    let mut task_to_update = tasks_struct.tasks.remove(i);

    task_to_update.status = 'd';

    tasks_struct.tasks.push(task_to_update);

    save_tasks(tasks_struct)?;

    Ok(())
}

pub fn list() -> std::io::Result<()> {
    let tasks_struct: Tasks = get_tasks();

    println!("Args: {:#?}", tasks_struct);

    Ok(())
}

fn get_tasks() -> Tasks {
    let mut file = File::open("tasks.toml").unwrap_or_else(|_err| {
        std::fs::File::create("tasks.toml").unwrap();
        return File::open("tasks.toml").unwrap();
    });

    let mut buffer = String::new();

    file.read_to_string(&mut buffer).unwrap();

    let t = toml::from_str(&buffer);

    return match t {
        Ok(tasks) => tasks,
        Err(_error) => Tasks { tasks: Vec::new() },
    };
}

fn save_tasks(tasks: Tasks) -> std::io::Result<()> {
    let task_toml = toml::to_vec(&tasks).unwrap();

    let mut task_file = File::create("tasks.toml")?;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_next_lowest_unused_int_for_task_number() {
        let mut existing_tasks = Vec::new();

        let task = Task {
            id: 1,
            description: "buy some milk".to_string(),
            status: 'p',
        };

        existing_tasks.push(task);

        assert_eq!(2, get_next_task_id(&existing_tasks))
    }
}
