use std::rc::Rc;

use std::collections::VecDeque;

use std::borrow::Borrow;

use std::fs::File;
use std::io::{read_to_string, BufReader};

use crate::babyfs::cmds::Cmd;
use crate::babyfs::error::FileSystemError;
use crate::babyfs::node::{Node, NodeType, RcRef};

#[derive(Debug)]
pub struct FileSystem {
    root: RcRef<Node>,
    cwd_stack: VecDeque<RcRef<Node>>,
}

impl FileSystem {
    pub fn new() -> Self {
        let root = Node::new_dir("/");
        FileSystem {
            root: Rc::clone(&root),
            cwd_stack: VecDeque::new(),
        }
    }

    pub fn get_cwd(&self) -> RcRef<Node> {
        Rc::clone(self.cwd_stack.front().expect("No directory set."))
    }

    pub fn cd(&mut self, name: &str) {
        match name {
            "/" => {
                self.cwd_stack.clear();
                self.cwd_stack.push_front(Rc::clone(&self.root));
            }
            ".." => {
                self.cwd_stack
                    .pop_front()
                    .expect("Cannot go up a directory, already at root.");
            }
            _ => {
                let new_cwd = self
                    .cwd_stack
                    .front()
                    .unwrap()
                    .as_ref()
                    .borrow()
                    .get_child_by_name(name)
                    .expect("Directory not found.");
                self.cwd_stack.push_front(Rc::clone(&new_cwd));
            }
        }

        println!("stack is now: {:?}", self.cwd_stack.len());
    }

    pub fn stat_dir(&mut self, name: &str) {
        let curr_dir = self
            .cwd_stack
            .front()
            .expect("Directory stack empty?")
            .as_ref();

        if let Ok(_) = curr_dir.borrow().get_child_by_name(name) {
            println!("{} already found, returning.", name);
            return;
        }

        curr_dir.borrow_mut().add_child(Node::new_dir(name));
    }

    pub fn stat_file(&mut self, name: &str, size: usize) {
        let curr_dir = self
            .cwd_stack
            .front()
            .expect("Directory stack empty?")
            .as_ref();

        if let Ok(_) = curr_dir.borrow().get_child_by_name(name) {
            println!("{} already found, returning.", name);
            return;
        }

        println!("{:?} adding {}", curr_dir.borrow().name, name);
        curr_dir.borrow_mut().add_child(Node::new_file(name, size));
    }

    pub fn from_path(path: &str) -> Result<Self, FileSystemError> {
        let mut reader = BufReader::new(File::open(path).expect("File not found"));
        let mut logs = Vec::new();
        while let Ok(line) = read_to_string(&mut reader) {
            logs.push(line);
        }
        Self::from_log(&logs)
    }

    pub fn from_log<T: Borrow<str>>(log: &[T]) -> Result<Self, FileSystemError> {
        let mut fs = Self::new();

        for line in log {
            println!("Parsing: {}", line.borrow());
            let cmd = Cmd::parse(line.borrow());
            match cmd {
                Cmd::ChangeDir(d) => fs.cd(&d),
                Cmd::ListDir => (),
                Cmd::StatDir(d) => fs.stat_dir(&d),
                Cmd::StatFile(s, n) => fs.stat_file(&n, s),
            };
        }

        Ok(fs)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_fs() {
        let cmds = vec![
            "$ cd /",
            "$ ls",
            "dir a",
            "1234 b.txt",
            "dir d",
            "$ cd d",
            "138 c.txt",
        ];
        let fs = FileSystem::from_log(&cmds).expect("Invalid filesystem log");

        let root = &fs.root.as_ref().borrow();

        assert_eq!(root.name, "/");

        println!("root: {:?}", fs.root);

        // TODO: helper for this verbosity? but borrow() drops at the end of the func, so can't return
        assert_eq!(
            root.get_child_by_name("a").unwrap().as_ref().borrow().which,
            NodeType::Directory
        );
        assert_eq!(
            root.get_child_by_name("b.txt")
                .unwrap()
                .as_ref()
                .borrow()
                .which,
            NodeType::File
        );
        assert_eq!(
            root.get_child_by_name("b.txt")
                .unwrap()
                .as_ref()
                .borrow()
                .size,
            1234
        );
        assert_eq!(
            root.get_child_by_name("d").unwrap().as_ref().borrow().which,
            NodeType::Directory
        );
        assert_eq!(
            root.get_child_by_name("d")
                .unwrap()
                .as_ref()
                .borrow()
                .get_child_by_name("c.txt")
                .unwrap()
                .as_ref()
                .borrow()
                .which,
            NodeType::File
        );
    }
}
