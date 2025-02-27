// @author: RenatoGM
fn main() {
    let mut t: String = String::new();
    std::io::stdin().read_line(&mut t).unwrap();
    let t = t.trim().parse().unwrap();
    
    (0..t).for_each(|_| solve());
}

fn solve() {
    let mut n: String = String::new();
    if let Ok(_) = std::io::stdin().read_line(&mut n) {
        let n = n.trim();
        if let (Some(a), Some(b)) = (n.chars().next(), n.chars().last()) {
            if let (Some(a), Some(b)) = (a.to_digit(10), b.to_digit(10)) {
                let res = a + b;
                println!("{}", res);
            }
        }
    }
}