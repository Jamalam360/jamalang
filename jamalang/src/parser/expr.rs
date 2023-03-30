use crate::parser::{
    ast::{parse_statement, Statement},
    Rule,
};
use enum_variant_type::EnumVariantType;
use pest::pratt_parser::PrattParser;

#[derive(Debug, Clone, EnumVariantType)]
pub enum Expr {
    #[evt(derive(Clone, Copy, Debug, PartialEq))]
    Float(f32),
    #[evt(derive(Clone, Copy, Debug, PartialEq))]
    Bool(bool),
    #[evt(derive(Clone, Copy, Debug, PartialEq))]
    Char(char),
    #[evt(derive(Clone, Copy, Debug, PartialEq))]
    TypeNone,
    #[evt(derive(Clone, Debug, PartialEq))]
    Identifier(String),
    #[evt(derive(Clone, Debug))]
    FunctionCall {
        identifier: String,
        parameters: Vec<Expr>,
        lambda_body: Option<Vec<Statement>>,
    },
    #[evt(derive(Clone, Debug))]
    UnaryMinus(Box<Expr>),
    #[evt(derive(Clone, Debug))]
    Array(Vec<Expr>),
    #[evt(derive(Clone, Debug))]
    ArrayIndex { array: Box<Expr>, index: Box<Expr> },
    #[evt(derive(Clone, Debug))]
    BinOp {
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    Range,
    Lt,
    Gt,
    Lte,
    Gte,
    Eq,
    NotEq,
}

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<crate::parser::Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use crate::parser::Rule::*;

        PrattParser::new()
            .op(Op::infix(Add, Left) | Op::infix(Sub, Left))
            .op(Op::infix(Lt, Left) | Op::infix(Gt, Left) | Op::infix(Lte, Left) | Op::infix(Gte, Left) | Op::infix(Eq, Left) | Op::infix(NotEq, Left))
            .op(Op::infix(Mul, Left) | Op::infix(Div, Left) | Op::infix(Mod, Left) | Op::infix(Pow, Left) | Op::infix(Range, Left))
            .op(Op::prefix(UnaryMinus))
            .op(Op::postfix(ArrayIndexing))
    };
}

pub(crate) fn parse_expression<'a>(pairs: pest::iterators::Pairs<'a, Rule>) -> Expr {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::Float => Expr::Float(
                primary
                    .as_str()
                    .parse::<f32>()
                    .expect("value should be a valid number"),
            ),
            Rule::Bool => Expr::Bool(primary.as_str() == "true"),
            Rule::Char => Expr::Char(
                primary
                    .as_str()
                    .chars()
                    .nth(1)
                    .expect("character should be present"),
            ),
            Rule::Identifier => Expr::Identifier(primary.as_str().to_owned()),
            Rule::FunctionCall => {
                let mut parameters = Vec::new();
                let mut identifier = None;
                let mut lambda_body = None;

                for inner_pair in primary.into_inner() {
                    match inner_pair.as_rule() {
                        Rule::Identifier => {
                            identifier = Some(inner_pair.as_str().to_owned());
                        }
                        Rule::FunctionArgsList => {
                            parameters.extend(
                                inner_pair
                                    .into_inner()
                                    .map(|pair| parse_expression(pair.into_inner())),
                            );
                        }
                        Rule::Block => {
                            lambda_body =
                                Some(inner_pair.into_inner().map(parse_statement).collect());
                        }
                        _ => unreachable!("Unexpected rule {:#?}", inner_pair.as_rule()),
                    }
                }

                Expr::FunctionCall {
                    identifier: identifier.expect("function identifier should be present"),
                    parameters,
                    lambda_body,
                }
            }
            Rule::Array => {
                let mut elements = Vec::new();

                primary.into_inner().for_each(|inner_pair| {
                    elements.push(parse_expression(inner_pair.into_inner()));
                });

                Expr::Array(elements)
            }
            Rule::None => Expr::TypeNone,
            Rule::Expr => parse_expression(primary.into_inner()),
            rule => unreachable!("parse_expression expected atom, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::Add => Op::Add,
                Rule::Sub => Op::Subtract,
                Rule::Mul => Op::Multiply,
                Rule::Div => Op::Divide,
                Rule::Mod => Op::Modulo,
                Rule::Pow => Op::Power,
                Rule::Range => Op::Range,
                Rule::Lt => Op::Lt,
                Rule::Lte => Op::Lte,
                Rule::Gt => Op::Gt,
                Rule::Gte => Op::Gte,
                Rule::NotEq => Op::NotEq,
                Rule::Eq => Op::Eq,
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
            };
            Expr::BinOp {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::UnaryMinus => Expr::UnaryMinus(Box::new(rhs)),
            _ => unreachable!(),
        })
        .map_postfix(|lhs, op| match op.as_rule() {
            Rule::ArrayIndexing => Expr::ArrayIndex {
                array: Box::new(lhs),
                index: Box::new(parse_expression(op.into_inner())),
            },
            _ => unreachable!(),
        })
        .parse(pairs)
}
