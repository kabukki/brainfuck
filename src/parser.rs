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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tokenize_correct_input () {
        assert_eq!(tokenize("+".chars()).unwrap(), [Token::INCREMENT]);
        assert_eq!(tokenize("-".chars()).unwrap(), [Token::DECREMENT]);
        assert_eq!(tokenize("<".chars()).unwrap(), [Token::LEFT]);
        assert_eq!(tokenize(">".chars()).unwrap(), [Token::RIGHT]);
        assert_eq!(tokenize("[".chars()).unwrap(), [Token::LOOP]);
        assert_eq!(tokenize("]".chars()).unwrap(), [Token::END]);
        assert_eq!(tokenize(".".chars()).unwrap(), [Token::PRINT]);
        assert_eq!(tokenize(",".chars()).unwrap(), [Token::READ]);
        assert_eq!(tokenize("+-[]<>.,".chars()).unwrap(), [
            Token::INCREMENT,
            Token::DECREMENT,
            Token::LOOP,
            Token::END,
            Token::LEFT,
            Token::RIGHT,
            Token::PRINT,
            Token::READ,
        ]);
    }

    #[test]
    fn tokenize_ignore_whitespace () {
        assert_eq!(tokenize(" ".chars()).unwrap(), []);
        assert_eq!(tokenize("          ".chars()).unwrap(), []);
        assert_eq!(tokenize("\t".chars()).unwrap(), []);
        assert_eq!(tokenize("\r".chars()).unwrap(), []);
        assert_eq!(tokenize("\n".chars()).unwrap(), []);
        assert_eq!(tokenize(" \t\r\n".chars()).unwrap(), []);
        assert_eq!(tokenize(" + - [ ] < > . , ".chars()).unwrap(), [
            Token::INCREMENT,
            Token::DECREMENT,
            Token::LOOP,
            Token::END,
            Token::LEFT,
            Token::RIGHT,
            Token::PRINT,
            Token::READ,
        ]);
    }

    #[test]
    fn tokenize_incorrect_input () {
        assert!(tokenize("A".chars()).is_err());
        assert!(tokenize("B".chars()).is_err());
        assert!(tokenize("C".chars()).is_err());
        assert!(tokenize("D".chars()).is_err());
        assert!(tokenize("//@123".chars()).is_err());
    }
}