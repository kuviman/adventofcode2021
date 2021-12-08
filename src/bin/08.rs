use std::{collections::HashMap, io::Read};

fn read(content: &str) -> Vec<(Vec<String>, Vec<String>)> {
    content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut tokens = line.split_whitespace();
            let mut digits = Vec::new();
            for _ in 0..10 {
                digits.push(tokens.next().unwrap().to_owned());
            }
            assert_eq!(tokens.next().unwrap(), "|");
            let output = tokens.map(|s| s.to_owned()).collect();
            (digits, output)
        })
        .collect()
}

fn solve(a: &[(Vec<String>, Vec<String>)]) -> usize {
    let mut result = 0;
    for (_digits, output) in a.to_vec() {
        for digit in output {
            if digit.len() == 2 || digit.len() == 3 || digit.len() == 4 || digit.len() == 7 {
                result += 1;
            }
        }
    }
    result
}

fn solve2(a: &[(Vec<String>, Vec<String>)]) -> usize {
    let mut result = 0;
    let normal_digits: Vec<Vec<char>> = [
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ]
    .into_iter()
    .map(|s| s.chars().collect::<Vec<char>>())
    .collect();
    let labels: Vec<char> = "abcdefg".chars().collect();
    let mut variants = HashMap::<Vec<Vec<char>>, Vec<Vec<char>>>::new();
    for permutation in itertools::Itertools::permutations(labels.iter().copied(), labels.len()) {
        let mut digits = normal_digits.clone();
        for digit in &mut digits {
            for x in digit.iter_mut() {
                let label_index = labels.iter().position(|label| x == label).unwrap();
                *x = permutation[label_index];
            }
            digit.sort();
        }
        let mut sorted = digits.clone();
        sorted.sort();
        variants.insert(sorted, digits);
    }
    for (digits, output) in a.to_vec() {
        let mut digits: Vec<Vec<char>> = digits
            .into_iter()
            .map(|s| {
                let mut s = s.chars().collect::<Vec<char>>();
                s.sort();
                s
            })
            .collect();
        digits.sort();
        let variant = &variants[&digits];
        let output: Vec<Vec<char>> = output
            .into_iter()
            .map(|s| {
                let mut s = s.chars().collect::<Vec<char>>();
                s.sort();
                s
            })
            .collect();
        let mut x = 0;
        for digit in &output {
            let digit = variant.iter().position(|s| s == digit).unwrap();
            x = x * 10 + digit;
        }
        result += x;
    }
    result
}

#[test]
fn test() {
    let input = "
    be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
    assert_eq!(solve(&read(input)), 26);
    assert_eq!(solve2(&read(input)), 61229);
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
