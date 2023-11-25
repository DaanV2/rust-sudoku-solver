#[cfg(test)]
mod test {
    use crate::test::util::general_tests;

    #[test]
    fn test_missing_number() {
        for i in 1..=9 {
            test_missing_specific_number(i);
        }
    }

    fn test_missing_specific_number(number: usize) {
        println!("Testing missing number {}", number);
        let mut grid = general_tests::filled_sudoku();

        //Reset all cells with nr 5 to empty
        general_tests::remove_number(&mut grid, number);

        general_tests::test_should_solve(grid);
    }
}
