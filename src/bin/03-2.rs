use std::io::Read;

fn read(content: &str) -> Vec<usize> {
    content
        .lines()
        .map(|line| usize::from_str_radix(line, 2).unwrap())
        .collect()
}

fn calc(a: &[usize], most: bool) -> usize {
    let mut a = a.to_vec();
    for bit in (0..usize::BITS).rev() {
        let zeros = a
            .iter()
            .map(|&x| (x >> bit) & 1)
            .filter(|&x| x == 0)
            .count();
        let ones = a.len() - zeros;
        let need_one =
            (most && ones >= zeros) || (!most && ones != 0 && (ones < zeros || zeros == 0));
        let need_bit = if need_one { 1 } else { 0 };
        a.retain(|&x| ((x >> bit) & 1) == need_bit);
    }
    a[0]
}

fn oxygen_generator_rating(a: &[usize]) -> usize {
    calc(a, true)
}

fn c02_scrubber_rating(a: &[usize]) -> usize {
    calc(a, false)
}

fn solve(a: &[usize]) -> u128 {
    oxygen_generator_rating(a) as u128 * c02_scrubber_rating(a) as u128
}

#[test]
fn test() {
    assert_eq!(
        solve(&[
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010
        ]),
        230
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
