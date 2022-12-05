mod one;
mod three;
mod two;

pub fn for_each(callback: fn(usize, fn() -> i32, fn() -> i32)) {
    let days = vec![
        (1, one::one as fn() -> i32, one::two as fn() -> i32),
        (2, two::one as fn() -> i32, two::two as fn() -> i32),
        (3, three::one as fn() -> i32, three::two as fn() -> i32),
    ];

    for (day, one, two) in days.iter() {
        callback(*day, *one, *two)
    }
}
