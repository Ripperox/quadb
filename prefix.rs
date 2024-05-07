fn longest_common_prefix(strs: Vec<&str>) -> String {
    if strs.is_empty() {
        return "".to_string();
    }
    let mut prefix = strs[0].to_string();

    for s in &strs[1..] {
        while s.starts_with(&prefix) == false {
            prefix.pop();
            if prefix.is_empty() {
                return "".to_string();
            }
        }
    }

    prefix
}

fn main() {
    let strings = vec!["flower", "flow", "flight"];
    println!("The longest common prefix is: {}", longest_common_prefix(strings));
}
