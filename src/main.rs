pub mod builder;
pub mod days;

fn main() {
    days::for_each(|day, one, two| {
        println!("[{}]:\n  [1]: {}\n  [2]: {}", day, one(), two());
    });
}
