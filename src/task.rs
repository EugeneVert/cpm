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

impl Task {
    pub fn gen_graphviz_table(&self) -> String {
        format!(
            r#"Node{id} [shape=none margin=0 label=
<<table border="0" cellspacing="0" cellborder="1">
<tr>
<td width="25">{name}</td>
<td width="25">{duration}</td>
</tr>
<tr>
<td width="25">{min_start}</td>
<td width="25">{min_finish}</td>
</tr>
<tr>
<td width="25">{max_start}</td>
<td width="25">{max_finish}</td>
</tr>
</table>>
]
            "#,
            name = self.name,
            id = self.id,
            duration = self.duration,
            min_start = self.min_start,
            min_finish = self.min_finish,
            max_start = self.max_start,
            max_finish = self.max_finish,
        )
    }
}
