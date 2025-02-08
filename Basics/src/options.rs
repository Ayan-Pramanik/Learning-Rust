fn main(){
    let index = find_first_a(String::from("Ayan"));

    match index {
        Some(value) => println!("index is {}",value),
        None => println!("a not found"),
    }
}

fn find_first_a(s: String) -> Option<i32> {
    for (index, char) in s.chars().enumerate() {
        if char == 'a' || char == 'A' {
            return Some(index as i32);
        }
    }
    return None;
}