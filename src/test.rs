use crate::{
    runtime::Interpreter,
    parser::*,
};

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

#[test]
fn interpret_writes_output () {
    let mut interpreter = Interpreter::new(vec![]);
    interpreter.run("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.".chars()).unwrap();

    assert_eq!((interpreter.out as Vec<u8>), b"A");
}
