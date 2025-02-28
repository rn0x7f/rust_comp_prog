// @author: RenatoGM
use std::collections::HashSet;
fn main() {
    let mut t: String = String::new();
    std::io::stdin().read_line(&mut t).unwrap();
    let t: u16 = t.trim().parse::<u16>().unwrap();
    (0..t).for_each(|_| solve());
}

fn solve() {
    let mut _dummy = String::new();
    std::io::stdin().read_line(&mut _dummy).unwrap();
    let mut tasks = String::new();
    std::io::stdin().read_line(&mut tasks).unwrap();
    let tasks = tasks.trim();
    
    let mut seen: HashSet<char> = HashSet::new();
    let mut current_task: Option<char> = None;
    
    for c in tasks.chars() {
        if Some(c) != current_task {
            if seen.contains(&c) {
                println!("NO");
                return;
            }
            if let Some(prev) = current_task {
                seen.insert(prev);
            }
            current_task = Some(c);
        }
    }
    println!("YES");
}