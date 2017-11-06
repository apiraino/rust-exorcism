use std::collections::HashMap;

fn get_bottles(n: i32) -> HashMap<&'static str, String> {
    let mut h = HashMap::new();
    // https://stackoverflow.com/a/38908324
    // let numbers = -1..100;
    for current in (0..100).rev() {
        // println!("prev={}, next={}", prev, next);
        let next = current - 1;

        if n == current {
            if current == 0 {
                h.insert("current", format!("No more bottles"));
            } else if current == 1 {
                h.insert("current", format!("{} bottle", current));
            } else {
                h.insert("current", format!("{} bottles", current));
            }

            if next == 0 {
                h.insert("next", format!("no more bottles"));
            } else if next == 1 {
                h.insert("next", format!("{} bottle", next));
            } else {
                h.insert("next", format!("{} bottles", next));
            }
            break;
        }
    }
    // match n {
    //     0 => {
    //         h.insert("current", format!("No more bottles"));
    //         h.insert("next", format!("{} bottles", 99));
    //     }
    //     1 => {
    //         h.insert("current", format!("{} bottle", 1));
    //         h.insert("next", format!("no more bottles"));
    //     }
    //     2 => {
    //         h.insert("current", format!("{} bottles", 2));
    //         h.insert("next", format!("{} bottle", 1));
    //     }
    //     _ => {
    //         h.insert("current", format!("{} bottles", n));
    //         h.insert("next", format!("{} bottles", n - 1));
    //     }
    // }
    h
}

pub fn verse(n: i32) -> String {
    let start = format!(
        "{} of beer on the wall, {} of beer.",
        get_bottles(n).get("current").unwrap(),
        get_bottles(n).get("current").unwrap().to_lowercase()
    );
    let end =
        format!(
        " {} of beer on the wall.",
        get_bottles(n).get("next").unwrap(),
    );
    let action;
    match n {
        0 => {
            action = format!("Go to the store and buy some more");
        }
        1 => {
            action = format!("Take it down and pass it around");
        }
        _ => {
            action = format!("Take one down and pass it around");
        }
    }
    return format!("{}\n{},{}\n", start, action, end);
}

pub fn sing(start: i32, end: i32) -> String {
    let mut s = String::new();
    for i in (end..start + 1).rev() {
        s.push_str(&verse(i));
        if i != end {
            s.push_str("\n");
        }
    }
    s
}
