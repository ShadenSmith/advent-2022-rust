use lazy_static::lazy_static;
use regex::Regex;
#[allow(dead_code, unused_variables)]
use std::borrow::Borrow;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{read_to_string, BufRead, BufReader, Read};
use std::rc::Rc;

#[derive(Debug)]
struct FileSystemError {
    msg: String,
}
impl fmt::Display for FileSystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for FileSystemError {}

#[derive(Debug, PartialEq, Clone)]
enum NodeType {
    File,
    Directory,
}

#[derive(Debug, PartialEq)]
enum Cmd {
    ChangeDir(String),
    ListDir,
    StatDir(String),
    StatFile(usize, String),
}

impl Cmd {
    pub fn parse(line: &str) -> Self {
        lazy_static! {
            static ref RE_CHANGE_DIR: Regex = Regex::new(r"\$ cd (.*)").unwrap();
            static ref RE_LIST_DIR: Regex = Regex::new(r"\$ ls").unwrap();
            static ref RE_STAT_DIR: Regex = Regex::new(r"dir (.*)").unwrap();
            static ref RE_STAT_FILE: Regex = Regex::new(r"(\d+) (.*)").unwrap();
        }

        if let Some(cap) = RE_CHANGE_DIR.captures(line) {
            return Cmd::ChangeDir(cap.get(1).unwrap().as_str().to_string());
        }

        if let Some(cap) = RE_LIST_DIR.captures(line) {
            return Cmd::ListDir;
        }

        if let Some(cap) = RE_STAT_DIR.captures(line) {
            return Cmd::StatDir(cap.get(1).unwrap().as_str().to_string());
        }

        if let Some(cap) = RE_STAT_FILE.captures(line) {
            return Cmd::StatFile(
                cap.get(1).unwrap().as_str().parse().unwrap(),
                cap.get(2).unwrap().as_str().to_string(),
            );
        }

        panic!("Could not parse command: {}", line);
    }
}

#[derive(Debug, Clone)]
struct Node {
    name: String,
    which: NodeType,
    size: usize,
    children: Vec<Rc<Node>>,
    parent: Option<Rc<Node>>,
}

impl Node {
    pub fn new_dir(name: &str, parent_dir: Option<Rc<Node>>) -> Self {
        Node {
            which: NodeType::Directory,
            name: name.to_string(),
            size: 0,
            children: vec![],
            parent: parent_dir,
        }
    }

    pub fn new_file(name: &str, size: usize, parent_dir: Option<Rc<Node>>) -> Self {
        Node {
            which: NodeType::File,
            name: name.to_string(),
            size: size,
            children: vec![],
            parent: parent_dir,
        }
    }

    pub fn get_child_by_name(&self, name: &str) -> Result<Rc<Self>, FileSystemError> {
        for c in &self.children {
            if c.name == name {
                return Ok(Rc::clone(c));
            }
        }

        Err(FileSystemError {
            msg: String::from("Node not found."),
        })
    }
}

#[derive(Debug)]
struct FileSystem {
    root: Rc<Node>,
    cwd: Rc<Node>,
}

impl FileSystem {
    pub fn new() -> Self {
        let root = Rc::new(Node::new_dir("/", None));
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

        let new_cwd = match name {
            "/" => Ok(Rc::clone(&self.root)),
            ".." => match &self.cwd.parent {
                Some(n) => Ok(Rc::clone(&n)),
                None => Err(FileSystemError {
                    msg: String::from("Invalid directory"),
                }),
            },
            _ => self.cwd.get_child_by_name(name),
        };

        if new_cwd.is_ok() {
            self.cwd = new_cwd.unwrap();
        } else {
            panic!("Could not change directory.");
        }
    }

    pub fn stat_dir(&mut self, name: &str) {
        // Walk the children and insert new directory if necessary
        for child in &self.cwd.children {
            if child.name == name {
                println!("{} already found, returning.", name);
                return;
            }
        }

        let parent = Some(Rc::clone(&self.cwd));

        // Build new node
        let mut cwd_mut = Rc::make_mut(&mut self.cwd);
        cwd_mut.children.push(Rc::new(Node::new_dir(name, parent)));
    }

    pub fn stat_file(&mut self, name: &str, size: usize) {
        // Walk the children and insert new file if necessary
        for child in &self.cwd.children {
            if child.name == name {
                assert_eq!(child.size, size);
                return;
            }
        }

        // Build new node
        let parent = Some(Rc::clone(&self.cwd));
        let mut cwd_mut = Rc::make_mut(&mut self.cwd);
        cwd_mut
            .children
            .push(Rc::new(Node::new_file(name, size, parent)));
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
    use super::*;

    #[test]
    fn parse_fs() {
        let cmds = vec!["$ cd /", "$ ls", "dir a", "1234 b.txt", "dir d"];
        let fs = FileSystem::from_log(&cmds);

        assert!(fs.is_ok());
        let fs = fs.unwrap();

        assert_eq!(fs.root.name, "/");

        println!("root: {:?}", fs.root);

        assert_eq!(fs.root.children[0].name, "a");
        assert_eq!(fs.root.children[0].which, NodeType::Directory);
        assert_eq!(fs.root.children[0].name, "b.txt");
        assert_eq!(fs.root.children[0].which, NodeType::File);
        assert_eq!(fs.root.children[0].name, "d");
        assert_eq!(fs.root.children[0].which, NodeType::Directory);
    }

    #[test]
    fn parse_cmd() {
        assert_eq!(Cmd::parse("$ cd /"), Cmd::ChangeDir(String::from("/")));
        assert_eq!(Cmd::parse("$ ls"), Cmd::ListDir);
        assert_eq!(Cmd::parse("dir a"), Cmd::StatDir(String::from("a")));
        assert_eq!(Cmd::parse("1234 a"), Cmd::StatFile(1234, String::from("a")));
    }

    #[test]
    fn test_part1() {
        //let fs = FileSystem::from_path("input_test.txt");
    }
}
