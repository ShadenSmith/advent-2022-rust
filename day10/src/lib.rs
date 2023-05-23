mod cpu;

#[cfg(test)]
mod tests {
    use crate::cpu::ElfCPU;

    use std::path::Path;

    use super::*;

    #[test]
    fn test_part1() {
        let cpu = ElfCPU::parse_and_execute(Path::new("input_test.txt")).unwrap();
        assert_eq!(cpu.part1(), 13140);
    }
}
