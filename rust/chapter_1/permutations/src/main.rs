fn main() {
    println!("Result is {}", is_permutation_by_order(&String::from("annaaona"), &String::from("nnaaanaa")));
}

fn is_permutation_by_order(source: &String, target: &String) -> bool {
    if source.len() != target.len() {
        return false;
    }

    let mut source_vec: Vec<char> = source.chars().collect();
    let mut target_vec: Vec<char> = target.chars().collect();

    source_vec.sort();
    target_vec.sort();

    for i in 0..source.len() {
        if source_vec[i] != target_vec[i] {
            return false;
        }
    }
    return true;
}

fn is_permutation(source: &String, target: &String) -> bool {
    
    if source.len() != target.len() {
        return false;
    }

    let mut source_vec: Vec<char> = source.chars().collect();
    let mut target_vec: Vec<char> = target.chars().collect();

    while !source_vec.is_empty() {
        let source_char = source_vec[0];
        let mut index = 0;
        let mut char_target_found = false;
        while index < target_vec.len() && !char_target_found {
            let target_char = target_vec[index];
            if target_char == source_char {
                char_target_found = true;
                target_vec.remove(index);
            }
            index += 1;
        }
        source_vec.remove(0);
    }

    if source_vec.len() == 0 && target_vec.len() == 0 {
        return true;
    }
    false
}
