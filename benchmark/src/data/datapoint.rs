pub struct DataPoint {
    // Number of grids updated
    pub updated: usize,
    // Number of grids solved
    pub solved: usize,
    // Number of grids that had an error
    pub error: usize,
    // Number of grids that were not solved
    pub nothing: usize,
    // Total time to solve all grids in ns
    pub solve_time: u128,
    // Average time to solve a grid in ns
    pub solve_time_per: u128,
    // Total time to generate all grids in ns
    pub generation_time: u128,
    // Average time to generate a grid in ns
    pub generation_time_per: u128,
    // Number of grids in the entire test
    pub size: usize,
    // Number of total iterations
    pub iterations: usize,
}

impl DataPoint {
    pub fn empty() -> DataPoint {
        DataPoint {
            updated: 0,
            solved: 0,
            solve_time: 0,
            solve_time_per: 0,
            size: 0,
            nothing: 0,
            iterations: 0,
            generation_time: 0,
            generation_time_per: 0,
            error: 0,
        }
    }

    pub fn to_csv(&self) -> String {
        format!(
            "{},{},{},{},{},{},{},{},{},{}",
            self.size,
            self.generation_time,
            self.solve_time,
            self.generation_time_per,
            self.solve_time_per,
            self.iterations,
            self.solved,
            self.nothing,
            self.error,
            self.updated
        )
    }

    pub fn csv_headers() -> String {
        "size,generation_time,solve_time,generation_time_per,solve_time_per,iterations,solved,nothing,error,updated".to_string()
    }
}
