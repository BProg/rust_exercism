pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut points: Vec<(usize, usize)> = vec![];
    let mut x = 0usize;
    let mut y = 0usize;
    let get_row = |idx: usize| { &input[idx] };
    let get_col = |idx: usize| { input.iter().map(|row| row[idx]).collect::<Vec<u64>>() };
    while y < input.len() {
        x = 0;
        while x < input[y].len() {
            let value_here = &input[y][x];
            let max_row = get_row(y).iter().max();
            let col = get_col(x);
            let min_column = col.iter().min();
            if let Some(max_row) = max_row {
                if let Some(min_column) = min_column {
                    if value_here == min_column && value_here == max_row {
                        points.push((y, x));
                    }
                }
            }
            x += 1;
        }
        y += 1;
    }
    points
}
