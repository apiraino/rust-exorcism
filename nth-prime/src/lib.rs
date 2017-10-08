#[derive(Debug, PartialEq)]
pub struct Error<T> {
    msg: T,
}

// brute force
fn __is_prime(n: i32) -> bool {
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

// AKS (?)
fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    if (n == 2) || (n == 3) {
        return true;
    }
    if (n % 2 == 0) || (n % 3 == 0) {
        return false;
    }

    let mut i = 5;
    let mut w = 2;
    while i < n {
        if n % i == 0 {
            return false;
        }
        i += w;
        w = 6 - w; // either 4 or 2
    }


    return true;
}

pub fn nth(n: i32) -> Result<i32, Error<String>> {
    match n {
        0 => Err(Error { msg: format!("zeroth prime") }),
        _ => {
            let mut count = 0;
            for x in 1.. {
                if is_prime(x) {
                    count += 1;
                }
                if count == n {
                    return Ok(x);
                }
            }
            Err(Error { msg: format!("did not find any prime for {}", n) })
        }
    }
}
