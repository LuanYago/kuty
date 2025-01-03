use std::fs::File;
use std::io::{self, Read};

struct Token {
    token: String,
    context: String,
}

fn main() -> io::Result<()> {
    //le o arquivo
    let caminho = "code.ky";
    let mut arquivo = File::open(caminho)?;
    let mut conteudo = String::new();
    arquivo.read_to_string(&mut conteudo)?;

    let tokens = lexer(conteudo);

    for token in tokens{
        println!("{}: {}", token.context, token.token);
    }

    Ok(())
}


fn lexer(code: String) -> Vec<Token> {

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
