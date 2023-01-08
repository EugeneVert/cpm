use std::{cell::RefCell, error::Error, path::Path, rc::Rc};

mod input_parser;
mod task;
use task::Task;

fn main() -> Result<(), Box<dyn Error>> {
    let mut tasks = input_parser::parse_csv_input_file(Path::new("./input.csv"))?;
    find_critical_path(&mut tasks);
    for task in &tasks {
        println!("{:?}", task);
    }
    println!("Hello, world!");
    Ok(())
}

fn find_critical_path(tasks: &mut [Task]) {
    let adjacency_matrix = generate_adjacency_matrix(tasks);
    for i in &adjacency_matrix {
        println!("{:?}", i);
    }
    // Forward
    for i in 0..tasks.len() {
        for j in 0..i {
            if adjacency_matrix[i][j] == 0 {
                continue;
            }
            let a = tasks[j].min_finish + tasks[i].duration;
            if a > tasks[i].min_finish {
                tasks[i].min_finish = a;
                tasks[i].max_finish = a;
            }
        }
        let b = tasks[i].min_finish - tasks[i].duration;
        if b > tasks[i].min_start {
            tasks[i].min_start = b;
            tasks[i].max_start = b;
        }
    }

    println!();

    // Backward

    for i in (0..tasks.len()).rev() {
        for j in 0..i {
            if adjacency_matrix[i][j] == 0 {
                continue;
            }
            let a = tasks[j].max_finish - tasks[j].duration;
            if j == 0 {
                println!("{:?}", tasks[i]);
                println!("{:?}", tasks[j]);
                println!("{}", a);
                println!("{}", tasks[i].max_start);
            }
            if a < tasks[j].max_finish {
                tasks[j].max_finish = tasks[i].max_start;
                tasks[j].max_start = tasks[j].max_finish - tasks[j].duration;
            }
        }
    }

    println!();
    // for task in tasks.get_mut().iter() {
    //     for prev in task.get_prev_tasks(tasks.borrow().as_ref()) {
    //         println!("{:?}", prev);
    //     }
    // for prev in task.get_prev_tasks(&Rc::clone(&tasks)) {
    //     let a = prev.min_start + task.duration;
    //     if a < task.min_start {
    //         task.min_start = a;
    //     }
    // }
    // }
}

fn generate_adjacency_matrix(tasks: &[Task]) -> Vec<Vec<u8>> {
    let mut adjacency_matrix = vec![vec![0; tasks.len()]; tasks.len()];
    for task in tasks {
        for prev in &task.prev_tasks {
            adjacency_matrix[task.id][*prev] = 1;
        }
    }
    adjacency_matrix
}
