// @author: RenatoGM
fn main() {
    let mut t: String = String::new();
    std::io::stdin().read_line(&mut t).unwrap();
    let t: u8 = t.trim().parse().unwrap();
    (0..t).for_each(|_| solve());
}

fn solve() {
    if let Some(Ok(apartment)) = std::io::stdin().lines().next() {
        let apartment = apartment.trim();
        let size: usize = apartment.len();
        let n: u32 = apartment.chars().next().unwrap().to_digit(10).unwrap();
        let solution = (10 * n) - (10 - ((size * (size + 1)) / 2) as u32);
        println!("{}", solution);
    }
}