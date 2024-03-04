use rand::{rngs::ThreadRng, Rng};
use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource};
use std::collections::HashSet;

struct Maze {
    width: u32,
    height: u32,
    open_walls: HashSet<(u32, u32, u32, u32)>,
}

impl Maze {
    fn new(width: u32, height: u32) -> Maze {
        Maze {
            width,
            height,
            open_walls: HashSet::new(),
        }
    }

    fn open_wall(&mut self, x1: u32, y1: u32, x2: u32, y2: u32) -> bool {
        self.open_walls.insert((x1, y1, x2, y2)) && self.open_walls.insert((x2, y2, x1, y1))
    }

    fn is_wall_open(&self, x1: u32, y1: u32, x2: u32, y2: u32) -> bool {
        self.open_walls.contains(&(x1, y1, x2, y2))
    }

    fn is_every_cell_reachable(&self) -> bool {
        let mut visited = HashSet::new();
        let mut stack = vec![(0, 0)];
        while let Some((x, y)) = stack.pop() {
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
        visited.len() == (self.width * self.height) as usize
    }

    fn gen(&mut self, rng: &mut ThreadRng) {
        while !self.is_every_cell_reachable() {
            loop {
                let x = rng.gen_range(0..self.width);
                let y = rng.gen_range(0..self.height);
                let direction = rng.gen_range(0..4);
                if match direction {
                    0 if x > 0 => self.open_wall(x, y, x - 1, y),
                    1 if x < self.width - 1 => self.open_wall(x, y, x + 1, y),
                    2 if y > 0 => self.open_wall(x, y, x, y - 1),
                    3 if y < self.height - 1 => self.open_wall(x, y, x, y + 1),
                    _ => false,
                } {
                    break;
                }
            }
        }
    }

    fn show(&self) {
        let cell_size = 20;
        let mut dt = DrawTarget::new(
            self.width as i32 * cell_size,
            self.height as i32 * cell_size,
        );
        let white_solid = raqote::Source::Solid(SolidSource {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        });
        let black_solid = raqote::Source::Solid(SolidSource {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        });
        dt.fill_rect(
            0.,
            0.,
            self.width as f32 * cell_size as f32,
            self.height as f32 * cell_size as f32,
            &white_solid,
            &DrawOptions::new(),
        );

        let mut pb = PathBuilder::new();

        for y in 0..self.height {
            for x in 0..self.width {
                pb.move_to(x as f32 * cell_size as f32, y as f32 * cell_size as f32);
                if x > 0 && !self.is_wall_open(x, y, x - 1, y) {
                    pb.line_to(
                        x as f32 * cell_size as f32,
                        (y + 1) as f32 * cell_size as f32,
                    );
                } else {
                    pb.move_to(
                        x as f32 * cell_size as f32,
                        (y + 1) as f32 * cell_size as f32,
                    );
                }
                if y < self.height - 1 && !self.is_wall_open(x, y, x, y + 1) {
                    pb.line_to(
                        (x + 1) as f32 * cell_size as f32,
                        (y + 1) as f32 * cell_size as f32,
                    );
                } else {
                    pb.move_to(
                        (x + 1) as f32 * cell_size as f32,
                        (y + 1) as f32 * cell_size as f32,
                    );
                }
                if x < self.width - 1 && !self.is_wall_open(x, y, x + 1, y) {
                    pb.line_to(
                        (x + 1) as f32 * cell_size as f32,
                        y as f32 * cell_size as f32,
                    );
                } else {
                    pb.move_to(
                        (x + 1) as f32 * cell_size as f32,
                        y as f32 * cell_size as f32,
                    );
                }
                if y > 0 && !self.is_wall_open(x, y, x, y - 1) {
                    pb.line_to(x as f32 * cell_size as f32, y as f32 * cell_size as f32);
                } else {
                    pb.move_to(x as f32 * cell_size as f32, y as f32 * cell_size as f32);
                }
            }
        }
        let path = pb.finish();
        dt.stroke(
            &path,
            &black_solid,
            &raqote::StrokeStyle {
                cap: raqote::LineCap::Round,
                join: raqote::LineJoin::Round,
                width: 3.,
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
    let mut rng = rand::thread_rng();
    let mut maze = Maze::new(20, 20);
    maze.gen(&mut rng);
    maze.show();
}
