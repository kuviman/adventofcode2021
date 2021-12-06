use std::io::Read;

fn read(content: &str) -> Vec<usize> {
    content
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

fn solve(a: &[usize], days: usize) -> usize {
    let mut d = vec![vec![0; days + 1]; 9];
    for c in 0..=8 {
        d[c][0] = 1;
    }
    for days in 1..=days {
        for c in 0..=8 {
            d[c][days] = if c == 0 {
                d[6][days - 1] + d[8][days - 1]
            } else {
                d[c - 1][days - 1]
            };
        }
    }

    let mut result = 0;
    for &c in a {
        result += d[c][days];
    }
    result
}

#[test]
fn test() {
    assert_eq!(solve(&[3, 4, 3, 1, 2], 80), 5934);
    assert_eq!(solve(&[3, 4, 3, 1, 2], 256), 26984457539);
}

fn main() {
    let mut content = String::new();
    std::io::stdin()
        .lock()
        .read_to_string(&mut content)
        .unwrap();
    println!("{}", solve(&read(&content), 80));
    println!("{}", solve(&read(&content), 256));
}
