struct Token {
    token: String,
    context: String,
}

fn main() {
    let code = "i32 idade = 30;";
    let tokens = lexer(code);

    for token in tokens{
        println!("{}: {}", token.context, token.token);
    }
}



fn lexer(code: &str) -> Vec<Token> {

    //Transform the str into a vector
    let mut code: Vec<char> = code.chars().collect();
    code.push(' ');

    let mut tokens: Vec<Token> = vec![];
    let mut tk = String::new();

    //Tokenizing
    for c in code{
        //If word ends then push
        if c != ' ' {
            tk.push(c);
            continue;
        }

        tokens.push(Token{
            token: tk,
            context: "KEYWORD".to_string(),
        });
        tk = String::new();
    }
    tokens
}
