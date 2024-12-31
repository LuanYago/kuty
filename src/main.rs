fn main() {
    let code = "i32 idade = 30;";
    let tokens = lexer(code);

    for i in tokens{
        println!("Token: {}",i);
    }
}

fn lexer(code: &str) -> Vec<String> {

    //Transform the chars into a vector
    let mut code: Vec<char> = code.chars().collect();
    code.push(' ');

    let mut tokens: Vec<String> = vec![];
    let mut token = String::new();

    //Iterating the chars
    for c in code{
        if c != ' ' {
            token.push(c);
            continue;
        }

        tokens.push(token);
        token = String::new();
    }
    tokens
}
