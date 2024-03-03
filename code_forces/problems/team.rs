// @author: RenatoGM
fn main() {
    solve();
}
 
fn solve() -> () {
    let mut n: String = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("Failed to read the line.");
    let n: u16 = n.trim().parse().expect("NaN");
    let mut result: u32 = 0;
    for _ in 0..n {
        let mut nums: String = String::new();
        std::io::stdin()
            .read_line(&mut nums)
            .expect("Failed to read the line");
        let nums = nums.trim();
 
        let mut counter: u8 = 0;
 
        for c in nums.chars() {
            if c == '1' {
                counter += 1;
            }
        }
        if counter > 1 {
            result += 1;
        }
    }
    println!("{}", result);
}
