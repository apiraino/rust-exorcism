pub fn build_proverb(list: Vec<&str>) -> String {
    let mut result = String::new();
    if list.len() != 0 {
        let item_count = list.len();
        let mut counter = 0;
        while counter < item_count {
            let first_item = list.get(counter).unwrap();
            let second_item = list.get(counter + 1).unwrap_or(&"");
            if !second_item.is_empty() {
                result += &format!(
                    "For want of a {} the {} was lost.\n",
                    first_item,
                    second_item
                );
            }
            counter += 1;
        }
        result += &format!("And all for the want of a {}.", list.get(0).unwrap());
    }
    result
}
