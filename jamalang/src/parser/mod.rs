use ast::{parse_statement, Statement};
use pest::Parser;

pub mod ast;
pub mod expr;

#[derive(pest_derive::Parser)]
#[grammar = "./parser/grammar.pest"]
pub(crate) struct ModifierParser;

#[derive(Debug, Clone)]
pub struct ModifierFile {
    pub source_file: String,
    pub statements: Vec<Statement>,
}

pub fn parse_ast<'a>(
    source_file: &str,
    input: &'a str,
) -> Result<ModifierFile, pest::error::Error<Rule>> {
    let pairs = ModifierParser::parse(Rule::File, input)?;
    let mut statements = Vec::new();

    for pair in pairs
        .into_iter()
        .next()
        .expect("file should contain at last one statement")
        .into_inner()
    {
        match pair.as_rule() {
            Rule::Statement => {
                statements.push(parse_statement(pair));
            }
            Rule::EOI => (),
            _ => unreachable!("Unexpected rule: {:?}", pair.as_rule()),
        }
    }

    Ok(ModifierFile {
        source_file: source_file.to_string(),
        statements,
    })
}
