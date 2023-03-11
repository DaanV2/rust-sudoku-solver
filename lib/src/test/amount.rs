#[cfg(test)]
mod test {
    use crate::test::util::general_tests;

    #[test]
    fn test_1() {
        test_amount(1)
    }

    #[test]
    fn test_10() {
        test_amount(10);
    }

    #[test]
    fn test_20() {
        test_amount(20);
    }

    #[test]
    fn test_30() {
        test_amount(30);
    }

    #[test]
    fn test_40() {
        test_amount(40);
    }

    #[test]
    fn test_50() {
        test_amount(50);
    }

    #[test]
    fn test_60() {
        test_amount(60);
    }

    fn test_amount(amount: usize) {
        println!("test_amount({})", amount);
        let mut grid = general_tests::filled_sudoku();

        general_tests::remove_cells_amount(&mut grid, amount);

        general_tests::test_should_solve(grid);
    }
}
