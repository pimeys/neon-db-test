#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: usize,
    name: String,
    age: u16,
}
