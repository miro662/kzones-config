use std::collections::HashSet;

use crate::arena::{Arena, Direction};

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
    pub fn slice(&self, arena: Arena) -> HashSet<Arena> {
        let mut result = HashSet::new();
        self.slice_inner(arena, &mut result);
        result
    }

    fn slice_inner(&self, arena: Arena, result: &mut HashSet<Arena>) {
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
        arena::{Arena, Direction},
        instruction::Node,
    };

    use super::Instruction;

    #[test]
    fn test_leaf() {
        let instruction = Instruction::Leaf;
        let expected = HashSet::from([Arena::full()]);
        let actual = instruction.slice(Arena::full());

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
            Arena {
                x: 0,
                y: 0,
                width: 100,
                height: 75,
            },
            Arena {
                x: 0,
                y: 75,
                width: 100,
                height: 25,
            },
        ]);
        let actual = instruction.slice(Arena::full());

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
            Arena {
                x: 0,
                y: 0,
                width: 25,
                height: 100,
            },
            Arena {
                x: 25,
                y: 50,
                width: 50,
                height: 50,
            },
            Arena {
                x: 25,
                y: 0,
                width: 50,
                height: 50,
            },
            Arena {
                x: 75,
                y: 0,
                width: 25,
                height: 100,
            },
        ]);
        let actual = instruction.slice(Arena::full());

        assert_eq!(expected, actual)
    }
}
