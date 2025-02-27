//@author: RenatoGM
fn main() {
    let mut line: String = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut line_iter = line.split_whitespace();
    if let (Some(n), Some(m)) = (line_iter.next(), line_iter.next()) {
        let n: u8 = n.parse::<u8>().unwrap();
        let m: u8 = m.parse::<u8>().unwrap();
        if n == m {
            println!("NO");
            std::process::exit(0);
        }
        for num in n + 1..m {
            if is_prime(num) {
                println!("NO");
                std::process::exit(0);
            }
        }
        if is_prime(m) {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}

fn is_prime(n: u8) -> bool {
    if n <= 1 {
        return false;
    }
    let limit = (n as f64).sqrt() as u8;
    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }
    true
}