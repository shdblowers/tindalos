extern crate tindalos;

use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        args.push("list".to_string());
    }

    match args[1].as_ref() {
        "add" => {
            let task = args[2].clone();
            tindalos::add(task).unwrap();
        }
        "list" => tindalos::list().unwrap(),
        _ => (),
    }
}
