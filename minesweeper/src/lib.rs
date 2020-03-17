pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let is_mine_here = |x: usize, y: usize| {
        minefield[y as usize].chars().collect::<Vec<char>>()[x as usize] == '*'
    };

    let height = minefield.len() as i32;
    if height == 0 { return vec![]; }
    let width = minefield[0].len() as i32;
    let mut field_with_numbers: Vec<String> = vec![];
    for i in 0..height {
        let mut row_with_numbers = String::from("");
        for j in 0..width {
            let mut mines_around = 0u32;
            if is_mine_here(j as usize, i as usize) {
                row_with_numbers.push('*');
                continue;
            }
            for (x, y) in look_around((j as i32, i as i32)) {
                if y >= 0 && y < height && x >= 0 && x < width {
                    if is_mine_here(x as usize, y as usize) {
                        mines_around += 1;
                    }
                }
            }
            if mines_around == 0 {
                row_with_numbers.push(' ');
            } else {
                if let Some(ch) = std::char::from_digit(mines_around, 10) {
                    row_with_numbers.push(ch);
                } else {
                    row_with_numbers.push(' ');
                }
            }
        }
        field_with_numbers.push(row_with_numbers);
    }
    field_with_numbers
}

fn look_around((x, y): (i32, i32)) -> Vec<(i32, i32)> {
    return vec![
        (x - 1,  y - 1), // top left
        (x,      y - 1), // top
        (x + 1,  y - 1), // top right

        (x - 1,  y + 1), // bottom left
        (x,      y + 1), // bottom
        (x + 1,  y + 1), // bottom right

        (x - 1,  y), // left
        (x + 1,  y), // right
    ]
}

#[cfg(test)]
#[test]
fn test_look_around() {
    let positions = look_around((0, 0));
    assert_eq!(positions[0], (-1, -1));
    assert_eq!(positions[1], (0, -1));
    assert_eq!(positions[2], (1, -1));

    assert_eq!(positions[3], (-1, 1));
    assert_eq!(positions[4], (0, 1));
    assert_eq!(positions[5], (1, 1));

    assert_eq!(positions[6], (-1, 0));
    assert_eq!(positions[7], (1, 0));
}
