pub struct ElfCRT {
    width: i64,
    height: i64,

    display: Vec<String>,
    cursor_row: i64,
    cursor_col: i64,
}

impl ElfCRT {
    pub fn new() -> Self {
        let mut disp = vec![];
        for _row in 0..6 {
            let mut line = String::new();
            disp.push(line);
        }
        ElfCRT {
            width: 40,
            height: 6,
            display: disp,
            cursor_row: 0,
            cursor_col: 0,
        }
    }

    pub fn tick(&mut self, sprite_midpoint: i64) {
        let row_idx: usize = self.cursor_row as usize;
        let mut row: &mut String = &mut self.display[row_idx];

        // Does the 3-pixel sprite get rendered?
        let left = self.cursor_col - 1;
        let right = self.cursor_col + 1;

        if (left..=right).contains(&sprite_midpoint) {
            row.push('#');
        } else {
            row.push('.');
        }

        // Update cursor and optionally wrap around
        self.cursor_col += 1;
        if self.cursor_col == self.width {
            self.cursor_row += 1;
            self.cursor_col = 0;
        }
    }

    pub fn draw(&self) {
        for row in self.display.iter() {
            println!("{}", row);
        }
    }
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
