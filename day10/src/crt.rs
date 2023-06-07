use crate::cpu::ElfCPU;

pub struct ElfCRT {
    width: usize,
    height: usize,

    display: Vec<String>,
    cursor_row: usize,
    cursor_col: usize,

    cpu: ElfCPU,
}

impl ElfCRT {
    pub fn new() -> Self {
        let mut disp = vec![];
        for _row in 0..6 {
            let mut line = String::new();
            for _col in 0..40 {
                line.push('.');
            }
            disp.push(line);
        }
        ElfCRT {
            width: 40,
            height: 6,
            display: disp,
            cursor_row: 0,
            cursor_col: 0,
            cpu: ElfCPU::new(),
        }
    }

    pub fn draw_cycle(&mut self) {}
}

impl<'a> ElfCRT {
    pub fn get_display(&'a self) -> &'a Vec<String> {
        &self.display
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_crt_new() {
        let crt = ElfCRT::new();
        assert_eq!(crt.width, 40);
        assert_eq!(crt.height, 6);
    }
}
