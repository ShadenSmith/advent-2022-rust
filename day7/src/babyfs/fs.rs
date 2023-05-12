use std::rc::Rc;

use std::collections::VecDeque;

use std::borrow::Borrow;

use std::fs::File;
use std::io::{BufRead, BufReader};

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
    }

    pub fn stat_dir(&mut self, name: &str) {
        let curr_dir = self
            .cwd_stack
            .front()
            .expect("Directory stack empty?")
            .as_ref();

        if let Ok(_) = curr_dir.borrow().get_child_by_name(name) {
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
            return;
        }

        curr_dir.borrow_mut().add_child(Node::new_file(name, size));
    }

    pub fn from_path(path: &str) -> Result<Self, FileSystemError> {
        let mut reader = BufReader::new(File::open(path).expect("File not found"));
        let mut logs = Vec::new();
        for line in reader.lines() {
            let line = line.unwrap();
            logs.push(line);
        }
        Self::from_log(&logs)
    }

    pub fn from_log<T: Borrow<str>>(log: &[T]) -> Result<Self, FileSystemError> {
        let mut fs = Self::new();

        for line in log {
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

    #[test]
    fn test_from_path() {
        let _ = FileSystem::from_path("input_test.txt").unwrap();
    }
}
