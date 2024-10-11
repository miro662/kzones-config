use pest::{error::Error as PestError, iterators::Pair, Parser};
use pest_derive::Parser;
use snafu::prelude::*;

use crate::{
    instruction::{Instruction, Node},
    zone::Direction,
};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct LayoutParser;

pub fn parse(input: &str) -> ParserResult<Instruction> {
    let mut parse = LayoutParser::parse(Rule::instruction, input).context(PestSnafu)?;
    let pair: pest::iterators::Pair<'_, Rule> = parse.next().expect("At last one pair is expected");
    let result = parse_instruction(pair)?;
    match parse.next() {
        Some(_) => Err(ParserError::MoreThanOne),
        None => Ok(result),
    }
}

fn parse_instruction(pair: Pair<Rule>) -> ParserResult<Instruction> {
    let mut inner = pair.into_inner();
    let direction = match inner.next().map(|p| p.as_rule()) {
        Some(Rule::horizontal) => Direction::Horizontal,
        Some(Rule::vertical) => Direction::Vertical,
        _ => unreachable!("Expected horizontal or vertical direction"),
    };
    let children_result: Result<Vec<_>, ParserError> = inner.map(parse_node).collect();
    Ok(Instruction::Split {
        direction,
        children: children_result?,
    })
}

fn parse_node(pair: Pair<Rule>) -> ParserResult<Node> {
    let rule = pair.as_rule();
    let mut inner = pair.into_inner();
    let first_pair = inner.next().expect("Node always have at last one child");
    let ratio = first_pair
        .as_str()
        .parse()
        .expect("Rule allows only correct integers");
    let result = match rule {
        Rule::leaf => Node {
            ratio,
            instruction: Instruction::Leaf,
        },
        Rule::split => {
            let instruction =
                parse_instruction(inner.next().expect("Split always have a second inner rule"))?;
            Node { ratio, instruction }
        }
        _ => unreachable!("Expected leaf or split"),
    };
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::{
        instruction::{Instruction, Node},
        zone::Direction,
    };

    use super::parse;

    #[test]
    fn test_parse() {
        let actual = parse("h(1, 2: v(3, 4), 5)").unwrap();
        let expected = Instruction::Split {
            direction: Direction::Horizontal,
            children: vec![
                Node {
                    ratio: 1.0,
                    instruction: Instruction::Leaf,
                },
                Node {
                    ratio: 2.0,
                    instruction: Instruction::Split {
                        direction: Direction::Vertical,
                        children: vec![
                            Node {
                                ratio: 3.0,
                                instruction: Instruction::Leaf,
                            },
                            Node {
                                ratio: 4.0,
                                instruction: Instruction::Leaf,
                            },
                        ],
                    },
                },
                Node {
                    ratio: 5.0,
                    instruction: Instruction::Leaf,
                },
            ],
        };

        assert_eq!(expected, actual);
    }
}

pub type ParserResult<T> = Result<T, ParserError>;

#[derive(Debug, Snafu)]
pub enum ParserError {
    #[snafu(display("parser error:\n{source}"))]
    Pest { source: PestError<Rule> },
    #[snafu(display("more than one layout description"))]
    MoreThanOne,
}
