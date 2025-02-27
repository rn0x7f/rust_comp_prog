// @author: RenatoGM
fn main() {
    let mut t: String = String::new();
    std::io::stdin().read_line(&mut t).unwrap();
    let t: u8 = t.trim().parse::<u8>().unwrap();
    (0..t).for_each(|_| solve());
}

fn solve() {
    let mut line: String = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut line_iter = line.split_whitespace();
    if let (Some(a), Some(b)) = (line_iter.next().and_then(|number| number.parse::<u8>().ok()), line_iter.next().and_then(|number| number.parse::<u8>().ok())) {
        println!("{}", b - a);
    }
}