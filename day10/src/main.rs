
mod elf_arch;
use elf_arch::cpu::ElfCPU;

use std::path::Path;

pub fn main() {
    let cpu = ElfCPU::parse_and_execute(Path::new("day10/input.txt")).unwrap();
    println!("Part 1: {}", cpu.part1());
    println!("Part 2:");
    cpu.render_display();
}
