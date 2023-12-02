mod day01;
mod day02;

struct Solver {
    solve: fn(input: &str, part1: bool) -> String,
    day: u8,
}

struct SolverCollection {
    solvers: Vec<Solver>,
}

impl SolverCollection {
    fn new() -> Self {
        Self { solvers: vec![] }
    }

    fn add_solver(&mut self, day: u8, solver: fn(input: &str, part1: bool) -> String) {
        self.solvers.push(Solver { solve: solver, day })
    }

    fn run_latest(&self) {
        let Some(latest) = self.solvers.iter().max_by(|a, b| a.day.cmp(&b.day)) else {
            eprintln!("No days available");
            return;
        };

        let input = load_input(latest.day);
        let result = (latest.solve)(&input, true);
        println!("Part 1: {result}");
        let result2 = (latest.solve)(&input, false);
        println!("Part 2: {result2}");
    }
}

fn load_input(day: u8) -> String {
    let test_input = std::fs::read_to_string("test").unwrap();
    if !test_input.is_empty() {
        return test_input;
    }

    std::fs::read_to_string(format!("inputs/{day}")).unwrap()
}

fn main() {
    let mut solvers = SolverCollection::new();
    solvers.add_solver(1, day01::solve);
    solvers.add_solver(2, day02::solve);

    solvers.run_latest();
}
