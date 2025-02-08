struct User {
    first_name: String,
    last_name: String,
    age: i32,
}

fn main() {
    let name = String::from("ayanpramanik");
    let user = User{
        first_name: String::from("Ayan"),
        last_name: String::from("Pramanik"),
        age:32,
    };
    let len = get_str_len(name);
    println!("The length of the string is {}",len);
    println!("{}{} age is {}",user.first_name,user.last_name,user.age);
}

fn get_str_len(str: String) -> usize {
    str.chars().count()
} 