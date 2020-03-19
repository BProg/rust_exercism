use minesweeper::annotate;

fn remove_annotations(board: &[&str]) -> Vec<String> {
    board.iter().map(|r| remove_annotations_in_row(r)).collect()
}

fn remove_annotations_in_row(row: &str) -> String {
    row.chars()
        .map(|ch| match ch {
            '*' => '*',
            _ => ' ',
        })
        .collect()
}

fn run_test(test_case: &[&str]) {
    let cleaned = remove_annotations(test_case);
    let cleaned_strs = cleaned.iter().map(|r| &r[..]).collect::<Vec<_>>();
    let expected = test_case.iter().map(|&r| r.to_string()).collect::<Vec<_>>();
    assert_eq!(expected, annotate(&cleaned_strs));
}

#[test]
#[ignore]
fn no_rows() {
    run_test(&[]);
}

#[test]
#[ignore]
fn no_columns() {
    run_test(&[""]);
}

#[test]
#[ignore]
fn no_mines() {
    run_test(&["   ", "   ", "   "]);
}

#[test]
#[ignore]
fn board_with_only_mines() {
    run_test(&["***", "***", "***"]);
}

#[test]
#[ignore]
fn mine_surrounded_by_spaces() {
    run_test(&["111", "1*1", "111"]);
}

#[test]
#[ignore]
fn space_surrounded_by_mines() {
    run_test(&["***", "*8*", "***"]);
}

#[test]
#[ignore]
fn horizontal_line() {
    run_test(&["1*2*1"]);
}

#[test]
// #[ignore]
fn horizontal_line_mines_at_edges() {
    run_test(&["*1 1*"]);
}

#[test]
#[ignore]
fn vertical_line() {
    run_test(&["1", "*", "2", "*", "1"]);
}

#[test]
#[ignore]
fn vertical_line_mines_at_edges() {
    run_test(&["*", "1", " ", "1", "*"]);
}

#[test]
#[ignore]
fn cross() {
    run_test(&[" 2*2 ", "25*52", "*****", "25*52", " 2*2 "]);
}

#[test]
#[ignore]
fn large_board() {
    run_test(&["1*22*1", "12*322", " 123*2", "112*4*", "1*22*2", "111111"]);
}
