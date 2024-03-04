use rand::seq::SliceRandom;
use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource};
use std::collections::{HashMap, HashSet, VecDeque};

struct Maze {
    width: u32,
    height: u32,
    open_walls: HashSet<(u32, u32, u32, u32)>,
}

impl Maze {
    fn new(width: u32, height: u32) -> Maze {
        let mut maze = Maze {
            width,
            height,
            open_walls: HashSet::new(),
        };
        let mut rng = rand::thread_rng();
        let mut walls = HashSet::new();
        for x in 0..width {
            for y in 0..height {
                if x > 0 {
                    walls.insert(Self::sort_wall(x, y, x - 1, y));
                }
                if x < width - 1 {
                    walls.insert(Self::sort_wall(x, y, x + 1, y));
                }
                if y > 0 {
                    walls.insert(Self::sort_wall(x, y, x, y - 1));
                }
                if y < height - 1 {
                    walls.insert(Self::sort_wall(x, y, x, y + 1));
                }
            }
        }
        let mut walls = walls
            .into_iter()
            .collect::<Vec<(u32, u32, u32, u32)>>();
        walls.shuffle(&mut rng);
        maze.gen(walls);
        maze
    }

    fn sort_wall(x1: u32, y1: u32, x2: u32, y2: u32) -> (u32, u32, u32, u32) {
        if x1 > x2 || (x1 == x2 && y1 > y2) {
            (x2, y2, x1, y1)
        } else {
            (x1, y1, x2, y2)
        }
    }

    fn open_wall(&mut self, x1: u32, y1: u32, x2: u32, y2: u32) {
        self.open_walls.insert((x1, y1, x2, y2));
        self.open_walls.insert((x2, y2, x1, y1));
    }

    fn is_wall_open(&self, x1: u32, y1: u32, x2: u32, y2: u32) -> bool {
        self.open_walls.contains(&(x1, y1, x2, y2))
    }

    fn is_dest_reachable(&self) -> bool {
        let mut visited = HashSet::new();
        let mut stack = vec![(0, 0)];
        while let Some((x, y)) = stack.pop() {
            if x == self.width - 1 && y == self.height - 1 {
                return true;
            }
            if visited.insert((x, y)) {
                if x > 0 && self.is_wall_open(x, y, x - 1, y) {
                    stack.push((x - 1, y));
                }
                if x < self.width - 1 && self.is_wall_open(x, y, x + 1, y) {
                    stack.push((x + 1, y));
                }
                if y > 0 && self.is_wall_open(x, y, x, y - 1) {
                    stack.push((x, y - 1));
                }
                if y < self.height - 1 && self.is_wall_open(x, y, x, y + 1) {
                    stack.push((x, y + 1));
                }
            }
        }
        false
    }

    fn gen(&mut self, walls: Vec<(u32, u32, u32, u32)>) {
        for (x1, y1, x2, y2) in walls {
            if self.is_dest_reachable() {
                break;
            }
            self.open_wall(x1, y1, x2, y2);
        }
    }

