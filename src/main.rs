use std::{
    error::Error,
    io::{BufWriter, Write},
    path::Path,
};

mod input_parser;
mod task;
use task::Task;

fn main() -> Result<(), Box<dyn Error>> {
    let mut tasks = input_parser::parse_csv_input_file(Path::new("./input.csv"))?;
    let critical_path_idxs = find_critical_path(&mut tasks);
    export_graphviz(&tasks, &critical_path_idxs)?;
    export_stdout(&tasks, &critical_path_idxs);
    Ok(())
}

fn find_critical_path(tasks: &mut [Task]) -> Vec<usize> {
    // Forward
    for i in 0..tasks.len() {
        // find min_start as maximum min_finish of previous nodes
        for j in tasks[i].prev_tasks.clone() {
            let a = tasks[j].min_finish;
            if a > tasks[i].min_start {
                tasks[i].min_start = a;
            }
        }
        // calc min_finish
        tasks[i].min_finish = tasks[i].min_start + tasks[i].duration;
    }

    tasks[tasks.len() - 1].max_finish = tasks[tasks.len() - 1].min_finish;
    tasks[tasks.len() - 1].max_start = tasks[tasks.len() - 1].min_start;

    // Backward
    for i in (0..tasks.len()).rev() {
        for j in tasks[i].prev_tasks.clone() {
            let a = tasks[i].max_start;
            if a < tasks[j].max_finish {
                tasks[j].max_finish = a;
                tasks[j].max_start = tasks[j].max_finish - tasks[j].duration;
            }
        }
    }

    let mut critical_idxs = Vec::<usize>::new();
    for task in tasks {
        if task.min_start == task.max_start {
            critical_idxs.push(task.id);
        }
    }
    critical_idxs
}

fn export_graphviz(tasks: &[Task], critical_path_idxs: &[usize]) -> std::io::Result<()> {
    let file = std::fs::File::create("./graph.dot")?;
    let mut writer = BufWriter::new(file);
    writeln!(writer, "digraph A{{\nrankdir=LR")?;
    for task in tasks {
        writeln!(writer, "{}", task.gen_graphviz_table())?;
    }
    for task in tasks {
        for prev in &task.prev_tasks {
            if critical_path_idxs.contains(prev) && critical_path_idxs.contains(&task.id) {
                writeln!(writer, "Node{} -> Node{} [color=\"blue\"]", prev, task.id)?;
            } else {
                writeln!(writer, "Node{} -> Node{}", prev, task.id)?;
            }
        }
    }
    writeln!(writer, "}}")?;
    Ok(())
}

fn export_stdout(tasks: &[Task], critical_path_idxs: &[usize]) {
    println!("    \tdura \tmin  \tmin   \tmax  \tmax");
    println!("Name\t  tion\tstart\tfinish\tstart\tfinish\tmargin");
    for task in tasks {
        if critical_path_idxs.contains(&task.id) {
            println!(
                "*{}*\t{}\t{}\t{}\t{}\t{}\t0",
                task.name,
                task.duration,
                task.min_start,
                task.min_finish,
                task.max_start,
                task.max_finish
            );
            continue;
        }
        println!(
            " {} \t{}\t{}\t{}\t{}\t{}\t{}",
            task.name,
            task.duration,
            task.min_start,
            task.min_finish,
            task.max_start,
            task.max_finish,
            task.min_finish - task.min_start
        );
    }
}
