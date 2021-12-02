use std::io::Read;

enum Command {
    Up(i64),
    Down(i64),
    Forward(i64),
}

fn read(content: &str) -> Vec<Command> {
    content
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            match parts.next().unwrap() {
                "up" => Command::Up(parts.next().unwrap().parse().unwrap()),
                "down" => Command::Down(parts.next().unwrap().parse().unwrap()),
                "forward" => Command::Forward(parts.next().unwrap().parse().unwrap()),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn solve(commands: &[Command]) -> i64 {
    let mut x = 0;
    let mut depth = 0;
    for command in commands {
        match *command {
            Command::Up(delta) => depth -= delta,
            Command::Down(delta) => depth += delta,
            Command::Forward(delta) => x += delta,
        }
    }
    x * depth
}

#[test]
fn test() {
    use Command::*;
    assert_eq!(
        solve(&[Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)]),
        150,
    );
}

fn main() {
    let mut content = String::new();
    std::io::stdin()
        .lock()
        .read_to_string(&mut content)
        .unwrap();
    println!("{}", solve(&read(&content)));
}
