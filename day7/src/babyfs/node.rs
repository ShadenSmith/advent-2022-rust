

use std::cell::RefCell;
use std::rc::Rc;

use crate::babyfs::error::FileSystemError;

type RcRef<T> = Rc<RefCell<T>>;

#[derive(Debug, PartialEq, Clone)]
pub enum NodeType {
    File,
    Directory,
}

#[derive(Debug, Clone)]
pub struct Node {
    name: String,
    which: NodeType,
    size: usize,
    children: Vec<RcRef<Node>>,
}

impl Node {
    pub fn new_dir(name: &str, parent_dir: Option<Rc<RefCell<Node>>>) -> Self {
        Node {
            which: NodeType::Directory,
            name: name.to_string(),
            size: 0,
            children: vec![],
        }
    }

    pub fn new_file(name: &str, size: usize, parent_dir: Option<Rc<Node>>) -> Self {
        Node {
            which: NodeType::File,
            name: name.to_string(),
            size: size,
            children: vec![],
        }
    }

    pub fn get_child_by_name(&self, name: &str) -> Result<Rc<RefCell<Self>>, FileSystemError> {
        for c in &self.children {
            if c.borrow().name == name {
                return Ok(Rc::clone(&c));
            }
        }

        Err(FileSystemError::new("Node not found."))
    }
}
