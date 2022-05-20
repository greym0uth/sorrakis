use std::cell::Cell;
use web_sys::WebGlRenderingContext as GL;
use rand::prelude::*;

use crate::{
    rendering::{Rectangle, Instance},
    simulations::Simulation,
    utils::{CoordinateEncoder, FlatEncoder},
};

const X_FLAG: u32    = 0xFFF00000;
const Y_FLAG: u32    = 0x000FFF00;
const TILE_FLAG: u32 = 0x000000FF;

#[derive(Clone, Copy)]
struct Tile {
    x: u32,
    y: u32,
    id: u8,
}

struct TileStorage {
    encoder: FlatEncoder,
    pub tiles: Vec<Cell<Tile>>,
    tilemap: Vec<Cell<Option<usize>>>,
}

impl TileStorage {
    pub fn new(width: u32, height: u32) -> Self {
        TileStorage {
            encoder: FlatEncoder { dimensions: (width, height) },
            tiles: Vec::<Cell<Tile>>::new(),
            tilemap: vec![Cell::<Option<usize>>::new(None); (width * height) as usize],
        }
    }

    pub fn get(&self, x: i32, y: i32) -> Option<&Cell<Tile>> {
        let mut result = None;
        if let Some(tile_index) = self.get_index(x, y) {
            result = Some(&self.tiles[tile_index as usize]);
        }
        result
    }

    fn get_index(&self, x: i32, y: i32) -> Option<usize> {
        let mut result = None;
        if let Some(map_index) = self.encoder.encode(x, y) {
            result = self.tilemap[map_index].get();
        }
        result
    }

    pub fn insert(&mut self, tile: Tile) {
        self.tilemap[self.encoder.encode(tile.x as i32, tile.y as i32).unwrap()].replace(Some(self.tiles.len()));
        self.tiles.push(Cell::<Tile>::new(tile));
    }

    pub fn swap(&self, tile: &Cell<Tile>, old_coords: (u32, u32), new_coords: (i32, i32)) {
        let old_index = self.encoder.encode(old_coords.0 as i32, old_coords.1 as i32).unwrap();
        // Only swap if new location is valid.
        if let Some(new_index) = self.encoder.encode(new_coords.0, new_coords.1) {
            self.tilemap[old_index].swap(&self.tilemap[new_index]);
            tile.set(Tile { x: new_coords.0 as u32, y: new_coords.1 as u32, id: tile.get().id });
            // Update the coordinates of the swapped tile if its an actual tile.
            if let Some(old_tile) = self.get(old_coords.0 as i32, old_coords.1 as i32) {
                old_tile.set(Tile { x: old_coords.0, y: old_coords.1, id: old_tile.get().id });
            }
        }
    }
}

pub struct FallingSand {
    dimensions: (u32, u32),
    tiles: TileStorage,
    renderer: Rectangle,
    random: ThreadRng,
    spawn_count: u32,
}

impl FallingSand {
    pub fn new(gl: &GL, width: u32, height: u32) -> Self {
        let mut tiles = TileStorage::new(width, height);
        let mut rng = rand::thread_rng();
        let encoder = FlatEncoder { dimensions: (width, height) };

        for index in 0..width*height {
            let weight = rng.gen::<f32>();
            let tile_id = if weight > 0.9 {
                1
            } else {
                0
            };

            if tile_id != 0 {
                let (x, y) = encoder.decode(index as usize);
                let tile = Tile { x, y, id: tile_id };
                tiles.insert(tile);
            }
        }

        Self {
            dimensions: (width, height),
            tiles,
            renderer: Rectangle::new(&gl),
            random: rng,
            spawn_count: 0,
        }
    }
}

impl Simulation for FallingSand {
    fn update(&mut self) {
        if self.spawn_count < 1000 {
            self.spawn_count += 1;
            for point in 0..20 {
                if self.random.gen::<f32>() > 0.75 {
                    self.tiles.insert(Tile { x: point + 4, y: self.dimensions.1 - 1, id: 2 })
                }
            }
        }

        for raw_tile in self.tiles.tiles.iter() {
            let tile = raw_tile.get();
            if tile.y > 0 {
                if tile.id == 1 {
                    if let Some(below) = self.tiles.get(tile.x as i32, tile.y as i32 - 1) {
                        if below.get().id == 2 {
                            self.tiles.swap(raw_tile, (tile.x, tile.y), (tile.x as i32, tile.y as i32 - 1));
                        } else {
                            let direction = if self.random.gen::<f32>() > 0.5 { -1 } else { 1 };
                            if let Some(below) = self.tiles.get(tile.x as i32 + direction, tile.y as i32 - 1) {
                                if below.get().id == 2 {
                                    self.tiles.swap(raw_tile, (tile.x, tile.y), (tile.x as i32 + direction, tile.y as i32 - 1));
                                }
                            } else {
                                self.tiles.swap(raw_tile, (tile.x, tile.y), (tile.x as i32 + direction, tile.y as i32 - 1));
                            }
                        }
                    } else {
                        self.tiles.swap(raw_tile, (tile.x, tile.y), (tile.x as i32, tile.y as i32 - 1));
                    }
                } else if tile.id == 2 {
                    if self.tiles.get(tile.x as i32, tile.y as i32 - 1).is_some() {
                        let direction = if self.random.gen::<f32>() > 0.5 { -1 } else { 1 };
                        if self.tiles.get(tile.x as i32 + direction, tile.y as i32).is_none() {
                            if (direction < 0 && tile.x > 0) || (direction > 0 && tile.x < self.dimensions.0 - 1) {
                                self.tiles.swap(raw_tile, (tile.x, tile.y), (tile.x as i32 + direction, tile.y as i32));
                            }
                        } else if self.tiles.get(tile.x as i32 - direction, tile.y as i32).is_none() {
                            if (-direction < 0 && tile.x > 0) || (-direction > 0 && tile.x < self.dimensions.0 - 1) {
                                self.tiles.swap(raw_tile, (tile.x, tile.y), (tile.x as i32 - direction, tile.y as i32));
                            }
                        }
                    } else {
                        self.tiles.swap(raw_tile, (tile.x, tile.y), (tile.x as i32, tile.y as i32 - 1));
                    }
                }
            }
        }
    }

    fn render(&self, gl: &GL) {
        let mut instances = Vec::<Instance>::with_capacity(self.tiles.tiles.len());
        self.renderer.bind(gl);
        for tile in &self.tiles.tiles {
            let tile = tile.get();
            let width = 2.0 / self.dimensions.0 as f32;
            let height = 2.0 / self.dimensions.1 as f32;
            let x: f32 = width * tile.x as f32 - 1.0;
            let y: f32 = height * tile.y as f32 - 1.0;
            let color: [f32; 4] = if tile.id == 1 {
                [237.0 / 256.0, 201.0 / 256.0, 175.0 / 256.0, 1.0]
            } else if tile.id == 2 {
                [0.0, 0.41, 0.58, 1.0]
            } else {
                [0.0,0.0,0.0,0.0]
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
