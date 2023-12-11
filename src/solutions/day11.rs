pub fn day11_solution(input_data: String) -> (String, String) {

    //Grandest variable declaration ever
    let cosmos: Vec<&[u8]> = input_data.lines().map(|line| line.as_bytes()).collect();

    //Map the galaxies and record which rows and colums are empty
    let mut empty_rows = vec![1i64; cosmos.len()];
    let mut empty_cols = vec![1i64; cosmos[0].len()];
    let mut galaxy_list: Vec<Galaxy> = Vec::new();

    for i in 0 .. cosmos.len() {
        for j in 0 .. cosmos[0].len() {
            if cosmos[i][j] == b'#' {
                galaxy_list.push(Galaxy{row: i, col: j});
                empty_rows[i] = 0;
                empty_cols[j] = 0;
            }
        }
    }

    //Now, calculate the cumulative empty rows to figure out offsets
    let cumulative_rows = cumulative_totals(&empty_rows);
    let cumulative_cols = cumulative_totals(&empty_cols);

    let p1_result = compute_expansion(&cumulative_rows, &cumulative_cols, &galaxy_list, 1);
    let p2_result = compute_expansion(&cumulative_rows, &cumulative_cols, &galaxy_list, 999_999);
    
    return (p1_result.to_string(), p2_result.to_string());
}

//Expansion offsets are one less than the number of replaced lines being inserted (offset of 0 preserves size etc)
fn compute_expansion(cumulative_rows: &Vec<i64>, cumulative_cols: &Vec<i64>, galaxy_list: &Vec<Galaxy>, expansion_offset: i64) -> usize {
    
    let offset_galaxies: Vec<Galaxy> = galaxy_list.iter().map(
        |g|
            g.offset(cumulative_rows[g.row] * expansion_offset, cumulative_cols[g.col] * expansion_offset)
    ).collect();

    let mut result = 0;

    for i in 0 .. offset_galaxies.len() - 1 {
        for j in i + 1 .. offset_galaxies.len() {
            result += offset_galaxies[i].distance(&offset_galaxies[j]);
        }
    }

    return result;
}

fn cumulative_totals<T: Copy + std::ops::Add<Output = T>>(source: &Vec<T>) -> Vec<T> {

    let mut result: Vec<T> = Vec::with_capacity(source.len());

    result.push(source[0]);

    for i in 1 .. source.len() {
        result.push(result[i - 1] + source[i]);
    }

    return result;
}

struct Galaxy {
    row: usize,
    col: usize,
}

impl Galaxy {
    fn distance(&self, other: &Self) -> usize {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }

    fn offset(&self, row_offset: i64, col_offset: i64) -> Galaxy {
        Galaxy{
            row: (self.row as i64 + row_offset) as usize,
            col: (self.col as i64 + col_offset) as usize,
        }
    }
}