// @author: RenatoGM
fn main() {
    let codeforces: &str = "codeforces";
    let mut t: String = String::new();
    std::io::stdin().read_line(&mut t).unwrap();
    let t: u16 = t.trim().parse::<u16>().unwrap();
    (0..t).for_each(|_| solve(&codeforces));
}

fn solve(codeforces: &str) {
    let mut word: String = String::new();
    std::io::stdin().read_line(&mut word).unwrap();
    let word = word.trim();
    let mut count: u8 = 0;
    for (i, c) in word.chars().enumerate() {
        if c != codeforces.chars().nth(i).unwrap() {
            count += 1;
        }
    }
    println!("{}", count);
}