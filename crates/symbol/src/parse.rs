use std::{ops::Range, str::FromStr};

use chumsky::prelude::*;

use crate::expr::{Expression, Integer, Rational};

/// Error that occurred while trying to parse a character stream into an expression.
pub type Error = chumsky::error::Simple<char, Range<usize>>;

/// Reason why a parse error occurred.
// pub type ErrorReason = chumsky::error::SimpleReason<char, Range<usize>>;

/// Returns a parser that produces expressions from character streams.
///
/// The purpose of this function is to be a building block for parsers that parse
/// expressions as parts of a more complex input language. If you simply want
/// to turn strings into expressions, use `"a + b".parse::<Expression>()`.
#[allow(clippy::let_and_return)]
pub fn parser() -> impl Parser<char, Expression, Error = Error> {
    recursive(|expression| {
        let identifier = text::ident()
            // .map(|identifier: String| match identifier.as_str() {
            //     // "true" => Expression::Boolean(true),
            //     // "false" => Expression::Boolean(false),
            //     _ => Expression::Variable(identifier),
            // })
            .map(|ident: String| Expression::Variable(ident))
            .labelled("identifier")
            .boxed();

        let number = text::int(10)
            .chain(just('.').ignore_then(text::digits(10)).or_not())
            .map(|parts: Vec<String>| match parts.as_slice() {
                [integer] => Expression::Integer(integer.parse::<Integer>().unwrap()),
                [integer_part, fractional_part] => {
                    let numerator = format!("{}{}", integer_part, fractional_part);
                    let denominator = format!("1{}", "0".repeat(fractional_part.len()));
                    Expression::RationalNumber(Rational::new(
                        numerator.parse::<Integer>().unwrap(),
                        denominator.parse::<Integer>().unwrap(),
                    ))
                }
                _ => unreachable!(),
            })
            .labelled("number")
            .boxed();

        // let vector_or_matrix = expression
        //     .clone()
        //     .separated_by(just(','))
        //     .padded()
        //     .delimited_by(just('['), just(']'))
        //     .map(|elements| {
        //         if let Some(Expression::Vector(v)) = elements.first() {
        //             // If all elements of the vector are themselves vectors, and have the same size,
        //             // they are interpreted as the rows of a rectangular matrix.
        //             let row_size = v.len();
        //             let mut rows = Vec::new();

        //             for element in &elements {
        //                 match element {
        //                     Expression::Vector(v) if v.len() == row_size => {
        //                         rows.push(v.transpose());
        //                     }
        //                     _ => return Expression::Vector(Vector::from_vec(elements)),
        //                 }
        //             }

        //             Expression::Matrix(Matrix::from_rows(&rows))
        //         } else {
        //             Expression::Vector(Vector::from_vec(elements))
        //         }
        //     })
        //     .labelled("vector_or_matrix")
        //     .boxed();

        let atomic_expression = number
            .or(identifier)
            .or(expression.clone().delimited_by(just('('), just(')')))
            .padded()
            .boxed();

        // let function_or_element = atomic_expression
        //     .then(
        //         expression
        //             .clone()
        //             .separated_by(just(','))
        //             .padded()
        //             .delimited_by(just('('), just(')'))
        //             .map(|arguments| (Some(arguments), None))
        //             .or(expression
        //                 .clone()
        //                 .separated_by(just(','))
        //                 .at_least(1)
        //                 .at_most(2)
        //                 .delimited_by(just('['), just(']'))
        //                 .map(|indices| (None, Some(indices))))
        //             .or_not(),
        //     )
        //     .map(
        //         |(expression, arguments_or_indices)| match arguments_or_indices {
        //             Some((Some(arguments), None)) => fun(expression, arguments),
        //             Some((None, Some(indices))) => {
        //                 if indices.len() == 1 {
        //                     Expression::VectorElement(
        //                         Box::new(expression),
        //                         Box::new(indices[0].clone()),
        //                     )
        //                 } else {
        //                     Expression::MatrixElement(
        //                         Box::new(expression),
        //                         Box::new(indices[0].clone()),
        //                         Box::new(indices[1].clone()),
        //                     )
        //                 }
        //             }
        //             None => expression,
        //             _ => unreachable!(),
        //         },
        //     )
        //     .padded()
        //     .boxed();

        let power = atomic_expression
            .separated_by(just('^'))
            .at_least(1)
            .map(|expressions| {
                expressions
                    .into_iter()
                    .rev()
                    .reduce(|exponent, base| Expression::Power(Box::new(base), Box::new(exponent)))
                    .unwrap()
            })
            .labelled("power")
            .boxed();

        let negation = just('-')
            .ignore_then(power.clone())
            .map(|a| -a)
            .or(just('!').ignore_then(power.clone()).map(|a| !a))
            .labelled("negation")
            .or(power)
            .padded()
            .boxed();

        let product_or_quotient = negation
            .clone()
            .then(just('*').or(just('/')).then(negation).repeated())
            .foldl(|a, (operator, b)| match operator {
                '*' => a * b,
                '/' => a / b,
                _ => unreachable!(),
            })
            .labelled("product_or_quotient")
            .boxed();

        let sum_or_difference = product_or_quotient
            .clone()
            .then(just('+').or(just('-')).then(product_or_quotient).repeated())
            .foldl(|a, (operator, b)| match operator {
                '+' => a + b,
                '-' => a - b,
                _ => unreachable!(),
            })
            .labelled("sum_or_difference")
            .boxed();

        let comparison = sum_or_difference
            .clone()
            .then(
                just('=')
                    .to(vec!['='])
                    // .or(just('!').chain(just('=')))
                    .or(just('<').chain(just('=')))
                    .or(just('<').to(vec!['<']))
                    .or(just('>').chain(just('=')))
                    .or(just('>').to(vec!['>']))
                    .collect::<String>()
                    .then(sum_or_difference)
                    .repeated(),
            )
            .foldl(|lhs, (operator, rhs)| match operator.as_str() {
                "=" => Expression::Equality(Box::new(lhs), Box::new(rhs)),
                // "!=" => Expression::NotEqual(Box::new(lhs), Box::new(rhs)),
                "<" => Expression::LessThan(Box::new(lhs), Box::new(rhs)),
                "<=" => Expression::LessThanOrEqual(Box::new(lhs), Box::new(rhs)),
                ">" => Expression::GreatherThan(Box::new(lhs), Box::new(rhs)),
                ">=" => Expression::GreatherThanOrEqual(Box::new(lhs), Box::new(rhs)),
                _ => unreachable!(),
            })
            .labelled("comparison")
            .boxed();

        // let conjunction = comparison
        //     .clone()
        //     .then(
        //         just('&')
        //             .ignore_then(just('&'))
        //             .ignore_then(comparison)
        //             .repeated(),
        //     )
        //     .foldl(|a,b| Expression::And(Box::new(a.into()), Box::new(b.into())))
        //     .labelled("conjunction")
        //     .boxed();

        // let disjunction = conjunction
        //     .clone()
        //     .then(
        //         just('|')
        //             .ignore_then(just('|'))
        //             .ignore_then(conjunction)
        //             .repeated(),
        //     )
        //     .foldl(or)
        //     .labelled("disjunction")
        //     .boxed();

        comparison
    })
}

impl FromStr for Expression {
    type Err = Vec<Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        parser().then_ignore(end()).parse(string)
    }
}

// use logos::Logos;

// #[derive(Logos, Debug, PartialEq)]
// #[logos(skip r" \t\r\n")]
// pub enum Token {
//     #[token("+")]
//     Plus,
//     #[token("-")]
//     Minus,
//     #[token("*")]
//     Asterix,
//     #[token("/")]
//     Slash,
//     #[token("=")]
//     EqualSign,

//     #[regex("[0-9]+(.[0-9]+)?")]
//     Number,
// }

// pub fn parse(input: &str) -> Vec<Token> {
//     Token::lexer(input).map(|r| r.unwrap()).collect::<Vec<_>>()
// }
