#[derive(Debug, PartialEq)]
pub enum Token {
    Increment,
    Decrement,
    Backward,
    Forward,
    LoopStart,
    LoopEnd,
    Print,
    Read,
}

pub fn tokenize (input: std::str::Chars) -> Result<Vec<Token>, String> {
    let mut tokens = vec![];

    for char in input {
        if let Some(token) = match char {
            '+' => Some(Token::Increment),
            '-' => Some(Token::Decrement),
            '>' => Some(Token::Forward),
            '<' => Some(Token::Backward),
            '[' => Some(Token::LoopStart),
            ']' => Some(Token::LoopEnd),
            '.' => Some(Token::Print),
            ',' => Some(Token::Read),
            ' ' | '\t' | '\r' | '\n' => None,
            _   => return Err(format!("Syntax error")),
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
        assert_eq!(tokenize("+".chars()).unwrap(), [Token::Increment]);
        assert_eq!(tokenize("-".chars()).unwrap(), [Token::Decrement]);
        assert_eq!(tokenize("<".chars()).unwrap(), [Token::Backward]);
        assert_eq!(tokenize(">".chars()).unwrap(), [Token::Forward]);
        assert_eq!(tokenize("[".chars()).unwrap(), [Token::LoopStart]);
        assert_eq!(tokenize("]".chars()).unwrap(), [Token::LoopEnd]);
        assert_eq!(tokenize(".".chars()).unwrap(), [Token::Print]);
        assert_eq!(tokenize(",".chars()).unwrap(), [Token::Read]);
        assert_eq!(tokenize("+-[]<>.,".chars()).unwrap(), [
            Token::Increment,
            Token::Decrement,
            Token::LoopStart,
            Token::LoopEnd,
            Token::Backward,
            Token::Forward,
            Token::Print,
            Token::Read,
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
            Token::Increment,
            Token::Decrement,
            Token::LoopStart,
            Token::LoopEnd,
            Token::Backward,
            Token::Forward,
            Token::Print,
            Token::Read,
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
