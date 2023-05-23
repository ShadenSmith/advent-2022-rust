mod cpu;

use cpu::ElfCPU;
use std::path::Path;

pub fn main() {
    let cpu = ElfCPU::parse_and_execute(Path::new("day10/input.txt")).unwrap();
    println!("Part 1: {}", cpu.part1());
}
