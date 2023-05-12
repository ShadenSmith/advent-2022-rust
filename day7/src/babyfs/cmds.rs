
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Cmd {
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_cmd() {
        assert_eq!(Cmd::parse("$ cd /"), Cmd::ChangeDir(String::from("/")));
        assert_eq!(Cmd::parse("$ ls"), Cmd::ListDir);
        assert_eq!(Cmd::parse("dir a"), Cmd::StatDir(String::from("a")));
        assert_eq!(Cmd::parse("1234 a"), Cmd::StatFile(1234, String::from("a")));
    }
}