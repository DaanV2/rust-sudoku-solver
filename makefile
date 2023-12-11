
assembly-tool:
	cargo install cargo-show-asm

test:
	cargo test

build:
	cargo build

asm:
	cargo asm -p sudoku-solver-lib --release --rust --full-name --simplify --intel "sudoku_solver_lib::solvers::determined_solver::DeterminedSolver::solve"