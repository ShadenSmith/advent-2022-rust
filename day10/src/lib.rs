pub mod elf_arch;

#[cfg(test)]
mod tests {
    use elf_arch::cpu::ElfCPU;

    use std::path::Path;

    use super::*;

    #[test]
    fn test_part1() {
        let cpu = ElfCPU::parse_and_execute(Path::new("input_test.txt")).unwrap();
        assert_eq!(cpu.part1(), 13140);
    }

    #[test]
    fn test_part2() {
        let gold = vec![
            "##..##..##..##..##..##..##..##..##..##..",
            "###...###...###...###...###...###...###.",
            "####....####....####....####....####....",
            "#####.....#####.....#####.....#####.....",
            "######......######......######......####",
            "#######.......#######.......#######.....",
        ];

        let cpu = ElfCPU::parse_and_execute(Path::new("input_test.txt")).unwrap();

        for idx in 0..cpu.get_display().len() {
            assert_eq!(cpu.get_display()[idx], gold[idx]);
        }
    }
}
