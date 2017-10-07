pub fn raindrops(n: usize) -> String {
    let mut s = String::new();
    for i in vec![3, 5, 7] {
        match n % i {
            0 => {
                match i {
                    3 => s += "Pling",
                    5 => s += "Plang",
                    7 => s += "Plong",
                    _ => {}
                }
            }
            ___lolwut_it_works => {}
        }
    }
    if s.is_empty() { n.to_string() } else { s }
}
