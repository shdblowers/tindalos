use std::env;
use tindalos;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        args.push("list".to_string());
    }

    match args[1].as_ref() {
        "add" => {
            let task = args[2..].join(" ");
            tindalos::add(task).unwrap();
            tindalos::list().unwrap();
        }
        "start" => {
            let task_id_string = args[2].clone();
            let task_id = task_id_string.parse::<u32>().unwrap();

            tindalos::start(task_id).unwrap();
            tindalos::list().unwrap();
        }
        "finish" => {
            let task_id_string = args[2].clone();
            let task_id = task_id_string.parse::<u32>().unwrap();

            tindalos::finish(task_id).unwrap();
            tindalos::list().unwrap();
        }
        "list" => tindalos::list().unwrap(),
        _ => (),
    }
}
