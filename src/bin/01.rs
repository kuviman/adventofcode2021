use std::io::Read;

fn read(content: &str) -> Vec<i64> {
    content.lines().map(|line| line.parse().unwrap()).collect()
}

fn solve(depths: &[i64]) -> usize {
    let mut previous = None;
    let mut answer = 0;
    for &current in depths {
        if let Some(previous) = previous {
            if current > previous {
                answer += 1;
            }
        }
        previous = Some(current);
    }
    answer
}

#[test]
fn test() {
    assert_eq!(
        solve(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
        7,
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
