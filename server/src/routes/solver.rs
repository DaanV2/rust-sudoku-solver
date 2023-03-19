use actix_web::{get, post, web::Json, HttpResponse};
use sudoku_solver_lib::{
    grid::utility::utility::filled_sudoku,
    solvers::{
        solver::{AnnotatedSolverResult, SolveResult, SolverResult},
        solver_manager::SolverManager,
    },
};

use crate::data::grid::{GridInput, GridOutput};

#[post("/api/v1/solve")]
pub async fn solve(input: Json<GridInput>) -> HttpResponse {
    if input.is_valid() == false {
        return HttpResponse::BadRequest().body("body is not valid");
    }

    let grid = input.to_grid();
    let solver = SolverManager::new();
    let result = solver.solve(grid);
    let output = GridOutput::from_grid(result);

    HttpResponse::Ok()
        .content_type("application/json")
        .json(output)
}

#[post("/api/v1/solve/once")]
pub async fn solve_once(input: Json<GridInput>) -> HttpResponse {
    if input.is_valid() == false {
        return HttpResponse::BadRequest().body("body is not valid");
    }

    let grid = input.to_grid();
    let solver = SolverManager::new();
    let wrapped = SolverResult::nothing(grid);
    let result = solver.solve_round(wrapped);
    let annotated = AnnotatedSolverResult {
        result: result.result,
        grid: result.grid,
        iterations: 1,
    };
    let output = GridOutput::from_grid(annotated);

    HttpResponse::Ok()
        .content_type("application/json")
        .json(output)
}

#[get("/api/v1/filled")]
pub async fn filled() -> HttpResponse {
    let annotated = AnnotatedSolverResult {
        result: SolveResult::Nothing,
        grid: filled_sudoku(),
        iterations: 1,
    };
    let output = GridOutput::from_grid(annotated);

    HttpResponse::Ok()
        .content_type("application/json")
        .json(output)
}
