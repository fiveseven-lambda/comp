use crate::ast::{Expr, Stmt};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1, multispace0},
    combinator::map,
    multi::{many0, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair},
    IResult,
};

pub fn parse_stmt(input: &str) -> IResult<&str, Stmt> {
    alt((
        map(tag("#"), |_| Stmt::Comment),
        map(
            separated_pair(alphanumeric1, preceded(multispace0, tag("=")), parse_expr),
            |(name, expr)| Stmt::Decl(name.to_string(), expr),
        ),
        map(parse_expr, |expr| Stmt::Expr(expr)),
    ))(input)
}

fn parse_expr(input: &str) -> IResult<&str, Expr> {
    let parser = alt((
        map(digit1, |s: &str| Expr::Int(s.parse().unwrap())),
        map(alphanumeric1, |s: &str| Expr::Id(s.to_string())),
        delimited(tag("("), parse_expr, tag(")")),
    ));
    let parser = delimited(multispace0, parser, multispace0);
    let parser = map(
        pair(
            parser,
            many0(delimited(
                tag("("),
                separated_list0(tag(","), parse_expr),
                tag(")"),
            )),
        ),
        |(func, argss)| {
            argss.into_iter().fold(func, |acc, args| Expr::Call {
                func: acc.into(),
                args,
                need_convert: None,
            })
        },
    );
    let parser = delimited(multispace0, parser, multispace0);
    let parser = map(separated_list1(tag("*"), parser), |list| {
        list.into_iter()
            .reduce(|acc, e| Expr::Call {
                func: Box::new(Expr::Id("mul".to_string())),
                args: vec![acc, e],
                need_convert: None,
            })
            .unwrap()
    });
    let parser = delimited(multispace0, parser, multispace0);
    let parser = map(separated_list1(tag("+"), parser), |list| {
        list.into_iter()
            .reduce(|acc, e| Expr::Call {
                func: Box::new(Expr::Id("add".to_string())),
                args: vec![acc, e],
                need_convert: None,
            })
            .unwrap()
    });
    delimited(multispace0, parser, multispace0)(input)
}
