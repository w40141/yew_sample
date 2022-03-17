#[derive(PartialEq, Clone, Debug)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub completed: bool,
}