    fn get_short_path(&self) -> Vec<(u32, u32)> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::from([(0, 0)]);
        let mut prevs = HashMap::<(u32, u32), (u32, u32)>::new();
        while let Some((x, y)) = queue.pop_front() {
            if x == self.width - 1 && y == self.height - 1 {
                let mut path = vec![(x, y)];
                prevs.remove(&(0, 0));
                while let Some(&prev) = prevs.get(path.last().unwrap()) {
                    path.push(prev);
                }
                return path.into_iter().rev().collect();
            }
            if visited.insert((x, y)) {
                if x > 0 && self.is_wall_open(x, y, x - 1, y) {
                    queue.push_back((x - 1, y));
                    prevs.entry((x - 1, y)).or_insert((x, y));
                }
                if x < self.width - 1 && self.is_wall_open(x, y, x + 1, y) {
                    queue.push_back((x + 1, y));
                    prevs.entry((x + 1, y)).or_insert((x, y));
                }
                if y > 0 && self.is_wall_open(x, y, x, y - 1) {
                    queue.push_back((x, y - 1));
                    prevs.entry((x, y - 1)).or_insert((x, y));
                }
                if y < self.height - 1 && self.is_wall_open(x, y, x, y + 1) {
                    queue.push_back((x, y + 1));
                    prevs.entry((x, y + 1)).or_insert((x, y));
                }
            }
        }
        vec![]
    }

    fn show(&self) {
        let cell_size = 50.;
        let mut dt = DrawTarget::new(
            self.width as i32 * cell_size as i32,
            self.height as i32 * cell_size as i32,
        );
        let background_color = raqote::Source::Solid(SolidSource {
            r: 211,
            g: 211,
            b: 211,
            a: 255,
        });
        let wall_color = raqote::Source::Solid(SolidSource {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        });
        let path_color = raqote::Source::Solid(SolidSource {
            r: 255,
            g: 87,
            b: 51,
            a: 255,
        });
        dt.fill_rect(
            0.,
            0.,
            self.width as f32 * cell_size,
            self.height as f32 * cell_size,
            &background_color,
            &DrawOptions::new(),
        );

        let mut pb = PathBuilder::new();

        for y in 0..self.height {
            for x in 0..self.width {
                pb.move_to(x as f32 * cell_size, y as f32 * cell_size);
                if x > 0 && !self.is_wall_open(x, y, x - 1, y) {
                    pb.line_to(x as f32 * cell_size, (y + 1) as f32 * cell_size);
                } else {
                    pb.move_to(x as f32 * cell_size, (y + 1) as f32 * cell_size);
                }
                if y < self.height - 1 && !self.is_wall_open(x, y, x, y + 1) {
                    pb.line_to((x + 1) as f32 * cell_size, (y + 1) as f32 * cell_size);
                } else {
                    pb.move_to((x + 1) as f32 * cell_size, (y + 1) as f32 * cell_size);
                }
                if x < self.width - 1 && !self.is_wall_open(x, y, x + 1, y) {
                    pb.line_to((x + 1) as f32 * cell_size, y as f32 * cell_size);
                } else {
                    pb.move_to((x + 1) as f32 * cell_size, y as f32 * cell_size);
                }
                if y > 0 && !self.is_wall_open(x, y, x, y - 1) {
                    pb.line_to(x as f32 * cell_size, y as f32 * cell_size)
                } else {
                    pb.move_to(x as f32 * cell_size, y as f32 * cell_size);
                }
            }
        }
        let path = pb.finish();
        dt.stroke(
            &path,
            &wall_color,
            &raqote::StrokeStyle {
                cap: raqote::LineCap::Round,
                join: raqote::LineJoin::Round,
                width: 5.,
                miter_limit: 1.,
                dash_array: vec![],
                dash_offset: 0.,
            },
            &DrawOptions::new(),
        );

        let mut pb = PathBuilder::new();
        let short_path = self.get_short_path();
        for (i, (x, y)) in short_path.iter().enumerate() {
            if i == 0 {
                pb.move_to(
                    *x as f32 * cell_size + cell_size / 2.,
                    *y as f32 * cell_size + cell_size / 2.,
                );
            } else {
                pb.line_to(
                    *x as f32 * cell_size + cell_size / 2.,
                    *y as f32 * cell_size + cell_size / 2.,
                );
            }
        }

        let path = pb.finish();
        dt.stroke(
            &path,
            &path_color,
            &raqote::StrokeStyle {
                cap: raqote::LineCap::Round,
                join: raqote::LineJoin::Round,
                width: 15.,
                miter_limit: 1.,
                dash_array: vec![],
                dash_offset: 0.,
            },
            &DrawOptions::new(),
        );

        dt.write_png("out.png").unwrap()
    }
}

fn main() {
    let maze = Maze::new(50, 40);
    maze.show();
}
