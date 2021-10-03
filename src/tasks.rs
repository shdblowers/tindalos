use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter, Result};

#[derive(Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
  Todo,
  InProgress,
  Done,
}

#[derive(Serialize, Deserialize)]
pub struct Task {
  pub id: u32,
  pub description: String,
  pub status: TaskStatus,
  pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct Tasks {
  pub tasks: Vec<Task>,
}

impl Display for Task {
  fn fmt(&self, f: &mut Formatter) -> Result {
    return write!(f, "#{:03}: {}\n", self.id, self.description);
  }
}

impl Display for Tasks {
  fn fmt(&self, f: &mut Formatter) -> Result {
    let done_tasks = self.tasks.iter().filter(|t| t.status == TaskStatus::Done);

    let mut done_tasks_by_dates: BTreeMap<String, Vec<&Task>> = BTreeMap::new();

    for task in done_tasks.clone() {
      let task_done_date: String = task.updated_at.format("%a (%y-%m-%d)").to_string();
      done_tasks_by_dates
        .entry(task_done_date)
        .or_insert(Vec::new())
        .push(task.clone())
    }

    for (date, tasks) in done_tasks_by_dates {
      write!(f, "\nDone {}\n{}\n", date, divider_string(date.len() + 5))?;
      tasks.iter().for_each(|t| t.fmt(f).unwrap());
    }

    let in_progress_tasks = self
      .tasks
      .iter()
      .filter(|t| t.status == TaskStatus::InProgress);

    if in_progress_tasks.clone().count() > 0 {
      write!(f, "\nIn Progress\n{}\n", divider_string(11))?;
    }

    in_progress_tasks.clone().for_each(|t| t.fmt(f).unwrap());

    let to_do_tasks = self.tasks.iter().filter(|t| t.status == TaskStatus::Todo);

    if to_do_tasks.clone().count() > 0 {
      write!(f, "\nTodo\n{}\n", divider_string(4))?;
    }

    to_do_tasks.clone().for_each(|t| t.fmt(f).unwrap());

    if done_tasks.count() + in_progress_tasks.count() + to_do_tasks.count() == 0 {
      write!(f, "\nNo tasks found!\n")?;
    }

    return write!(f, "");
  }
}

fn divider_string(len: usize) -> String {
  let mut s = String::new();

  for _ in 0..len {
    s.push_str("=")
  }

  return s;
}
