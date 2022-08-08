#[derive(Debug, PartialEq)]
pub enum Token {
    INCREMENT,
    DECREMENT,
    BACKWARD,
    FORWARD,
    LOOP_START,
    LOOP_END,
    PRINT,
    READ,
}

pub fn tokenize (input: std::str::Chars) -> Result<Vec<Token>, String> {
    let mut tokens = vec![];

    for char in input {
        if let Some(token) = match char {
            '+' =>  Some(Token::INCREMENT),
            '-' =>  Some(Token::DECREMENT),
            '>' =>  Some(Token::FORWARD),
            '<' =>  Some(Token::BACKWARD),
            '[' =>  Some(Token::LOOP_START),
            ']' =>  Some(Token::LOOP_END),
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn correct_input () {
        assert_eq!(tokenize("+".chars()).unwrap(), [Token::INCREMENT]);
        assert_eq!(tokenize("-".chars()).unwrap(), [Token::DECREMENT]);
        assert_eq!(tokenize("<".chars()).unwrap(), [Token::BACKWARD]);
        assert_eq!(tokenize(">".chars()).unwrap(), [Token::FORWARD]);
        assert_eq!(tokenize("[".chars()).unwrap(), [Token::LOOP_START]);
        assert_eq!(tokenize("]".chars()).unwrap(), [Token::LOOP_END]);
        assert_eq!(tokenize(".".chars()).unwrap(), [Token::PRINT]);
        assert_eq!(tokenize(",".chars()).unwrap(), [Token::READ]);
        assert_eq!(tokenize("+-[]<>.,".chars()).unwrap(), [
            Token::INCREMENT,
            Token::DECREMENT,
            Token::LOOP_START,
            Token::LOOP_END,
            Token::BACKWARD,
            Token::FORWARD,
            Token::PRINT,
            Token::READ,
        ]);
    }

    #[test]
    fn ignore_whitespace () {
        assert_eq!(tokenize(" ".chars()).unwrap(), []);
        assert_eq!(tokenize("          ".chars()).unwrap(), []);
        assert_eq!(tokenize("\t".chars()).unwrap(), []);
        assert_eq!(tokenize("\r".chars()).unwrap(), []);
        assert_eq!(tokenize("\n".chars()).unwrap(), []);
        assert_eq!(tokenize(" \t\r\n".chars()).unwrap(), []);
        assert_eq!(tokenize(" + - [ ] < > . , ".chars()).unwrap(), [
            Token::INCREMENT,
            Token::DECREMENT,
            Token::LOOP_START,
            Token::LOOP_END,
            Token::BACKWARD,
            Token::FORWARD,
            Token::PRINT,
            Token::READ,
        ]);
    }

    #[test]
    fn incorrect_input () {
        assert!(tokenize("A".chars()).is_err());
        assert!(tokenize("B".chars()).is_err());
        assert!(tokenize("C".chars()).is_err());
        assert!(tokenize("D".chars()).is_err());
        assert!(tokenize("//@123".chars()).is_err());
    }
}
