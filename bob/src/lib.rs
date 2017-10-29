// found elsewhere. adapted and fixed
pub fn reply(msg: &str) -> &str {
    let msg = msg.trim();

    // filter only alphabetic stuff
    let mut msg_2 = String::new();
    for c in msg.chars() {
        if c.is_alphabetic() {
            msg_2.push(c);
        }
    }

    // .all() returns true if you check 0 elements
    let is_all_upper = |msg: &str| {
        msg.chars().filter(|c| c.is_alphabetic()).all(|c| {
            c.is_uppercase()
        })
    };
    let is_question = |msg: &str| msg.chars().last() == Some('?');

    match msg {
        _ if msg.is_empty() => "Fine. Be that way!",
        _ if !msg_2.is_empty() && is_all_upper(&msg_2) => "Whoa, chill out!",
        _ if is_question(msg) => "Sure.",
        _ => "Whatever.",
    }
}
