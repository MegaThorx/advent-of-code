use std::fs;

fn main() {
    println!("Example");
    run("./example.txt");
    println!();
    println!("Input");
    run("./input.txt");
}

fn run(file_name: &str) {
    let (numbers_left, numbers_right) = parse(file_name);

    let solution_1 = solve_1(&mut numbers_left.clone(), &mut numbers_right.clone());
    let solution_2 = solve_2(&mut numbers_left.clone(), &mut numbers_right.clone());

    println!("Solution 1: {}", solution_1);
    println!("Solution 2: {}", solution_2);
}

fn parse(file_name: &str) -> (Vec<i64>, Vec<i64>) {
    let lines = get_lines(file_name);

    let mut numbers_left: Vec<i64> = Vec::new();
    let mut numbers_right: Vec<i64> = Vec::new();

    for line in lines {
        let parts = line.split("   ").collect::<Vec<&str>>();

        let left = parts[0].parse::<i64>().expect("Unable to parse left number");
        let right = parts[1].parse::<i64>().expect("Unable to parse right number");

        numbers_left.push(left);
        numbers_right.push(right);
    }

    (numbers_left, numbers_right)
}

fn solve_1(numbers_left: &mut Vec<i64>, numbers_right: &mut Vec<i64>) -> i64 {
    numbers_left.sort();
    numbers_right.sort();

    let mut result: i64 = 0;

    for index in 0..numbers_left.len()
    {
        result += (numbers_left[index] - numbers_right[index]).abs();
    }

    result
}

fn solve_2(numbers_left: &mut Vec<i64>, numbers_right: &mut Vec<i64>) -> i64 {
    numbers_left.sort();
    numbers_right.sort();

    let mut result: i64 = 0;

    for index in 0..numbers_left.len() {
        let mut count = 0;

        for index_inner in 0..numbers_left.len() {
            if numbers_left[index] == numbers_right[index_inner] {
                count += 1;
            }
        }

        result += numbers_left[index] * count;
    }

    result
}


fn get_lines(file_name: &str) -> Vec<String> {
    let mut contents = fs::read_to_string(file_name)
        .expect("Something went wrong reading the file");

    contents = contents.replace("\u{feff}", "");
    contents = contents.replace("\r\n", "\n");
    let lines = contents.split("\n");

    lines.map(|line| line.to_string()).collect()
}