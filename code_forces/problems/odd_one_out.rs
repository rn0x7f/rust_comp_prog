// @author: RenatoGM
fn main() {
    let mut t: String = String::new();
    std::io::stdin().read_line(&mut t).unwrap();
    let t: u16 = t.trim().parse().unwrap();
    (0..t).for_each(|_| solve());
}
 
fn solve() {
    let mut numbers: Vec<u16> = Vec::new();
    let mut line: String = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    for num in line.split_whitespace() {
        if let Ok(num) = num.parse::<u16>(){
            numbers.push(num);
        }
    }
    println!("{}", {
        if numbers.get(0) == numbers.get(1) {
            numbers.get(2).unwrap()
        } else if numbers.get(0) == numbers.get(2) {
            numbers.get(1).unwrap()
        } else {
            numbers.get(0).unwrap()
        }
    });
}