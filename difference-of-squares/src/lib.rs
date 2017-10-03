pub fn square_of_sum(n: usize) -> usize {
    let mut sum: i32 = 0;
    for i in 1..n + 1 {
        sum += i as i32;
    }
    sum.pow(2) as usize
}

pub fn sum_of_squares(n: usize) -> usize {
    let mut sum: usize = 0;
    for i in 1..n + 1 {
        sum += i.pow(2);
    }
    sum
}

pub fn difference(n: usize) -> usize {
    square_of_sum(n) - sum_of_squares(n)
}
