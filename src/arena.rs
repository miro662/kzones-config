use serde::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct Arena {
    pub x: u8,
    pub y: u8,
    pub width: u8,
    pub height: u8,
}

impl Arena {
    pub fn full() -> Arena {
        Arena {
            x: 0,
            y: 0,
            width: 100,
            height: 100,
        }
    }

    pub fn slice<'a>(
        &'a self,
        divisions: &'a [f64],
        direction: Direction,
    ) -> impl Iterator<Item = Arena> + 'a {
        let size = match direction {
            Direction::Horizontal => self.width,
            Direction::Vertical => self.height,
        };

        let total: f64 = divisions.iter().sum();
        let mut sizes: Vec<u8> = divisions
            .iter()
            .map(move |d| size as f64 * (d / total))
            .map(|f: f64| f.floor() as u8)
            .collect();

        let rem = size - sizes.iter().sum::<u8>();
        for size in sizes.iter_mut().take(rem as usize) {
            *size += 1
        }

        let mut pos = match direction {
            Direction::Horizontal => self.x,
            Direction::Vertical => self.y,
        };

        sizes.into_iter().map(move |size| {
            pos += size;
            match direction {
                Direction::Horizontal => Arena {
                    x: pos - size,
                    y: self.y,
                    width: size,
                    height: self.height,
                },
                Direction::Vertical => Arena {
                    x: self.x,
                    y: pos - size,
                    width: self.width,
                    height: size,
                },
            }
        })
    }
}

impl Default for Arena {
    fn default() -> Self {
        Self::full()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Horizontal,
    Vertical,
}

#[cfg(test)]
mod tests {
    use super::{Arena, Direction};

    #[test]
    fn test_horizontal_half() {
        let expected = vec![
            Arena {
                x: 0,
                y: 0,
                width: 50,
                height: 100,
            },
            Arena {
                x: 50,
                y: 0,
                width: 50,
                height: 100,
            },
        ];
        let actual: Vec<_> = Arena::full()
            .slice(&[1.0, 1.0], Direction::Horizontal)
            .collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_vertical_three() {
        let expected = vec![
            Arena {
                x: 0,
                y: 0,
                width: 100,
                height: 34,
            },
            Arena {
                x: 0,
                y: 34,
                width: 100,
                height: 33,
            },
            Arena {
                x: 0,
                y: 67,
                width: 100,
                height: 33,
            },
        ];
        let actual: Vec<_> = Arena::full()
            .slice(&[1.0, 1.0, 1.0], Direction::Vertical)
            .collect();

        assert_eq!(expected, actual);
    }
}
