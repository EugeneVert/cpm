#[derive(Default, Debug, Clone)]
pub struct Task {
    pub id: usize,
    pub name: String,
    pub duration: i32,
    pub min_start: i32,
    pub max_start: i32,
    pub min_finish: i32,
    pub max_finish: i32,
    pub prev_tasks: Vec<usize>,
}
