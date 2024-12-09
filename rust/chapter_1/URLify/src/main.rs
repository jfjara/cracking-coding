fn main() {
    let result = urlify_text(&String::from("hello world    "), 11);
    println!("URLify: {}", result);
}

fn urlify_text(text: &String, size: u8) -> String{

    let space_encoded = vec!['%', '2', '0'];
    let mut result: Vec<char> = Vec::new();
    
    for i in 0..text.len() {
        if i == size as usize {
            break;
        }
        let character = text.chars().nth(i).unwrap();
        if character == ' ' {
                result.extend(&space_encoded);
        } else {
                result.push(character);
        }
    }
    return result.into_iter().collect();
}
