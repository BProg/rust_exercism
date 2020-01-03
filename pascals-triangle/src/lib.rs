pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        match self.row_count {
            0 => vec![],
            1 => vec![vec![1]],
            2 => vec![vec![1], vec![1, 1]],
            _ => (2..self.row_count).fold(vec![vec![1], vec![1, 1]], |mut acc, current| {
                let mut line: Vec<u32> = acc[current as usize - 1]
                    .windows(2)
                    .map(|ns| ns[0] + ns[1])
                    .collect();
                line.insert(0, 1);
                line.push(1);
                acc.push(line);
                acc
            }),
        }
    }
}
