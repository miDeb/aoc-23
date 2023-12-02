pub fn solve(input: &str, part1: bool) -> String {
    if part1 {
        input.lines().map(parse_line_value).sum::<u32>().to_string()
    } else {
        input
            .lines()
            .map(parse_line_value2)
            .sum::<u32>()
            .to_string()
    }
}

fn parse_line_value(line: &str) -> u32 {
    let first_digit = line.chars().find(|c| c.is_ascii_digit()).unwrap();
    let last_digit = line.chars().rev().find(|c| c.is_ascii_digit()).unwrap();

    first_digit.to_digit(10).unwrap() * 10 + last_digit.to_digit(10).unwrap()
}

fn parse_line_value2(line: &str) -> u32 {
    let first = parse_first_or_last_digit(line, true);
    let last = parse_first_or_last_digit(line, false);

    first * 10 + last
}

fn parse_first_or_last_digit(line: &str, parse_first: bool) -> u32 {
    let c = if parse_first {
        line.chars().next().unwrap()
    } else {
        line.chars().next_back().unwrap()
    };

    if let Some(d) = c.to_digit(10) {
        return d;
    }

    let spelled_digits = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    for digit in spelled_digits {
        let is_match = if parse_first {
            line.starts_with(digit.0)
        } else {
            line.ends_with(digit.0)
        };
        if is_match {
            return digit.1;
        }
    }

    if parse_first {
        parse_first_or_last_digit(&line[1..], parse_first)
    } else {
        parse_first_or_last_digit(&line[..line.len() - 1], parse_first)
    }
}
