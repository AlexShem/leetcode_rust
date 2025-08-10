use super::Solve;

pub struct DecodeCiphertext;

impl DecodeCiphertext {
    pub fn decode_ciphertext(encoded_text: String, rows: i32) -> String {
        let rows = rows as usize;
        let encoded_len = encoded_text.len();
        let columns = encoded_len / rows;

        let mut grid = vec![vec![' '; columns]; rows];

        // Fill in the grid
        for (loc, ch) in encoded_text.chars().enumerate() {
            let (i, j) = (loc / columns, loc % columns);
            grid[i][j] = ch;
        }

        let mut decoded_text = String::new();
        // Traverse the grid
        for diag in 0..columns {
            let (mut current_row, mut current_column) = (0, diag);
            while current_row < rows && current_column < columns {
                decoded_text.push(grid[current_row][current_column]);
                current_row += 1;
                current_column += 1;
            }
        }

        decoded_text.trim_end().to_string()
    }
}

impl Solve<(String, i32), String> for DecodeCiphertext {
    fn solve(input: (String, i32)) -> String {
        Self::decode_ciphertext(input.0, input.1)
    }
}
