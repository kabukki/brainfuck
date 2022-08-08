use crate::Token;

#[derive(Debug, PartialEq)]
pub enum Expression {
    INCREMENT,
    DECREMENT,
    BACKWARD,
    FORWARD,
    LOOP (Vec<Expression>),
    PRINT,
    READ,
}

impl std::fmt::Display for Expression {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Expression::LOOP(children) = self {
            writeln!(f, "loop {{")?;
            for child in children {
                writeln!(f, "    {}", child)?;
            }
            writeln!(f, "}}")
        } else {
            write!(f, "{:?}", self)
        }
    }
}

pub fn parse (tokens: &[Token]) -> Result<Vec<Expression>, String> {
    let mut ast: Vec<Expression> = vec![];
    let mut n = 0;

    while n < tokens.len() {
        match tokens[n] {
            Token::INCREMENT => ast.push(Expression::INCREMENT),
            Token::DECREMENT => ast.push(Expression::DECREMENT),
            Token::FORWARD => ast.push(Expression::FORWARD),
            Token::BACKWARD => ast.push(Expression::BACKWARD),
            Token::LOOP_START => {
                let children = parse(&tokens[n + 1..]).unwrap();
                n += children.len() + 1;
                ast.push(Expression::LOOP(children));
            },
            Token::LOOP_END => { break; },
            Token::PRINT => ast.push(Expression::PRINT),
            Token::READ => ast.push(Expression::READ),
        }
        n += 1;
    }

    Ok(ast)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple () {
        assert_eq!(parse(&[Token::INCREMENT]).unwrap(), &[Expression::INCREMENT]);
        assert_eq!(parse(&[Token::DECREMENT]).unwrap(), &[Expression::DECREMENT]);
        assert_eq!(parse(&[Token::BACKWARD]).unwrap(), &[Expression::BACKWARD]);
        assert_eq!(parse(&[Token::FORWARD]).unwrap(), &[Expression::FORWARD]);
        assert_eq!(parse(&[Token::PRINT]).unwrap(), &[Expression::PRINT]);
        assert_eq!(parse(&[Token::READ]).unwrap(), &[Expression::READ]);
    }

    #[test]
    fn nested () {
        assert_eq!(
            parse(&[Token::LOOP_START, Token::INCREMENT, Token::LOOP_END]).unwrap(),
            &[Expression::LOOP(vec![Expression::INCREMENT])],
        );
        assert_eq!(
            parse(&[Token::LOOP_START, Token::LOOP_START, Token::INCREMENT, Token::LOOP_END, Token::LOOP_END]).unwrap(),
            &[Expression::LOOP(vec![Expression::LOOP(vec![Expression::INCREMENT])])],
        );
    }
}