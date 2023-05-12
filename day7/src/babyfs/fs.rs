
use std::cell::RefCell;
use std::rc::Rc;

use std::borrow::Borrow;

use std::fs::File;
use std::io::{read_to_string, BufReader};

use crate::babyfs::error::FileSystemError;
use crate::babyfs::node::Node;
use crate::babyfs::cmds::Cmd;


#[derive(Debug)]
pub struct FileSystem {
    root: Rc<RefCell<Node>>,
    cwd: Rc<RefCell<Node>>,
}


impl FileSystem {
    pub fn new() -> Self {
        let root = Rc::new(RefCell::new(Node::new_dir("/", None)));
        FileSystem {
            root: Rc::clone(&root),
            cwd: Rc::clone(&root),
        }
    }

    pub fn cd(&mut self, name: &str) {
        if name == "/" {
            self.cwd = Rc::clone(&self.root);
            return;
        }

        //let new_cwd = match name {
        //    "/" => Ok(Rc::clone(&self.root)),
        //    ".." => todo!(),
        //       // match &self.cwd.parent {
        //       // Some(n) => Ok(Rc::clone(&n)),
        //       // None => Err(FileSystemError {
        //       //     msg: String::from("Invalid directory"),
        //       // }),
        //    _ => todo!(),// self.cwd.get_child_by_name(name),
        //};

        //if new_cwd.is_ok() {
        //    self.cwd = new_cwd.unwrap();
        //} else {
        //    panic!("Could not change directory.");
        //}
    }

    pub fn stat_dir(&mut self, name: &str) {
        // // Walk the children and insert new directory if necessary
        // for child in &self.cwd.children {
        //     if child.name == name {
        //         println!("{} already found, returning.", name);
        //         return;
        //     }
        // }

        // let parent = Some(Rc::clone(&self.cwd));

        // // Build new node
        // let mut cwd_mut = Rc::make_mut(&mut self.cwd);
        // cwd_mut.children.push(Rc::new(Node::new_dir(name, parent)));
    }

    pub fn stat_file(&mut self, name: &str, size: usize) {
       // // Walk the children and insert new file if necessary
       // for child in &self.cwd.children {
       //     if child.name == name {
       //         assert_eq!(child.size, size);
       //         return;
       //     }
       // }

       // // Build new node
       // let parent = Some(Rc::clone(&self.cwd));
       // let mut cwd_mut = Rc::make_mut(&mut self.cwd);
       // cwd_mut
       //     .children
       //     .push(Rc::new(Node::new_file(name, size, parent)));
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
            println!("FS is now: {:?}", fs);
        }

        Ok(fs)
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn parse_fs() {
        todo!();
        //let cmds = vec!["$ cd /", "$ ls", "dir a", "1234 b.txt", "dir d"];
        //let fs = FileSystem::from_log(&cmds);

        //assert!(fs.is_ok());
        //let fs = fs.unwrap();

        //assert_eq!(fs.root.name, "/");

        //println!("root: {:?}", fs.root);

        //assert_eq!(fs.root.children[0].name, "a");
        //assert_eq!(fs.root.children[0].which, NodeType::Directory);
        //assert_eq!(fs.root.children[1].name, "b.txt");
        //assert_eq!(fs.root.children[1].which, NodeType::File);
        //assert_eq!(fs.root.children[2].name, "d");
        //assert_eq!(fs.root.children[2].which, NodeType::Directory);
    }
}