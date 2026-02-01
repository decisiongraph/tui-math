//! MathBox - A 2D character grid for math rendering

/// Represents a box of characters for rendering math expressions.
/// Uses a 2D grid with baseline tracking for proper vertical alignment.
#[derive(Clone, Debug)]
pub struct MathBox {
    content: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
    /// The baseline row (0-indexed from top)
    pub baseline: usize,
}

impl MathBox {
    /// Create a MathBox from a single-line string
    pub fn from_text(text: &str) -> Self {
        let chars: Vec<char> = text.chars().collect();
        let width = chars.len();
        Self {
            content: vec![chars],
            width,
            height: 1,
            baseline: 0,
        }
    }

    /// Create an empty MathBox with specified dimensions
    pub fn empty(width: usize, height: usize, baseline: usize) -> Self {
        Self {
            content: vec![vec![' '; width]; height],
            width,
            height,
            baseline,
        }
    }

    /// Create a MathBox from multiple lines
    pub fn from_lines(lines: Vec<String>, baseline: usize) -> Self {
        let height = lines.len();
        let width = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
        let mut content = vec![vec![' '; width]; height];

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if x < width {
                    content[y][x] = ch;
                }
            }
        }

        Self {
            content,
            width,
            height,
            baseline,
        }
    }

    /// Get character at position (returns space if out of bounds)
    pub fn get(&self, x: usize, y: usize) -> char {
        if y < self.height && x < self.width {
            self.content[y][x]
        } else {
            ' '
        }
    }

    /// Set character at position
    pub fn set(&mut self, x: usize, y: usize, ch: char) {
        if y < self.height && x < self.width {
            self.content[y][x] = ch;
        }
    }

    /// Copy another MathBox into this one at the specified offset
    pub fn blit(&mut self, other: &MathBox, x_offset: usize, y_offset: usize) {
        for y in 0..other.height {
            for x in 0..other.width {
                let target_x = x_offset + x;
                let target_y = y_offset + y;
                if target_y < self.height && target_x < self.width {
                    let ch = other.get(x, y);
                    if ch != ' ' {
                        self.set(target_x, target_y, ch);
                    }
                }
            }
        }
    }

    /// Concatenate horizontally, aligning by baseline
    pub fn concat_horizontal(boxes: &[MathBox]) -> MathBox {
        if boxes.is_empty() {
            return MathBox::empty(0, 1, 0);
        }

        // Find max ascent (baseline) and max descent (height - baseline - 1)
        let max_ascent = boxes.iter().map(|b| b.baseline).max().unwrap_or(0);
        let max_descent = boxes
            .iter()
            .map(|b| b.height.saturating_sub(b.baseline + 1))
            .max()
            .unwrap_or(0);

        let total_width: usize = boxes.iter().map(|b| b.width).sum();
        let total_height = max_ascent + 1 + max_descent;

        let mut result = MathBox::empty(total_width, total_height, max_ascent);
        let mut x_pos = 0;

        for b in boxes {
            let y_offset = max_ascent - b.baseline;
            result.blit(b, x_pos, y_offset);
            x_pos += b.width;
        }

        result
    }

    /// Stack vertically, centered horizontally
    pub fn stack_vertical(boxes: &[MathBox]) -> MathBox {
        if boxes.is_empty() {
            return MathBox::empty(0, 1, 0);
        }

        let max_width = boxes.iter().map(|b| b.width).max().unwrap_or(0);
        let total_height: usize = boxes.iter().map(|b| b.height).sum();

        let mut result = MathBox::empty(max_width, total_height, 0);
        let mut y_pos = 0;

        for b in boxes {
            let x_offset = (max_width - b.width) / 2;
            result.blit(b, x_offset, y_pos);
            y_pos += b.height;
        }

        // Baseline at middle
        result.baseline = total_height / 2;
        result
    }

    /// Fill a row with a character
    pub fn fill_row(&mut self, y: usize, ch: char) {
        if y < self.height {
            for x in 0..self.width {
                self.set(x, y, ch);
            }
        }
    }

    /// Fill a column with a character
    pub fn fill_col(&mut self, x: usize, ch: char) {
        if x < self.width {
            for y in 0..self.height {
                self.set(x, y, ch);
            }
        }
    }

    /// Convert to string representation
    pub fn to_string(&self) -> String {
        self.content
            .iter()
            .map(|row| row.iter().collect::<String>().trim_end().to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Get lines as vector of strings
    pub fn to_lines(&self) -> Vec<String> {
        self.content
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect()
    }
}

impl Default for MathBox {
    fn default() -> Self {
        Self::empty(0, 1, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_text() {
        let mb = MathBox::from_text("abc");
        assert_eq!(mb.width, 3);
        assert_eq!(mb.height, 1);
        assert_eq!(mb.get(0, 0), 'a');
        assert_eq!(mb.get(2, 0), 'c');
    }

    #[test]
    fn test_concat_horizontal() {
        let a = MathBox::from_text("x");
        let b = MathBox::from_text("+");
        let c = MathBox::from_text("y");
        let result = MathBox::concat_horizontal(&[a, b, c]);
        assert_eq!(result.to_string(), "x+y");
    }
}
