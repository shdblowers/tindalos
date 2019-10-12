use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Tasks {
    filename: String,
    tasks: Vec<Task>,
}

pub fn add(task: String) -> std::io::Result<()> {
    let mut tasks_struct: Tasks = get_tasks();

    let new_task = Task {
        id: get_next_task_id(&tasks_struct.tasks),
        description: task,
    };

    tasks_struct.tasks.push(new_task);

    let task_toml = toml::to_vec(&tasks_struct).unwrap();

    let mut task_file = File::create("tasks.toml")?;

    task_file.write_all(&task_toml)?;

    Ok(())
}

pub fn list() -> std::io::Result<()> {
    let tasks_struct: Tasks = get_tasks();

    println!("Args: {:#?}", tasks_struct);

    Ok(())
}

fn get_tasks() -> Tasks {
    let mut f = File::open("tasks.toml").unwrap();
    let mut buffer = String::new();

    f.read_to_string(&mut buffer).unwrap();

    return toml::from_str(&buffer).unwrap();
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
            description: "buy some milk".to_string()
        };

        existing_tasks.push(task);


        assert_eq!(2, get_next_task_id(&existing_tasks))
    }
}
