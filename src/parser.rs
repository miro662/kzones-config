use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::{
    arena::Direction,
    instruction::{Instruction, Node},
};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct SplitParser;

pub fn parse(input: &str) -> Instruction {
    let mut parse = SplitParser::parse(Rule::instruction, input).unwrap();
    let pair: pest::iterators::Pair<'_, Rule> = parse.next().unwrap();
    println!("{:#?}", pair);
    parse_instruction(pair)
}

fn parse_instruction(pair: Pair<Rule>) -> Instruction {
    let mut inner = pair.into_inner();
    let direction = match inner.next().map(|p| p.as_rule()) {
        Some(Rule::horizontal) => Direction::Horizontal,
        Some(Rule::vertical) => Direction::Vertical,
        _ => unreachable!("Expected horizontal or vertical direction"),
    };
    let children = inner.map(parse_node).collect();
    Instruction::Split {
        direction,
        children,
    }
}

fn parse_node(pair: Pair<Rule>) -> Node {
    let rule = pair.as_rule();
    let mut inner = pair.into_inner();
    let first_pair = inner.next().expect("Node always have at last one child");
    let ratio = first_pair
        .as_str()
        .parse()
        .expect("Rule allows only correct integers");
    match rule {
        Rule::leaf => Node {
            ratio,
            instruction: Instruction::Leaf,
        },
        Rule::split => {
            let instruction =
                parse_instruction(inner.next().expect("Split always have a second inner rule"));
            Node {
                ratio: 2.0,
                instruction,
            }
        }
        _ => unreachable!("Expected leaf or split"),
    }
}

#[cfg(test)]
mod tests {
    use crate::{arena::Direction, instruction::{Instruction, Node}};

    use super::parse;

    #[test]
    fn test_parse() {
        let actual = parse("h(1, 2: v(3, 4), 5)");
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