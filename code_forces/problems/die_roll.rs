// @author: RenatoGM
fn main() {
    let mut line: String = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let line = line.trim();

    println!("{}", {
        if let (Some(y), Some(w)) = (line.chars().next().and_then(|c| c.to_digit(10)), line.chars().last().and_then(|c| c.to_digit(10))){
            let maximum = std::cmp::max(w, y);
            let favorable = 6 - maximum + 1;
            let div = gcd(favorable, 6);
            let numerator = favorable / div;
            let denominator = 6 / div;
            format!("{}/{}", numerator, denominator)
        } else { "Saludos".to_string() }
    });
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}