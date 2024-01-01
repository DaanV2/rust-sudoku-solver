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

    #[test]
    fn test_manual_specific_test2() {
        let grid = utility::parse_from_ascii(
            r#"4 3 5 | 2 6 9 | . . 1
               6 8 2 | . 7 . | 4 9 3
               1 9 7 | 8 3 4 | 5 6 2
               ------|-------|------
               8 2 6 | . 9 . | 3 4 7
               3 7 4 | 6 8 2 | 9 1 5
               9 5 1 | 7 4 3 | 6 2 8
               ------|-------|------
               5 1 9 | 3 2 6 | . . 4
               2 4 8 | 9 5 7 | 1 3 6
               7 6 3 | 4 1 8 | 2 5 9"#,
        );

        general_tests::test_should_solve(grid);
    }
}
