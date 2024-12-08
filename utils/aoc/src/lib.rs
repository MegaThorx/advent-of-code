use std::fs;
use tabled::Tabled;

pub trait Solver {
    fn solve_first(&self, lines: Vec<String>) -> String;
    fn solve_second(&self, lines: Vec<String>) -> String;
}

pub trait SolverFactory {
    fn get(&self, day: i32) -> Option<Box<dyn Solver>>;
}

#[derive(Tabled)]
pub enum ResultState {
    Correct,
    Incorrect,
    Unknown,
}

#[derive(Tabled)]
pub struct Result {
    pub day: i32,
    pub file: String,
    pub solution_1: String,
    #[tabled(inline)]
    pub solution_1_state: ResultState,
    pub solution_2: String,
    #[tabled(inline)]
    pub solution_2_state: ResultState,
}

pub fn run_all(base_path: String, year: i32, solver_factory: Box<dyn SolverFactory>) -> Vec<Result> {
    let mut results: Vec<Result> = Vec::new();

    for day in 1..25 {
        let solver = solver_factory.get(day);

        if solver.is_none() {
            continue;
        }

        let solver = solver.unwrap();

        let result = run(&solver, day, year, &base_path, "example");
        results.push(result);
        let result = run(&solver, day, year, &base_path, "input");
        results.push(result);
    }

    results
}

pub fn run(solver: &Box<dyn Solver>, day: i32, year: i32, base_path: &str, file: &str) -> Result {
    let lines = get_lines(&format!("{}/{:0>4}/{:0>2}/{}.txt", base_path, year, day, file)).unwrap();
    let solutions = get_solutions(&format!("{}/{:0>4}/{:0>2}/{}_solutions.txt", base_path, year, day, file));

    let solution_1 = solver.solve_first(lines.clone());
    let solution_2 = solver.solve_second(lines.clone());
    let mut solution_1_state = ResultState::Unknown;
    let mut solution_2_state = ResultState::Unknown;

    if solutions.is_some() {
        let solutions = solutions.unwrap();

        solution_1_state = match solution_1 == solutions.0 {
            true => ResultState::Correct,
            false => ResultState::Incorrect,
        };

        solution_2_state = match solution_2 == solutions.1 {
            true => ResultState::Correct,
            false => ResultState::Incorrect,
        };
    }

    Result {
        day,
        file: file.to_string(),
        solution_1,
        solution_1_state,
        solution_2,
        solution_2_state,
    }
}

pub fn get_lines(file_name: &str) -> Option<Vec<String>> {
    let contents = fs::read_to_string(file_name);

    if contents.is_err() {
        return None;
    }

    let mut contents = contents.unwrap();
    contents = contents.replace("\u{feff}", "");
    contents = contents.replace("\r\n", "\n");
    let lines = contents.split("\n");

    Some(lines.map(|line| line.to_string()).collect())
}

pub fn get_solutions(file_name: &str) -> Option<(String, String)> {
    let lines = get_lines(file_name);

    if lines.is_none() {
        return None;
    }

    let lines = lines.unwrap();

    Some((lines[0].to_string(), lines[1].to_string()))
}
