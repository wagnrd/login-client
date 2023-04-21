fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut id_token: String = String::new();
    let mut access_token: String = String::new();
    let mut is_invalid = true;

    if args.len() >= 5 {
        is_invalid = false;

        match get_token_type(&args[1]) {
            TokenType::IdToken => id_token = args[2].clone(),
            TokenType::AccessToken => access_token = args[2].clone(),
            TokenType::Invalid => is_invalid = true,
        }

        match get_token_type(&args[3]) {
            TokenType::IdToken => id_token = args[4].clone(),
            TokenType::AccessToken => access_token = args[4].clone(),
            TokenType::Invalid => is_invalid = true,
        }
    }

    match is_invalid {
        false => println!("id_token: {}\naccess_token: {}", id_token, access_token),
        true => println!("Invalid args: {}", args_to_string()),
    }
}

enum TokenType {
    IdToken,
    AccessToken,
    Invalid,
}

fn get_token_type(option: &String) -> TokenType {
    match option.as_str() {
        "--id_token" => TokenType::IdToken,
        "--access_token" => TokenType::AccessToken,
        _ => TokenType::Invalid
    }
}

fn args_to_string() -> String {
    let args: Vec<String> = std::env::args().collect();
    let mut result = String::new();

    for arg in args {
        result += format!("{arg} ").as_str();
    }

    result
}