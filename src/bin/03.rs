use std::io::Read;

fn read(content: &str) -> Vec<usize> {
    content
        .lines()
        .map(|line| usize::from_str_radix(line, 2).unwrap())
        .collect()
}

fn calc(a: &[usize], most: bool) -> usize {
    let mut result = 0;
    for bit in 0..usize::BITS {
        let zeros = a
            .iter()
            .map(|&x| (x >> bit) & 1)
            .filter(|&x| x == 0)
            .count();
        let ones = a.len() - zeros;
        assert!(ones != zeros);
        if (most && ones > zeros) || (!most && ones != 0 && ones < zeros) {
            result |= 1 << bit;
        }
    }
    result
}

fn gamma(a: &[usize]) -> usize {
    calc(a, true)
}

fn epsilon(a: &[usize]) -> usize {
    calc(a, false)
}

fn solve(a: &[usize]) -> u128 {
    gamma(a) as u128 * epsilon(a) as u128
}

#[test]
fn test() {
    assert_eq!(
        solve(&[
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010
        ]),
        198
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
