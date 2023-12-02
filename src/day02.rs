pub fn solve(input: &str, part1: bool) -> String {
    let games = input.lines().map(Game::parse);
    if part1 {
        games
            .filter(|game| {
                game.is_possible_with(&GameState {
                    r: 12,
                    g: 13,
                    b: 14,
                })
            })
            .map(|game| game.id)
            .sum::<usize>()
            .to_string()
    } else {
        games
            .map(|game| game.min_needed_game_state().square())
            .sum::<u32>()
            .to_string()
    }
}

struct Game {
    states: Vec<GameState>,
    id: usize,
}

impl Game {
    fn parse(input: &str) -> Self {
        let (game, content) = input.split_once(':').unwrap();
        let states = content.split(';');
        let id = game.split_once(' ').unwrap().1;
        Self {
            states: states.map(GameState::parse).collect(),
            id: id.parse().unwrap(),
        }
    }

    fn is_possible_with(&self, available: &GameState) -> bool {
        self.states
            .iter()
            .all(|state| state.is_possible_with(available))
    }

    fn min_needed_game_state(&self) -> GameState {
        let mut min_state = *self.states.first().unwrap();
        for state in &self.states {
            if state.r > min_state.r {
                min_state.r = state.r;
            }
            if state.g > min_state.g {
                min_state.g = state.g;
            }
            if state.b > min_state.b {
                min_state.b = state.b;
            }
        }
        min_state
    }
}

#[derive(Clone, Copy)]
struct GameState {
    r: u32,
    g: u32,
    b: u32,
}

impl GameState {
    fn parse(input: &str) -> Self {
        let components = input.split(',');
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        for component in components {
            let mut parts = component.trim().split(' ');
            let count = parts.next().unwrap().parse().unwrap();
            let name = parts.next().unwrap();
            match name {
                "red" => r = count,
                "green" => g = count,
                "blue" => b = count,
                _ => unreachable!(),
            }
        }
        Self { r, g, b }
    }

    fn is_possible_with(&self, available: &GameState) -> bool {
        self.r <= available.r && self.g <= available.g && self.b <= available.b
    }

    fn square(&self) -> u32 {
        self.r * self.g * self.b
    }
}
