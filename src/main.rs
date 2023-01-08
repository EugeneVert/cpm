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
    Ok(())
}

fn find_critical_path(tasks: &mut [Task]) -> Vec<usize> {
    // Forward
    for i in 0..tasks.len() {
        for j in tasks[i].prev_tasks.clone() {
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

    // Backward
    for i in (0..tasks.len()).rev() {
        for j in tasks[i].prev_tasks.clone() {
            let a = tasks[j].max_finish - tasks[j].duration;
            if a < tasks[j].max_finish {
                tasks[j].max_finish = tasks[i].max_start;
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
