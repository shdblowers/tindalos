use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize)]
struct Task {
    id: u32,
    description: String,
    status: char, // 't' = to-do, 'p' = in progress, 'd' = done
    date_done: Option<chrono::DateTime<chrono::Utc>>,
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "#{:03}: {}\n", self.id, self.description);
    }
}

#[derive(Serialize, Deserialize)]
struct Tasks {
    tasks: Vec<Task>,
}

impl std::fmt::Display for Tasks {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let done_tasks = self.tasks.iter().filter(|t| t.status == 'd');

        if done_tasks.clone().count() > 0 {
            write!(f, "\nDONE\n====\n")?;
        }

        done_tasks.clone().for_each(|t| t.fmt(f).unwrap());

        let in_progress_tasks = self.tasks.iter().filter(|t| t.status == 'p');

        if in_progress_tasks.clone().count() > 0 {
            write!(f, "\nIN PROGRESS\n===========\n")?;
        }

        in_progress_tasks.clone().for_each(|t| t.fmt(f).unwrap());

        let to_do_tasks = self.tasks.iter().filter(|t| t.status == 't');

        if to_do_tasks.clone().count() > 0 {
            write!(f, "\nTODO\n====\n")?;
        }

        to_do_tasks.clone().for_each(|t| t.fmt(f).unwrap());

        if done_tasks.count() + in_progress_tasks.count() + to_do_tasks.count() == 0 {
            write!(f, "\nNo tasks found!\n")?;
        }

        return write!(f, "");
    }
}

pub fn add(task: String) -> std::io::Result<()> {
    let mut tasks_struct: Tasks = get_tasks();

    let new_task = Task {
        id: get_next_task_id(&tasks_struct.tasks),
        description: task,
        status: 't',
        date_done: None,
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
    task_to_update.date_done = Some(chrono::Utc::now());

    tasks_struct.tasks.push(task_to_update);

    save_tasks(tasks_struct)?;

    Ok(())
}

pub fn list() -> std::io::Result<()> {
    let tasks_struct: Tasks = get_tasks();

    println!("{}", tasks_struct);

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
            date_done: None,
        };

        existing_tasks.push(task);

        assert_eq!(2, get_next_task_id(&existing_tasks))
    }

    #[test]
    fn prints_a_task_in_nice_format() {
        let task_todo = Task {
            id: 33,
            description: "ring up john".to_string(),
            status: 't',
            date_done: None,
        };

        assert_eq!("#033: ring up john\n", format!("{}", task_todo));
    }
}
