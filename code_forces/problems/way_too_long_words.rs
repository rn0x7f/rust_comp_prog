// @author: RenatoGM
fn main() {
    let mut n: String = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("Failed to read the line.");
    let mut n: u8 = n.trim().parse().expect("NaN");
    n += 1;
    while {
        n -= 1;
        n > 0
    } {
        solve();
    }
}
 
fn solve() {
    let mut word: String = String::new();
    std::io::stdin()
        .read_line(&mut word)
        .expect("Failed to read the line.");
    let word = word.trim();
 
    if word.len() > 10 {
        let wi: char = word.chars().next().unwrap_or_default();
        let wf: char = word.chars().last().unwrap_or_default();
        let wlen: usize = word.len() - 2;
        println!("{}{}{}", wi, wlen, wf);
    } else {
        println!("{}", word);
    }
}
