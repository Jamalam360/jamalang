use std::fs;

use enum_variant_type::EnumVariantType;

use crate::parser::{
    expr::{parse_expression, Expr as CExpr},
    parse_ast, ModifierFile, Rule,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AssignmentType {
    Set,
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Void,
    Number,
    Char,
    Bool,
    Array{ len: u32, kind: Box<Type> },
    Custom(String),
}

#[derive(Debug, Clone, EnumVariantType)]
pub enum Statement {
    #[evt(derive(Clone, Debug))]
    Assignment {
        identifier: String,
        index: Option<CExpr>,
        r#type: AssignmentType,
        value: CExpr,
    },
    #[evt(derive(Clone, Debug))]
    FunctionDefinition {
        lambda: bool,
        identifier: String,
        parameters: Vec<(String, Type)>,
        return_type_hint: Type,
        body: Vec<Statement>,
    },
    #[evt(derive(Clone, Debug))]
    Return { value: CExpr },
    #[evt(derive(Clone, Debug))]
    WhileLoop {
        condition: CExpr,
        body: Vec<Statement>,
    },
    #[evt(derive(Clone, Debug))]
    ForLoop {
        identifier: String,
        identifier_type_hint: Option<Type>,
        expression: CExpr,
        body: Vec<Statement>,
    },
    #[evt(derive(Clone, Debug))]
    IfStatement {
        condition: CExpr,
        body: Vec<Statement>,
        else_ifs: Vec<(CExpr, Vec<Statement>)>,
        else_body: Vec<Statement>,
    },
    #[evt(derive(Clone, Debug))]
    ForeignModule(ModifierFile),
    #[evt(derive(Clone, Debug))]
    Expr { expr: CExpr },
    #[evt(derive(Clone, Debug))]
    Comment(String),
}

pub(crate) fn parse_statement<'a>(pair: pest::iterators::Pair<'a, Rule>) -> Statement {
    let statement = pair
        .into_inner()
        .next()
        .expect("statement pair should have at least one inner pair");
    match statement.as_rule() {
        Rule::Assignment => {
            let mut identifier = None;
            let mut index = None;
            let mut r#type = None;
            let mut expr = None;

            for inner in statement.into_inner() {
                match inner.as_rule() {
                    Rule::Identifier => identifier = Some(inner.as_str().to_owned()),
                    Rule::ArrayIndexing => index = Some(parse_expression(inner.into_inner())),
                    Rule::Expr => {
                        expr = Some(parse_expression(inner.into_inner()));
                    }
                    _ => {
                        r#type = Some(match inner.as_rule() {
                            Rule::AssSet => AssignmentType::Set,
                            Rule::AssAdd => AssignmentType::Add,
                            Rule::AssSub => AssignmentType::Sub,
                            Rule::AssMul => AssignmentType::Mul,
                            Rule::AssDiv => AssignmentType::Div,
                            Rule::AssPow => AssignmentType::Pow,
                            _ => unreachable!("Unexpected rule {:#?}", inner.as_rule()),
                        });
                    }
                }
            }

            Statement::Assignment {
                identifier: identifier.expect("assignment requires an identifier"),
                index,
                r#type: r#type.expect("assignment requires a type"),
                value: expr.expect("assignment requires a value"),
            }
        }
        Rule::FunctionDefinition => {
            let mut lambda = false;
            let mut identifier = None;
            let mut return_type_hint = None;
            let mut parameters = Vec::new();
            let mut body = Vec::new();

            for inner_pair in statement.into_inner() {
                match inner_pair.as_rule() {
                    Rule::Lambda => lambda = true,
                    Rule::IdentifierDefinition => identifier = Some(inner_pair.as_str().to_owned()),
                    Rule::TypeHint => return_type_hint = Some(parse_type_hint(inner_pair.as_str())),
                    Rule::FunctionArgsDefinitionList => {
                        let mut parameter = (None, None);

                        for arg_pair in inner_pair.into_inner() {
                            match arg_pair.as_rule() {
                                Rule::IdentifierDefinition => {
                                    parameter.0 = Some(arg_pair.as_str());
                                }
                                Rule::TypeHint => {
                                    parameter.1 = Some(parse_type_hint(arg_pair.as_str()))
                                }
                                _ => unreachable!("Unexpected rule {:#?}", arg_pair.as_rule()),
                            }

                            if parameter.0.is_some() && parameter.1.is_some() {
                                parameters
                                    .push((parameter.0.unwrap().to_owned(), parameter.1.unwrap()));
                                parameter.0 = None;
                                parameter.1 = None;
                            }
                        }
                    }
                    Rule::Block => {
                        body.extend(inner_pair.into_inner().map(parse_statement));
                    }
                    _ => unreachable!("Unexpected rule {:#?}", inner_pair.as_rule()),
                }
            }

            Statement::FunctionDefinition {
                lambda,
                return_type_hint: return_type_hint.unwrap_or(Type::Void),
                identifier: identifier.expect("function definition requires an identifier"),
                parameters,
                body,
            }
        }
        Rule::IfBlock => {
            let mut condition = None;
            let mut body = Vec::new();
            let mut else_ifs = Vec::new();
            let mut else_body = Vec::new();

            for inner_pair in statement.into_inner() {
                match inner_pair.as_rule() {
                    Rule::IfStatement => {
                        let if_inner = inner_pair.into_inner();

                        for if_inner_pair in if_inner {
                            match if_inner_pair.as_rule() {
                                Rule::Expr => {
                                    condition = Some(parse_expression(if_inner_pair.into_inner()));
                                }
                                Rule::Block => {
                                    body.extend(if_inner_pair.into_inner().map(parse_statement));
                                }
                                _ => unreachable!("Unexpected rule {:#?}", if_inner_pair.as_rule()),
                            }
                        }
                    }
                    Rule::ElifStatement => {
                        let elif_inner = inner_pair.into_inner();
                        let mut elif_condition = None;
                        let mut elif_body = Vec::new();

                        for elif_inner_pair in elif_inner {
                            match elif_inner_pair.as_rule() {
                                Rule::Expr => {
                                    elif_condition =
                                        Some(parse_expression(elif_inner_pair.into_inner()));
                                }
                                Rule::Block => {
                                    elif_body
                                        .extend(elif_inner_pair.into_inner().map(parse_statement));
                                }
                                _ => {
                                    unreachable!("Unexpected rule {:#?}", elif_inner_pair.as_rule())
                                }
                            }
                        }

                        else_ifs.push((
                            elif_condition.expect("elif requires a condition"),
                            elif_body,
                        ));
                    }
                    Rule::ElseStatement => {
                        let else_inner = inner_pair.into_inner();

                        for else_inner_pair in else_inner {
                            match else_inner_pair.as_rule() {
                                Rule::Block => {
                                    else_body
                                        .extend(else_inner_pair.into_inner().map(parse_statement));
                                }
                                _ => {
                                    unreachable!("Unexpected rule {:#?}", else_inner_pair.as_rule())
                                }
                            }
                        }
                    }
                    _ => unreachable!("Unexpected rule {:#?}", inner_pair.as_rule()),
                }
            }

            Statement::IfStatement {
                condition: condition.expect("if statement requires a condition"),
                body,
                else_ifs,
                else_body,
            }
        }
        Rule::WhileStatement => {
            let mut condition = None;
            let mut body = Vec::new();

            for inner_pair in statement.into_inner() {
                match inner_pair.as_rule() {
                    Rule::Expr => {
                        condition = Some(parse_expression(inner_pair.into_inner()));
                    }
                    Rule::Block => {
                        body.extend(inner_pair.into_inner().map(parse_statement));
                    }
                    _ => unreachable!("Unexpected rule {:#?}", inner_pair.as_rule()),
                }
            }

            Statement::WhileLoop {
                condition: condition.expect("while statement requires a condition"),
                body,
            }
        }
        Rule::ForStatement => {
            let mut identifier = None;
            let mut type_hint = None;
            let mut expr = None;
            let mut body = Vec::new();

            for inner_pair in statement.into_inner() {
                match inner_pair.as_rule() {
                    Rule::Identifier => identifier = Some(inner_pair.as_str()),
                    Rule::TypeHint => type_hint = Some(parse_type_hint(inner_pair.as_str())),
                    Rule::Expr => {
                        expr = Some(parse_expression(inner_pair.into_inner()));
                    }
                    Rule::Block => {
                        body.extend(inner_pair.into_inner().map(parse_statement));
                    }
                    _ => unreachable!("Unexpected rule {:#?}", inner_pair.as_rule()),
                }
            }

            Statement::ForLoop {
                identifier: identifier
                    .expect("for statement requires an identifier")
                    .to_owned(),
                identifier_type_hint: type_hint,
                expression: expr.expect("for statement requires a value"),
                body,
            }
        }
        Rule::ImportStatement => {
            //TODO: Make paths relative to the file they are in
            let path = statement
                .into_inner()
                .next()
                .expect("import requires a path")
                .as_str()
                .to_owned();
            let file = fs::read_to_string(&path).expect("imported file should be available");
            Statement::ForeignModule(
                parse_ast(&path, &file).expect("imported file should have valid syntax"),
            )
        }
        Rule::Expr => Statement::Expr {
            expr: parse_expression(statement.into_inner()),
        },
        Rule::ReturnStatement => Statement::Return {
            value: parse_expression(statement.into_inner()),
        },
        Rule::Comment => Statement::Comment(statement.as_str().to_owned()),
        _ => unreachable!("Unexpected rule {:#?}", statement.as_rule()),
    }
}

fn parse_type_hint(mut hint: &str) -> Type {
    if hint.starts_with(':') {
        //TODO: Proper fix
        hint = hint.trim_start_matches(": ");
    }

    match hint {
        "number" => Type::Number,
        "bool" => Type::Bool,
        "void" => Type::Void,
        "char" => Type::Char,
        _ => {
            if hint.ends_with("]") {
                todo!()
                // let len = hint.split_at(mi)
                // Type::Array(Box::new(parse_type_hint(hint.trim_end_matches("]"))))
            } else {
                Type::Custom(hint.to_owned())
            }
        }
    }
}
