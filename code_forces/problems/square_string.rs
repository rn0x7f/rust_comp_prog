// @author: RenatoGM
use std::convert::TryInto;

fn main() {
    let mut t: String = String::new();
    std::io::stdin().read_line(&mut t).unwrap();
    let t: u8 = t.trim().parse::<u8>().unwrap();
    (0..t).for_each(|_| solve());
}

fn solve() {
    if let Some(Ok(input)) = std::io::stdin().lines().next() {
        let input_size: u8 = input.len().try_into().unwrap_or(255);
        if input_size % 2 != 0 {
            println!("NO");
        } else {
            let a: &str = &input[0..(input_size / 2) as usize];
            let b: &str = &input[(input_size / 2) as usize..input_size as usize];
            if a == b {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}