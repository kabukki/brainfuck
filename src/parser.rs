#[derive(Debug, PartialEq)]
pub enum Token {
    INCREMENT,
    DECREMENT,
    LEFT,
    RIGHT,
    LOOP,
    END,
    PRINT,
    READ,
}

// Tmp
pub type Ast = Vec<Token>;

pub fn tokenize (input: std::str::Chars) -> Result<Vec<Token>, String> {
    let mut tokens = vec![];

    for char in input {
        if let Some(token) = match char {
            '+' =>  Some(Token::INCREMENT),
            '-' =>  Some(Token::DECREMENT),
            '>' =>  Some(Token::RIGHT),
            '<' =>  Some(Token::LEFT),
            '[' =>  Some(Token::LOOP),
            ']' =>  Some(Token::END),
            '.' =>  Some(Token::PRINT),
            ',' =>  Some(Token::READ),
            ' ' | '\t' | '\r' | '\n' => None,
            _   =>  return Err(format!("Syntax error")),
        } {
            tokens.push(token);
        }
    }

    Ok(tokens)
}

pub fn parse (tokens: Vec<Token>) -> Result<Ast, String> {
    Ok(tokens)
}
