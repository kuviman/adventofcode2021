use std::io::Read;

fn read(content: &str) -> Vec<usize> {
    content
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

fn solve(a: &[usize]) -> usize {
    let mut a = a.to_vec();
    a.sort();
    let mut left = 0;
    let mut right = 0;
    for &x in &a {
        right += x - a[0];
    }
    let mut result = left + right;
    for (index, &_x) in a.iter().enumerate().skip(1) {
        left += index * (a[index] - a[index - 1]);
        right -= (a.len() - index) * (a[index] - a[index - 1]);
        result = result.min(left + right);
    }
    result
}

fn solve2(a: &[usize]) -> usize {
    let mut a = a.to_vec();
    a.sort();
    // println!("{:?}", a);
    let mut left = 0;
    let mut left_sum = 0;
    let mut right = 0;
    let mut right_sum = 0;
    for &x in &a {
        right += (x - a[0]) * (x - a[0] + 1) / 2;
        right_sum += x - a[0];
    }
    let mut result = left + right;
    let mut i = 0;
    for x in a[0] + 1..=*a.last().unwrap() {
        while a[i] < x {
            i += 1;
        }
        left_sum += i;
        left += left_sum;
        let j = a.len() - i;
        right -= right_sum;
        right_sum -= j;
        // println!("{}: {} + {} = {}", x, left, right, left + right);
        result = result.min(left + right);
    }
    result
}

#[test]
fn test() {
    assert_eq!(solve(&[16, 1, 2, 0, 4, 2, 7, 1, 2, 14]), 37);
    assert_eq!(solve2(&[16, 1, 2, 0, 4, 2, 7, 1, 2, 14]), 168);
}

fn main() {
    let mut content = String::new();
    std::io::stdin()
        .lock()
        .read_to_string(&mut content)
        .unwrap();
    println!("{}", solve(&read(&content)));
    println!("{}", solve2(&read(&content)));
}
