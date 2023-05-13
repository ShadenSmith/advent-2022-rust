use std::cell::RefCell;
use std::rc::Rc;

use crate::babyfs::error::FileSystemError;

pub type RcRef<T> = Rc<RefCell<T>>;

#[derive(Debug, PartialEq, Clone)]
pub enum NodeType {
    File,
    Directory,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub which: NodeType,
    pub size: usize,
    pub children: Vec<RcRef<Node>>,
}

impl Node {
    pub fn new_dir(name: &str) -> RcRef<Self> {
        Rc::new(RefCell::new(Node {
            which: NodeType::Directory,
            name: name.to_string(),
            size: 0,
            children: vec![],
        }))
    }

    pub fn new_file(name: &str, size: usize) -> RcRef<Self> {
        Rc::new(RefCell::new(Node {
            which: NodeType::File,
            name: name.to_string(),
            size,
            children: vec![],
        }))
    }

    pub fn get_child_by_name(&self, name: &str) -> Result<RcRef<Self>, FileSystemError> {
        for c in &self.children {
            if c.borrow().name == name {
                return Ok(Rc::clone(c));
            }
        }

        Err(FileSystemError::new("Node not found."))
    }

    pub fn add_child(&mut self, node: RcRef<Node>) {
        self.children.push(Rc::clone(&node));
    }

    pub fn get_size(&self) -> usize {
        match self.which {
            NodeType::File => self.size,
            NodeType::Directory => self.children.iter().map(|n| n.borrow().get_size()).sum(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_node_new() {
        let root_ref = Node::new_dir("/");
        let node = root_ref.borrow();

        assert_eq!(node.name, "/");
        assert!(node.children.is_empty());
        assert_eq!(node.which, NodeType::Directory);
    }

    #[test]
    fn test_node_add() {
        let root = Node::new_dir("/");
        {
            let mut mut_root = root.borrow_mut();
            mut_root.add_child(Node::new_file("Barnacle", 8));
        }

        assert_eq!(root.borrow().children.len(), 1);
        assert_eq!(root.borrow().children[0].borrow().name, "Barnacle");
        assert_eq!(
            root.borrow()
                .get_child_by_name("Barnacle")
                .unwrap()
                .borrow()
                .size,
            8
        );
    }

    #[test]
    fn test_dir_size() {
        let root = Node::new_dir("/");
        {
            let mut mut_root = root.borrow_mut();
            mut_root.add_child(Node::new_file("Barnacle", 8));
            mut_root.add_child(Node::new_dir("Beluga"));
            mut_root
                .get_child_by_name("Beluga")
                .unwrap()
                .borrow_mut()
                .add_child(Node::new_file("Cheeto", 2));
        }

        assert_eq!(root.borrow().get_size(), 10);
        assert_eq!(
            root.borrow()
                .get_child_by_name("Beluga")
                .unwrap()
                .borrow()
                .get_size(),
            2
        );
    }
}
