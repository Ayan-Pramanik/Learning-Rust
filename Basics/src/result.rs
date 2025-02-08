use std::fs::read_to_string;

fn main(){
    let result = read_from_file(file_path: String::from("a.txt"));
    println!("{}",result);
}

fn read_from_file(file_path: String) -> Result<String, String> {
    let result = read_to_string(file_path);

    match result {
        Ok(data) => Ok(data),
        Error(err) => Err(String::from("Error while reading the file")),
    }

}