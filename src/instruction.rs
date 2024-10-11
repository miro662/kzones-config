use std::collections::HashSet;

use crate::zone::{Direction, Zone};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Node {
    pub ratio: f64,
    pub instruction: Instruction,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Instruction {
    #[default]
    Leaf,
    Split {
        direction: Direction,
        children: Vec<Node>,
    },
}

impl Instruction {
    pub fn slice(&self, arena: Zone) -> HashSet<Zone> {
        let mut result = HashSet::new();
        self.slice_inner(arena, &mut result);
        result
    }

    fn slice_inner(&self, arena: Zone, result: &mut HashSet<Zone>) {
        match self {
            Instruction::Leaf => {
                result.insert(arena);
            }
            Instruction::Split {
                direction,
                children,
            } => {
                let divisions: Vec<f64> = children.iter().map(|c| c.ratio).collect();
                let child_areas = arena.slice(&divisions, *direction);
                for (node, child) in children.iter().zip(child_areas) {
                    node.instruction.slice_inner(child, result);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{
        instruction::Node,
        zone::{Direction, Zone},
    };

    use super::Instruction;

    #[test]
    fn test_leaf() {
        let instruction = Instruction::Leaf;
        let expected = HashSet::from([Zone::full()]);
        let actual = instruction.slice(Zone::full());

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_shallow_split() {
        let instruction = Instruction::Split {
            direction: Direction::Vertical,
            children: vec![
                Node {
                    ratio: 3.0,
                    instruction: Instruction::Leaf,
                },
                Node {
                    ratio: 1.0,
                    instruction: Instruction::Leaf,
                },
            ],
        };
        let expected = HashSet::from([
            Zone {
                x: 0,
                y: 0,
                width: 100,
                height: 75,
            },
            Zone {
                x: 0,
                y: 75,
                width: 100,
                height: 25,
            },
        ]);
        let actual = instruction.slice(Zone::full());

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_deep_split() {
        let instruction = Instruction::Split {
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
                                ratio: 1.0,
                                instruction: Instruction::Leaf,
                            },
                            Node {
                                ratio: 1.0,
                                instruction: Instruction::Leaf,
                            },
                        ],
                    },
                },
                Node {
                    ratio: 1.0,
                    instruction: Instruction::Leaf,
                },
            ],
        };
        let expected = HashSet::from([
            Zone {
                x: 0,
                y: 0,
                width: 25,
                height: 100,
            },
            Zone {
                x: 25,
                y: 50,
                width: 50,
                height: 50,
            },
            Zone {
                x: 25,
                y: 0,
                width: 50,
                height: 50,
            },
            Zone {
                x: 75,
                y: 0,
                width: 25,
                height: 100,
            },
        ]);
        let actual = instruction.slice(Zone::full());

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_three_split() {
        let instruction = Instruction::Split {
            direction: Direction::Horizontal,
            children: vec![
                Node {
                    ratio: 1.0,
                    instruction: Instruction::Leaf,
                },
                Node {
                    ratio: 1.0,
                    instruction: Instruction::Split {
                        direction: Direction::Vertical,
                        children: vec![
                            Node {
                                ratio: 1.0,
                                instruction: Instruction::Leaf,
                            },
                            Node {
                                ratio: 1.0,
                                instruction: Instruction::Leaf,
                            },
                        ],
                    },
                },
            ],
        };
        let expected = HashSet::from([
            Zone {
                x: 0,
                y: 0,
                width: 50,
                height: 100,
            },
            Zone {
                x: 50,
                y: 0,
                width: 50,
                height: 50,
            },
            Zone {
                x: 50,
                y: 50,
                width: 50,
                height: 50,
            },
        ]);
        let actual = instruction.slice(Zone::full());

        assert_eq!(expected, actual)
    }
}
