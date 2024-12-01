use std::fs;
use tabled::Tabled;

pub trait Solver {
    fn solve_first(&self, lines: Vec<String>) -> String;
    fn solve_second(&self, lines: Vec<String>) -> String;
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

pub fn run(solver: &Box<dyn Solver>, day: i32, file: &str) -> Result {
    let lines = get_lines(&format!("files/{:0>2}_{}.txt", day, file));
    let solutions = get_solutions(&format!("files/{:0>2}_{}_solutions.txt", day, file));

    let solution_1 = solver.solve_first(lines.clone());
    let solution_2 = solver.solve_second(lines.clone());
    let mut solution_1_state = ResultState::Unknown;
    let mut solution_2_state = ResultState::Unknown;

    if solutions.is_some() {
        let solutions = solutions.unwrap();

        if solution_1 == solutions.0 {
            solution_1_state = ResultState::Correct;
        } else {
            solution_1_state = ResultState::Incorrect;
        }

        if solution_2 == solutions.1 {
            solution_2_state = ResultState::Correct;
        } else {
            solution_2_state = ResultState::Incorrect;
        }
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

pub fn get_lines(file_name: &str) -> Vec<String> {
    let mut contents = fs::read_to_string(file_name)
        .expect("Something went wrong reading the file");

    contents = contents.replace("\u{feff}", "");
    contents = contents.replace("\r\n", "\n");
    let lines = contents.split("\n");

    lines.map(|line| line.to_string()).collect()
}

pub fn get_solutions(file_name: &str) -> Option<(String, String)> {
    let contents = fs::read_to_string(file_name);

    if contents.is_err() {
        return None;
    }

    let mut contents = contents.unwrap();
    contents = contents.replace("\u{feff}", "");
    contents = contents.replace("\r\n", "\n");
    let lines = contents.split("\n").collect::<Vec<&str>>();

    Some((lines[0].to_string(), lines[1].to_string()))
}
