#[derive(Debug)]
enum Balls {
    Red(usize),
    Blue(usize),
    Green(usize),
    None,
}

#[derive(Debug)]
struct Game {
    id: usize,
    balls: Vec<Vec<Balls>>,
}

#[derive(Copy, Clone)]
struct Rules {
    blue: usize,
    green: usize,
    red: usize,
}

fn parse_game(line: &str) -> Game {
    let game_split = line.split(": ").collect::<Vec<&str>>();
    let game_idx = game_split[0].split(" ").collect::<Vec<&str>>()[1];

    let sets_split = game_split[1].split("; ").collect::<Vec<&str>>();

    let balls = sets_split
        .iter()
        .map(|sub_line| parse_set(sub_line))
        .collect::<Vec<Vec<Balls>>>();

    let game = Game {
        id: game_idx.parse::<usize>().unwrap(),
        balls,
    };

    game
}

fn parse_set(sub_line: &str) -> Vec<Balls> {
    let line_split = sub_line.split(", ");

    let mut balls = vec![];

    for point in line_split {
        let point_split = point.split(" ").collect::<Vec<&str>>();
        let num_count = point_split[0].parse::<usize>().unwrap();

        let ball = match point_split[1] {
            "red" => Balls::Red(num_count),
            "blue" => Balls::Blue(num_count),
            "green" => Balls::Green(num_count),
            _ => Balls::None,
        };

        balls.push(ball)
    }

    balls
}

fn parse_lines(lines: &str) -> Vec<Game> {
    lines
        .lines()
        .map(|line| parse_game(line))
        .collect::<Vec<Game>>()
}

fn check_set(set: &Vec<Balls>, rules: Rules) -> bool {
    for ball in set {
        let ball_check = match *ball {
            Balls::Blue(b) => b <= rules.blue,
            Balls::Green(b) => b <= rules.green,
            Balls::Red(b) => b <= rules.red,
            _ => false,
        };

        if !ball_check {
            return false;
        }
    }

    true
}

fn check_game(g: &Game, rules: Rules) -> bool {
    g.balls.iter().map(|set| check_set(set, rules)).all(|ch| ch)
}

fn max_value_in_game(g: &Game) -> Vec<usize> {
    let mut max_red = 0;
    let mut max_blue = 0;
    let mut max_green = 0;

    for set in &g.balls {
        for ball in set {
            match ball {
                Balls::Red(b) => {
                    if b > &max_red {
                        max_red = *b
                    }
                }
                Balls::Blue(b) => {
                    if b > &max_blue {
                        max_blue = *b
                    }
                }
                Balls::Green(b) => {
                    if b > &max_green {
                        max_green = *b
                    }
                }
                _ => {}
            };
        }
    }

    vec![max_red, max_blue, max_green]
}

fn sum_maxes(lines: &str) -> usize {
    let games = parse_lines(lines);

    let mut res = 0;
    for game in games {
        let max_summs = max_value_in_game(&game);

        let mut partial_sum = 1;
        for num in max_summs {
            partial_sum *= num;
        }

        res += partial_sum;
    }

    res
}

fn sum_ids(lines: &str, rules: Rules) -> usize {
    let games = parse_lines(lines);
    let filtered_games = games.iter().filter(|g| check_game(*g, rules));

    filtered_games
        .map(|g| g.id)
        .reduce(|game_a, game_b| game_a + game_b)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn text_example_a() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

        // let res = parse_set("3 blue, 4 red");
        // let res = parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        let res = parse_lines(input);
        println!("{res:?}");

        let rules = Rules {
            red: 12,
            blue: 14,
            green: 13,
        };

        // let check = check_set(res, rules);
        // let check = check_game(res, rules);
        let check = sum_ids(input, rules);
        println!("{check}")
    }

    #[test]
    fn text_a() {
        let input = fs::read_to_string("./src/two/input_a.txt").unwrap();

        let rules = Rules {
            red: 12,
            blue: 14,
            green: 13,
        };

        let res = sum_ids(&input, rules);
        println!("{res}")
    }

    #[test]
    fn text_example_b() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

        let res = sum_maxes(input);
        println!("{res}");
    }

    #[test]
    fn text_b() {
        let input = fs::read_to_string("./src/two/input_b.txt").unwrap();

        let res = sum_maxes(&input);
        println!("{res}");
    }
}
