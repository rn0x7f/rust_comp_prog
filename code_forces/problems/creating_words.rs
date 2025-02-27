//@author: RenatoGM
fn main() {
    let mut t: String = String::new();
    std::io::stdin().read_line(&mut t).unwrap();
    let t: u8 = t.trim().parse::<u8>().unwrap();
    (0..t).for_each(|_| solve());
}

fn solve() {
    let mut words: String = String::new();
    std::io::stdin().read_line(&mut words).unwrap();
    let mut words_iter = words.split_whitespace();
    if let (Some(a), Some(b)) = (words_iter.next(), words_iter.next()){
        let mut a = a.to_string();
        let mut b = b.to_string();

        let a_first_char = a.chars().next().unwrap();
        let b_first_char = b.chars().next().unwrap();

        a.replace_range(0..1, &b_first_char.to_string());
        b.replace_range(0..1, &a_first_char.to_string());

        println!("{} {}", a, b);
    }
}