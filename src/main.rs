use std::collections::HashMap;
use std::fs;
use std::io;

fn main() {
    let mut file_path = String::new();

    println!("Please give the file path: ");
    io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to read line");

    let file_path = file_path.trim();

    let file_contents = file_to_string(file_path);
    println!("The file reads:\n{file_contents}\nEOF");
}

fn file_to_string(file_path: &str) -> String {
    let file_contents = fs::read_to_string(file_path.trim()).expect("Failed to read the file");

    file_contents
}

// Nested Map Data Structure
pub enum NestedMapNodeData<'a> {
    Value(String),
    Map(HashMap<String, NestedMap<'a>>),
}
pub struct NestedMap<'a> {
    data: NestedMapNodeData<'a>,
    parent_node: Option<&'a HashMap<String, NestedMapNodeData<'a>>>, // if it has a parent, the parent is a map
    child_node: Option<&'a NestedMapNodeData<'a>>,                   // a child could be anything
}
impl<'a> NestedMap<'a> {
    pub fn new() -> Self {
        Self {
            data: , // I DON'T KNOW YET WHAT TO PUT HERE
            parent_node: None, // it is the root, therefore no parent
            child_node: None,
        }
    }
    pub fn insert(&mut self, key: &str, value: NestedMapNodeData) {
        self.root_node.insert(key.to_string(), value);
    }
}
