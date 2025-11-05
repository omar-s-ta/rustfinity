use std::{
    collections::VecDeque,
    ops::{Add, Sub},
};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Point {
    fn from(p: (usize, usize)) -> Self {
        Point { x: p.0, y: p.1 }
    }
}

impl From<Point> for (usize, usize) {
    fn from(p: Point) -> Self {
        (p.x, p.y)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
        }
    }
}

struct State {
    point: Point,
    path: Vec<Point>,
}

impl State {
    fn new(point: Point) -> Self {
        State {
            point,
            path: vec![point],
        }
    }

    fn new_state(&self, point: Point) -> Self {
        let mut path = self.path.clone();
        path.push(point);
        Self { point, path }
    }
}

struct Maze {
    board: Vec<Vec<bool>>,
}

impl Maze {
    fn new<T: AsRef<[char]>>(maze: &[T]) -> Self {
        let board = maze
            .iter()
            .map(|row| {
                row.as_ref()
                    .iter()
                    .map(|&ch| ch == '.' || ch == 'S' || ch == 'E')
                    .collect()
            })
            .collect();

        Maze { board }
    }

    fn solve(&mut self, src: Point, trg: Point) -> Vec<Point> {
        let x_dir = Point { x: 1, y: 0 };
        let y_dir = Point { x: 0, y: 1 };
        let mut queue: VecDeque<_> = vec![State::new(src)].into();

        while let Some(current) = queue.pop_front() {
            if current.point == trg {
                return current.path;
            }
            self.unset(current.point);
            for dir in [x_dir, y_dir] {
                let add = current.point + dir;
                let sub = current.point - dir;
                if self.available(&add) {
                    queue.push_back(current.new_state(add));
                }
                if self.available(&sub) {
                    queue.push_back(current.new_state(sub));
                }
            }
        }
        vec![]
    }

    fn contains(&self, point: &Point) -> bool {
        (0..self.board.len()).contains(&point.x) && (0..self.board[0].len()).contains(&point.y)
    }

    fn available(&self, point: &Point) -> bool {
        self.contains(point) && self.is_clear(point)
    }

    fn unset(&mut self, point: Point) {
        self.board[point.x][point.y] = false;
    }

    fn is_clear(&self, point: &Point) -> bool {
        self.board[point.x][point.y]
    }
}

pub fn solve_maze(
    maze: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut maze = Maze::new(maze.as_ref());
    let result = maze.solve(start.into(), end.into());

    result.iter().map(|&p| p.into()).collect()
}
