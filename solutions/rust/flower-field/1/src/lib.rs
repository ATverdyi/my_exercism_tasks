fn count_neighbors(matrix: &[Vec<u8>]) -> Vec<Vec<u8>> {
    if matrix.is_empty() {
        return Vec::new();
    }
    
    let rows = matrix.len() as isize;
    let cols = matrix[0].len() as isize;

    let mut result = vec![vec![0; cols as usize]; rows as usize];

    for r in 0..rows {
        for c in 0..cols {
            if matrix[r as usize][c as usize] == 1 {
                result[r as usize][c as usize] = 9;
                continue;
            }

            // counting
            let mut sum = 0;
            for dr in -1..=1 {
                for dc in -1..=1 {
                    if dr == 0 && dc == 0 {
                        continue;
                    }
                    let nr = r + dr;
                    let nc = c + dc;
                    if nr >= 0 && nr < rows && nc >= 0 && nc < cols {
                        sum += matrix[nr as usize][nc as usize];
                    }
                }
            }

            result[r as usize][c as usize] = sum;
        }
    }

    result
}

fn to_matrix(lines: &[&str]) -> Vec<Vec<u8>> {
    lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| if c == '*' { 1 } else { 0 })
                .collect()
        })
        .collect()
}

fn result_matrix(matrix: &[Vec<u8>]) -> Vec<String> {
    matrix
        .iter()
        .map(|row| {
            let mut line = String::with_capacity(row.len());
            for &v in row {
                line.push(match v {
                    9 => '*',
                    0 => ' ',
                    _ => char::from(b'0' + v),
                });
            }
            line
        })
        .collect()
}

pub fn annotate(garden: &[&str]) -> Vec<String> {
    let m = to_matrix(garden);
    let result = count_neighbors(&m);
    result_matrix(&result)
}
