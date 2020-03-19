const MINE: char = '*';
const EMPTY: char = ' ';

struct MineField {
    grid: Vec<Vec<char>>,
}

impl MineField {
    fn new() -> Self {
        MineField { grid: vec![] }
    }

    pub fn plant_mines_in_row(&mut self, row: &str) {
        self.grid.push(row.chars().collect())
    }

    pub fn annotate(&mut self) {
        for (col, row) in self.enumerate_positions() {
            if self.is_mine(col, row) == 1 {
                continue;
            }
            let mines_around = self.compute_mines_around(col, row);
            if mines_around == 0 {
                continue;
            }
            match (
                self.grid
                    .get_mut(col)
                    .and_then(|cell_row| cell_row.get_mut(row)),
                std::char::from_digit(mines_around, 10),
            ) {
                (Some(cell), Some(digit)) => *cell = digit,
                _ => (),
            }
        }
    }

    pub fn get_annotated_rows(self) -> Vec<String> {
        self.grid.into_iter().fold(vec![], |mut acc, row| {
            acc.push(row.into_iter().collect());
            acc
        })
    }

    fn enumerate_positions(&self) -> Vec<(usize, usize)> {
        let mut positions = vec![];
        for column_idx in 0..self.grid.len() {
            for row_idx in 0..self.grid[column_idx].len() {
                positions.push((column_idx, row_idx));
            }
        }
        positions
    }

    fn compute_mines_around(&self, column_idx: usize, row_idx: usize) -> u32 {
        let mut found_mines = 0u32;
        let count_mines = |col, row| found_mines += self.is_mine(col, row);
        self.enumerate_around(column_idx, row_idx, count_mines);
        found_mines
    }

    fn enumerate_around<F>(&self, column_idx: usize, row_idx: usize, mut position: F)
    where
        F: FnMut(usize, usize),
    {
        let from_col = if column_idx > 0 { column_idx - 1 } else { 0 };
        let from_row = if row_idx > 0 { row_idx - 1 } else { 0 };
        for col in from_col..=(column_idx + 1) {
            for row in from_row..=(row_idx + 1) {
                if col == column_idx && row == row_idx {
                    continue;
                }
                position(col, row);
            }
        }
    }

    fn is_mine(&self, column_idx: usize, row_idx: usize) -> u32 {
        match self
            .grid
            .get(column_idx)
            .and_then(|cell_row| cell_row.get(row_idx))
        {
            Some(_cell @ &MINE) => 1,
            _ => 0,
        }
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut mine_field = MineField::new();
    for row in minefield {
        mine_field.plant_mines_in_row(row);
    }
    mine_field.annotate();
    mine_field.get_annotated_rows()
}
