use crate::Token;

#[derive(Debug, PartialEq)]
pub enum Node {
    Increment,
    Decrement,
    Backward,
    Forward,
    Loop (Vec<Node>),
    Print,
    Read,
}

pub fn parse (tokens: &[Token]) -> Result<Vec<Node>, String> {
    fn inner (tokens: &[Token], offset: usize) -> (usize, Vec<Node>) {
        let mut ast: Vec<Node> = vec![];
        let mut n = offset;

        while n < tokens.len() {
            let token = &tokens[n];

            n += 1;

            match token {
                Token::Increment => ast.push(Node::Increment),
                Token::Decrement => ast.push(Node::Decrement),
                Token::Forward => ast.push(Node::Forward),
                Token::Backward => ast.push(Node::Backward),
                Token::LoopStart => {
                    let (read, children) = inner(tokens, n);
                    n += read;
                    ast.push(Node::Loop(children));
                },
                Token::LoopEnd => { break; },
                Token::Print => ast.push(Node::Print),
                Token::Read => ast.push(Node::Read),
            }
        }

        (n - offset, ast)
    }

    let (_, ast) = inner(tokens, 0);

    Ok(ast)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple () {
        assert_eq!(parse(&[Token::Increment]).unwrap(), [Node::Increment]);
        assert_eq!(parse(&[Token::Decrement]).unwrap(), [Node::Decrement]);
        assert_eq!(parse(&[Token::Backward]).unwrap(), [Node::Backward]);
        assert_eq!(parse(&[Token::Forward]).unwrap(), [Node::Forward]);
        assert_eq!(parse(&[Token::Print]).unwrap(), [Node::Print]);
        assert_eq!(parse(&[Token::Read]).unwrap(), [Node::Read]);
    }

    #[test]
    fn nested () {
        // [+]
        assert_eq!(
            parse(&[
                Token::LoopStart,
                    Token::Increment,
                Token::LoopEnd,
            ]).unwrap(),
            [
                Node::Loop(vec![
                    Node::Increment
                ])
            ],
        );
        // [[+]]
        assert_eq!(
            parse(&[
                Token::LoopStart,
                    Token::LoopStart,
                        Token::Increment,
                    Token::LoopEnd,
                Token::LoopEnd,
            ]).unwrap(),
            [
                Node::Loop(vec![
                    Node::Loop(vec![
                        Node::Increment
                    ])
                ])
            ],
        );
        // [[[+]]]
        assert_eq!(
            parse(&[
                Token::LoopStart,
                    Token::LoopStart,
                        Token::LoopStart,
                            Token::Increment,
                        Token::LoopEnd,
                    Token::LoopEnd,
                Token::LoopEnd,
            ]).unwrap(),
            [
                Node::Loop(vec![
                    Node::Loop(vec![
                        Node::Loop(vec![
                            Node::Increment
                        ])
                    ])
                ])
            ],
        );
    }
}