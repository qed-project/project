use std::{collections::HashMap, ops::Range};

use chumsky::prelude::*;

use crate::{expr::Expression, signature::Signature};

#[derive(Debug)]
pub struct Rule {
    pub name: String,
    pub expr: Expression,
}

impl Rule {
    pub fn apply(&self, expr: &Expression) -> Expression {
        let Expression::Equality(ref lhs, ref rhs) = self.expr else {
            panic!("right now we expect the rule to be an equality (<expr>=<expr>)")
        };

        

        todo!()
    }
}

pub fn load_rules() -> std::io::Result<Vec<Rule>> {
    let content = std::fs::read_to_string("./rules/real.rule")?;

    Ok(parser().then_ignore(end()).parse(content).unwrap())
}

pub fn load_rules_with_signatures() -> std::io::Result<HashMap<Signature, Rule>> {
    Ok(load_rules()?
        .into_iter()
        .map(|r| (Signature::from(&r.expr), r))
        .collect::<HashMap<Signature, Rule>>())
}

pub type Error = chumsky::error::Simple<char, Range<usize>>;

fn parser() -> impl Parser<char, Vec<Rule>, Error = Error> {
    recursive(|_| {
        let rule = text::keyword("rule")
            .ignored()
            .padded()
            .then(text::ident())
            .padded()
            .then_ignore(just("{"))
            .then(crate::parse::parser())
            .padded()
            .then_ignore(just("}"))
            .padded()
            .map(|((_, ident), expr)| Rule { name: ident, expr })
            .boxed();

        rule
            .repeated()
            .collect::<Vec<Rule>>()
    })
}

// impl FromStr for Vec<Rule> {
//     type Err = Vec<Error>;

//     fn from_str(string: &str) -> Result<Self, Self::Err> {
//         parser().then_ignore(end()).parse(string)
//     }
// }
