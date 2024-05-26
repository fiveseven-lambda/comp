mod token;
use crate::ast::{BinOp, Expr, Ty};
use enum_iterator::Sequence;
use token::{Lexer, Token};

pub fn parse(input: &str) -> Option<Expr> {
    let mut lexer = Lexer::new(input);
    let ret = parse_assign(&mut lexer);
    if let Some(token) = lexer.next_token {
        panic!("unexpected token {token:?}");
    }
    ret
}

fn parse_assign(lexer: &mut Lexer) -> Option<Expr> {
    let left_hand_side = parse_binary_operation(lexer)?;
    if let Some(Token::Equal) = lexer.next_token {
        lexer.consume_token();
        let right_hand_side = parse_assign(lexer).expect("empty right hand side");
        Some(Expr::Assign(
            Box::new(left_hand_side),
            Box::new(right_hand_side),
        ))
    } else {
        Some(left_hand_side)
    }
}

fn parse_binary_operation(lexer: &mut Lexer) -> Option<Expr> {
    parse_binary_operation_rec(lexer, Precedence::first())
}

fn parse_binary_operation_rec(lexer: &mut Lexer, precedence: Option<Precedence>) -> Option<Expr> {
    let Some(precedence) = precedence else {
        return parse_factor(lexer);
    };
    let mut left = parse_binary_operation_rec(lexer, precedence.next());
    while let Some(operator) = lexer
        .next_token
        .as_ref()
        .and_then(|token| binary_operator(token, precedence))
    {
        lexer.consume_token();
        let right = parse_binary_operation_rec(lexer, precedence.next());
        left = Some(Expr::Bin(
            Box::new(left.expect("empty left operand")),
            operator,
            Box::new(right.expect("empty right operand")),
        ));
    }
    left
}

#[derive(Clone, Copy, Sequence)]
enum Precedence {
    AddSub,
    MulDivRem,
}

fn binary_operator(token: &Token, precedence: Precedence) -> Option<BinOp> {
    match (token, precedence) {
        (Token::Asterisk, Precedence::MulDivRem) => Some(BinOp::Mul),
        (Token::Slash, Precedence::MulDivRem) => Some(BinOp::Div),
        (Token::Percent, Precedence::MulDivRem) => Some(BinOp::Rem),
        (Token::Plus, Precedence::AddSub) => Some(BinOp::Add),
        (Token::Hyphen, Precedence::AddSub) => Some(BinOp::Sub),
        _ => None,
    }
}

fn parse_factor(lexer: &mut Lexer) -> Option<Expr> {
    let mut expr = match lexer.next_token.as_mut() {
        Some(Token::Integer(value)) => {
            let value = value.parse().unwrap();
            lexer.consume_token();
            Expr::Int(value)
        }
        Some(Token::Identifier(name)) => {
            let name = std::mem::take(name);
            lexer.consume_token();
            Expr::Var(name)
        }
        Some(Token::OpeningParenthesis) => {
            lexer.consume_token();
            let expr = parse_binary_operation(lexer).expect("empty parentheses");
            assert!(matches!(lexer.next_token, Some(Token::ClosingParenthesis)));
            lexer.consume_token();
            expr
        }
        Some(Token::OpeningBracket) => {
            lexer.consume_token();
            let ty = parse_ty(lexer).expect("empty brackets");
            assert!(matches!(lexer.next_token, Some(Token::ClosingBracket)));
            lexer.consume_token();
            Expr::Id(ty)
        }
        _ => return None,
    };
    while let Some(Token::OpeningParenthesis) = lexer.next_token {
        lexer.consume_token();
        let mut args = Vec::new();
        loop {
            let arg = parse_binary_operation(lexer);
            if let Some(Token::Comma) = lexer.next_token {
                lexer.consume_token();
                args.push(arg.expect("empty argument"));
            } else {
                args.extend(arg);
                break;
            }
        }
        assert!(matches!(lexer.next_token, Some(Token::ClosingParenthesis)));
        lexer.consume_token();
        expr = Expr::Call(Box::new(expr), args);
    }
    Some(expr)
}

fn parse_ty(lexer: &mut Lexer) -> Option<Ty> {
    match lexer.next_token {
        Some(Token::Identifier(ref name)) => {
            let ret = match &name[..] {
                "Int" => Some(Ty::Int),
                _ => None,
            };
            lexer.consume_token();
            ret
        }
        Some(Token::OpeningParenthesis) => {
            lexer.consume_token();
            let mut args = Vec::new();
            loop {
                let arg = parse_ty(lexer);
                if let Some(Token::Comma) = lexer.next_token {
                    lexer.consume_token();
                    args.push(arg.expect("empty argument"));
                } else {
                    args.extend(arg);
                    break;
                }
            }
            assert!(matches!(lexer.next_token, Some(Token::ClosingParenthesis)));
            lexer.consume_token();
            let ret = parse_ty(lexer).expect("empty return type");
            Some(Ty::Func {
                args,
                ret: Box::new(ret),
            })
        }
        _ => None,
    }
}
