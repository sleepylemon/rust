mod solution;

fn main() {
    let request = "W55555RW555555W444444W1".to_string();
    let result = solution::Solution::walking_bot(request);
    match result {
        Some(r) => println!("{}", r),
        None => println!("NON"),
    }
}
