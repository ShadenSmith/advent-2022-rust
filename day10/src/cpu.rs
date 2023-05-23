
use std::fs::File;
use std::io::{BufRead,BufReader, Result};
use std::path::Path;

pub enum ElfInst {
    AddX(i64),
    NoOp,
}

impl ElfInst {
    pub fn parse(inp: &str) -> Result<Self> {
        let cmd_parts : Vec<&str> = inp.split(" ").collect();
        match cmd_parts[0] {
            "addx" => { 
                let val: i64 = cmd_parts[1].parse().unwrap();
                Ok(Self::AddX(val)) 
            },
            "noop" => { Ok(Self::NoOp)}
            _ => panic!("Could not parse {}", inp)
        } 
    }
}

type Register = i64;

pub struct ElfCPU {
    reg_x: Register,
    cycle_count: usize,
}

impl ElfCPU {

    pub fn new() -> Self {
        Self { reg_x: 1, cycle_count: 0 }
    }

    pub fn cycles(&self) -> usize {
        self.cycle_count
    }

    pub fn x(&self) -> Register {
        self.reg_x
    }

    pub fn set_x(&mut self, val: Register) -> Register {
        self.reg_x = val;
        self.x()
    }

    pub fn execute(&mut self, instruction: ElfInst) {

    }

    pub fn parse_and_execute(path: &Path) -> Result<Self> {

        let input_fd = File::open(path)?;

        let reader = BufReader::new(&input_fd);
        for line in reader.lines() {
            let inst = ElfInst::parse(&line.unwrap());
        }

        todo!();
    }

}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_cpu_init() {
        let cpu = ElfCPU::new();
        assert_eq!(cpu.cycles(), 0);
        assert_eq!(cpu.x(), 1);
    }

    #[test]
    fn test_cpu_exec_noop() {
        let mut cpu = ElfCPU::new();
        cpu.execute(ElfInst::NoOp);
        assert_eq!(cpu.cycles(), 1);
        assert_eq!(cpu.x(), 1);
    }

    #[test]
    fn test_cpu_exec_addx() {
        let mut cpu = ElfCPU::new();
        cpu.execute(ElfInst::AddX(3));
        assert_eq!(cpu.cycles(), 2);
        assert_eq!(cpu.x(), 4);
        cpu.execute(ElfInst::AddX(-1));
        assert_eq!(cpu.cycles(), 4);
        assert_eq!(cpu.x(), 3);
    }


    #[test]
    fn test_cpu_exec_addx_noop() {
        let mut cpu = ElfCPU::new();
        cpu.execute(ElfInst::AddX(3));
        assert_eq!(cpu.cycles(), 2);
        assert_eq!(cpu.x(), 4);
        cpu.execute(ElfInst::NoOp);
        assert_eq!(cpu.cycles(), 3);
        assert_eq!(cpu.x(), 4);
    }

    #[test]
    fn test_cpu_exec_file() {
        let mut cpu = ElfCPU::parse_and_execute(Path::new("test_inputs/basic.txt")).unwrap();
        assert_eq!(cpu.cycles(), 2);
        assert_eq!(cpu.x(), 4);
    }
}