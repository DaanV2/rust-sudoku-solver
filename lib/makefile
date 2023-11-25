

assembly:
	cargo rustc --release -- --emit asm

assembly:
	cargo asm --build-type release --asm-style=intel --no-color --rust "<sudoku_solver_lib::solvers::mark_shapes::MarkShapes as sudoku_solver_lib::solvers::solver::Solver>::solve" > marshapes.asm