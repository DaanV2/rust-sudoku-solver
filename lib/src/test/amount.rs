#[cfg(test)]
mod test {
    use crate::{
        grid::constants::{GRID_HEIGHT, GRID_WIDTH},
        test::util::general_tests,
    };

    #[test]
    fn test_1() {
        test_amount(1)
    }

    #[test]
    fn test_9() {
        test_amount(9);
    }

    #[test]
    fn test_18() {
        test_amount(18);
    }

    #[test]
    fn test_27() {
        test_amount(27);
    }

    #[test]
    fn test_36() {
        test_amount(36);
    }

    #[test]
    fn test_45() {
        test_amount(45);
    }

    #[test]
    fn test_54() {
        test_amount(54);
    }

    #[test]
    fn test_63() {
        test_amount(63);
    }

    #[test]
    fn test_72() {
        test_amount(72);
    }

    fn test_amount(amount: usize) {
        let total = GRID_HEIGHT * GRID_WIDTH;
        let percent = (amount * 100) / total;

        println!("test_amount({}) {}%", amount, percent);
        let mut grid = general_tests::filled_sudoku();

        general_tests::remove_cells_amount(&mut grid, amount);

        general_tests::test_should_solve(grid);
    }
}
