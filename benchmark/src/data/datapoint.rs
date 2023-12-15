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

    pub fn avg(points: Vec<DataPoint>) -> DataPoint {
        let mut avg = DataPoint::empty();
        let size = points.len();

        for point in points {
            avg.updated += point.updated;
            avg.solved += point.solved;
            avg.solve_time += point.solve_time;
            avg.solve_time_per += point.solve_time_per;
            avg.size += point.size;
            avg.nothing += point.nothing;
            avg.iterations += point.iterations;
            avg.generation_time += point.generation_time;
            avg.generation_time_per += point.generation_time_per;
            avg.error += point.error;
        }

        avg.updated /= size;
        avg.solved /= size;
        avg.solve_time /= size as u128;
        avg.solve_time_per /= size as u128;
        avg.size /= size;
        avg.nothing /= size;
        avg.iterations /= size;
        avg.generation_time /= size as u128;
        avg.generation_time_per /= size as u128;
        avg.error /= size;

        avg
    }
}
