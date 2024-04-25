use std::ops::Range;

use chumsky::prelude::*;

use crate::expr::Expression;

#[derive(Debug)]
pub struct Rule {
    pub name: String,
    pub expr: Expression,
}

pub fn load_rules() -> std::io::Result<Vec<Rule>> {
    let content = std::fs::read_to_string("./rules/real.rule")?;

    Ok(rule_parser().then_ignore(end()).parse(content).unwrap())
}

pub type Error = chumsky::error::Simple<char, Range<usize>>;

fn rule_parser() -> impl Parser<char, Vec<Rule>, Error = Error> {
    recursive(|_| {
        text::keyword("rule")
            .ignored()
            .padded()
            .then(text::ident())
            .padded()
            .then_ignore(just("{"))
            .then(crate::parse::expression_parser())
            .padded()
            .then_ignore(just("}"))
            .padded()
            .map(|((_, ident), expr)| Rule { name: ident, expr })
            .repeated()
            .collect::<Vec<Rule>>()
    })
}
