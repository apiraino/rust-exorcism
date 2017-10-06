#[derive(Debug, PartialEq)]
pub struct Error;

fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    for y in (2..n).rev() {
        if n % y == 0 {
            return false;
        }
    }
    return true;
}

pub fn nth(n: i32) -> Result<i32, Error> {
    match n {
        0 => Err(Error),
        _ => {
            let mut count = 0;
            for x in 0.. {
                if is_prime(x) == true {
                    count += 1;
                }
                if count == n {
                    return Ok(x);
                }
            }
            Ok(count)
        }
    }
}
