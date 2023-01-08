use crate::task::Task;
use std::{collections::BTreeMap, error::Error, io::prelude::*, path::Path};

pub fn parse_csv_input_file(filepath: &Path) -> Result<Vec<Task>, Box<dyn Error>> {
    let mut res = Vec::<Task>::new();
    let mut prev_map = BTreeMap::<String, usize>::new();

    // Parse csv
    let file = std::fs::File::open(filepath)?;
    let reader = std::io::BufReader::new(file);
    for line in reader.lines().skip(1) {
        let line = line?;
        // Comma separated
        let mut split = line.split(',');
        // 3 Fields
        let (name, duration, prev_nodes_names) = || -> Option<_> {
            Some((
                // name
                split.next()?.to_string(),
                // duration
                split.next()?.parse::<i32>().unwrap(),
                // prev_nodes_names list separated by spaces
                split
                    .next()?
                    .trim()
                    .split(' ')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string()),
            ))
        }()
        .unwrap();

        // Replace prev_nodes names with idxs
        let id = res.len();
        let prev_tasks = prev_nodes_names
            .map(|node| prev_map.get(&node).unwrap().to_owned())
            .collect();
        prev_map.entry(name.clone()).or_insert(id);

        res.push(Task {
            id,
            name,
            duration,
            prev_tasks,
            ..Default::default()
        });
    }
    Ok(res)
}
