use std::io::Read;

fn read(content: &str) -> Vec<i64> {
    content.lines().map(|line| line.parse().unwrap()).collect()
}

fn find_increases(values: impl IntoIterator<Item = i64>) -> usize {
    let mut previous = None;
    let mut answer = 0;
    for current in values {
        if let Some(previous) = previous {
            if current > previous {
                answer += 1;
            }
        }
        previous = Some(current);
    }
    answer
}

fn solve(values: &[i64]) -> usize {
    find_increases(values.windows(3).map(|window| window.iter().copied().sum()))
}

#[test]
fn test() {
    assert_eq!(
        solve(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
        5,
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
