#[cfg(test)]
mod tests {
    use crate::{grid::utility::utility, test::util::general_tests};

    #[test]
    fn test_manual_specific_test1() {
        let grid = utility::parse_from_ascii(
            r#". 3 . | 2 6 . | . . .
               . . . | . 7 . | . 9 .
               1 . 7 | 8 3 4 | 5 . .
               ------|-------|------
               . 2 . | . . . | . . 7
               3 . 4 | 6 . 2 | . . 5
               9 5 1 | 7 4 3 | 6 . .
               ------|-------|------
               . 1 . | 3 . 6 | . . 4
               2 . 8 | . 5 . | . . .
               7 6 3 | 4 1 . | 2 5 ."#,
        );

        general_tests::test_should_solve(grid);
    }
}
