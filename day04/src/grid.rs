use std::fmt::Display;

use aoc_common::Point;

#[derive(PartialEq, Clone)]
pub struct GridCell<T> {
    pub position: Point<usize>,
    pub value: T,
}

#[derive(PartialEq, Clone)]
pub struct Grid<T> {
    pub height: usize,
    pub width: usize,
    values: Vec<Vec<T>>,
}

impl<T> Grid<T>
where
    T: Copy,
{
    pub fn get(&self, pos: Point<usize>) -> T {
        self.values[pos.y][pos.x]
    }

    pub fn set(&mut self, pos: Point<usize>, value: T) {
        self.values[pos.y][pos.x] = value
    }

    pub fn get_neighbors(&self, pos: Point<usize>) -> Vec<GridCell<T>> {
        let mut neighbors = Vec::with_capacity(8);

        if pos.y > 0 {
            let y = pos.y - 1;

            // North West
            if pos.x > 0 {
                let x = pos.x - 1;
                let v = self.values[y][x];

                neighbors.push(GridCell {
                    position: Point::new(x, y),
                    value: v,
                })
            }

            // North
            {
                let v = self.values[y][pos.x];

                neighbors.push(GridCell {
                    position: Point::new(pos.x, y),
                    value: v,
                })
            }

            // North East
            if pos.x < self.width - 1 {
                let x = pos.x + 1;
                let v = self.values[y][x];

                neighbors.push(GridCell {
                    position: Point::new(x, y),
                    value: v,
                })
            }
        }

        // West
        if pos.x > 0 {
            let x = pos.x - 1;
            let v = self.values[pos.y][x];

            neighbors.push(GridCell {
                position: Point::new(x, pos.y),
                value: v,
            })
        }

        // East
        if pos.x < self.width - 1 {
            let x = pos.x + 1;
            let v = self.values[pos.y][x];

            neighbors.push(GridCell {
                position: Point::new(x, pos.y),
                value: v,
            })
        }

        if pos.y < self.height - 1 {
            let y = pos.y + 1;

            // South West
            if pos.x > 0 {
                let x = pos.x - 1;
                let v = self.values[y][x];

                neighbors.push(GridCell {
                    position: Point::new(x, y),
                    value: v,
                })
            }

            // South
            {
                let v = self.values[y][pos.x];

                neighbors.push(GridCell {
                    position: Point::new(pos.x, y),
                    value: v,
                })
            }

            // South East
            if pos.x < self.width - 1 {
                let x = pos.x + 1;
                let v = self.values[y][x];

                neighbors.push(GridCell {
                    position: Point::new(x, y),
                    value: v,
                })
            }
        }

        neighbors
    }
}

impl<T> Grid<T>
where
    T: From<char>,
{
    #[tracing::instrument(skip_all)]
    pub fn from_input(input: &[String]) -> Result<Self, &'static str> {
        let height = input.len();
        let width = input[0].len();

        let values: Result<Vec<Vec<T>>, &'static str> = input
            .iter()
            .map(|r| {
                if r.len() != width {
                    return Err("Invalid row length");
                }
                Ok(r.chars().map(T::from).collect())
            })
            .collect();

        values.map(|v| Grid {
            height,
            width,
            values: v,
        })
    }
}

impl<T> std::fmt::Debug for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Grid {\n")?;

        for r in &self.values {
            f.write_fmt(format_args!(
                "{}\n",
                r.iter().map(|v| format!("{}", v)).collect::<String>()
            ))?;
        }

        f.write_str("}\n")
    }
}
