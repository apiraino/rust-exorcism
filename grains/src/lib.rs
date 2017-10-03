pub fn square(s: u32) -> u64 {
    // When you don't know Rust
    // if (s < 1) || (s > 64) {
    //     panic!("Square must be between 1 and 64")
    // } else {
    //     let two = 2_f64;
    //     two.powi(s as i32 - 1) as u64 // return 2^s-1
    // }

    // more idiomatic
    match s {
        1...64 => {
            let two = 2_f64;
            // returns 2^s-1
            two.powi(s as i32 - 1) as u64
        }
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    let mut total = 0;
    for i in 1..65 {
        total += square(i);
    }
    total
}
