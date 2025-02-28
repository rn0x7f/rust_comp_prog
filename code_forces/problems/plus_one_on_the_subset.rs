// @author: RenatoGM
fn main() {
    let mut t: String = String::new();
    std::io::stdin().read_line(&mut t).unwrap();
    let t: u16 = t.trim().parse::<u16>().unwrap();
    (0..t).for_each(|_| solve());
}

fn solve() {
    let mut n: String = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n: u8 = n.trim().parse::<u8>().unwrap();

    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut numeros: Vec<u32> = input
        .split_whitespace()
        .take(n.into())
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    numeros.sort();
    if let (Some(primero), Some(ultimo)) = (numeros.first(), numeros.last()) {
        println!("{}", ultimo - primero);
    }
}