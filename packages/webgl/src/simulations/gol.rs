use rand::prelude::*;
use web_sys::WebGlRenderingContext as GL;

use crate::rendering::{Rectangle, Instance};

pub struct GoL {
    dimensions: (u32, u32),
    tiles: Vec<bool>,
    renderer: Rectangle,
}

impl GoL {
    pub fn new(gl: &GL, width: u32, height: u32) -> Self {
        let mut tiles = Vec::<bool>::new();
        let mut rng = rand::thread_rng();

        for _ in 0..width * height {
            tiles.push(rng.gen::<f32>() > 0.9);
        }

        Self {
            dimensions: (width, height),
            tiles,
            renderer: Rectangle::new(&gl),
        }
    }

    fn decode(&self, index: usize) -> (u32, u32) {
        (
            index as u32 % self.dimensions.0,
            index as u32 / self.dimensions.0,
        )
    }

    fn encode(&self, x: i32, y: i32) -> usize {
        // If location is negative loop back to end of corresponding coordinate space.
        let x = if x < 0 {
            (self.dimensions.0 as i32 + x) as u32
        } else {
            x as u32
        };
        let y = if y < 0 {
            (self.dimensions.1 as i32 + y) as u32
        } else {
            y as u32
        };
        // Perform a modulo on the length of the tiles vector to loop coordinate space.
        (y * self.dimensions.0 + x) as usize % self.tiles.len() as usize
    }

    fn get_active_neighbor_count(&self, x: u32, y: u32) -> u32 {
        //wrap screen maybe?
        let mut count = 0;
        for horizontal_offset in -1..2 {
            for vertical_offset in -1..2 {
                if horizontal_offset == 0 && vertical_offset == 0 {
                    continue;
                }
                count += self.tiles
                    [self.encode(x as i32 + horizontal_offset, y as i32 + vertical_offset)]
                    as u32;
            }
        }
        count
    }

    pub fn update(&mut self, width: i32, height: i32) {
        let mut tiles_buffer = self.tiles.clone();
        for (index, tile) in self.tiles.iter().enumerate() {
            let (x, y) = self.decode(index);
            let active_neighbor_count = self.get_active_neighbor_count(x, y);
            tiles_buffer[index] =
                active_neighbor_count == 3 || (*tile && active_neighbor_count == 2);
        }
        self.tiles = tiles_buffer;
    }

    pub fn render(&self, gl: &GL) {
        let mut instances = Vec::<Instance>::with_capacity(self.tiles.len());
        for (index, &active) in self.tiles.iter().enumerate() {
            let (col, row) = self.decode(index);
            let width = 2.0 / self.dimensions.0 as f32;
            let height = 2.0 / self.dimensions.1 as f32;
            let x: f32 = width * col as f32 - 1.0;
            let y: f32 = height * row as f32 - 1.0;
            let color: [f32; 4] = if active {
                [115.0 / 256.0, 69.0 / 256.0, 124.0 / 256.0, 1.0]
            } else {
                [0.0, 0.0, 0.0, 0.0]
            };

            instances.push(Instance {
                x,
                y,
                width,
                height,
                angle: 0.0,
                color,
            });
        }
        self.renderer.render_instances(gl, instances);
    }
}
