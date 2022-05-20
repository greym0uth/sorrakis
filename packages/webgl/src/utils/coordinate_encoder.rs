pub trait CoordinateEncoder {
    fn decode(&self, index: usize) -> (u32, u32);
    fn encode(&self, x: i32, y: i32) -> Option<usize>;
}

pub struct LoopingEncoder {
    pub dimensions: (u32, u32),
}

impl CoordinateEncoder for LoopingEncoder {
    fn decode(&self, index: usize) -> (u32, u32) {
        (
            index as u32 % self.dimensions.0,
            index as u32 / self.dimensions.0,
        )
    }

    fn encode(&self, x: i32, y: i32) -> Option<usize> {
        // If location is negative loop back to end of corresponding coordinate space.
        let x = if x < 0 {
            (self.dimensions.0 as i32 + x) as u32
        } else {
            x as u32 % self.dimensions.0
        };
        let y = if y < 0 {
            (self.dimensions.1 as i32 + y) as u32
        } else {
            y as u32
        };
        // Perform a modulo on the length of the tiles vector to loop coordinate space.
        Some(
            (y * self.dimensions.0 + x) as usize % (self.dimensions.0 * self.dimensions.1) as usize,
        )
    }
}

impl Clone for LoopingEncoder {
    fn clone(&self) -> LoopingEncoder {
        LoopingEncoder {
            dimensions: self.dimensions,
        }
    }
}

pub struct FlatEncoder {
    pub dimensions: (u32, u32),
}

impl CoordinateEncoder for FlatEncoder {
    fn decode(&self, index: usize) -> (u32, u32) {
        (
            index as u32 % self.dimensions.0,
            index as u32 / self.dimensions.0,
        )
    }

    fn encode(&self, x: i32, y: i32) -> Option<usize> {
        if x >= 0 && x < self.dimensions.0 as i32 && y >= 0 && y < self.dimensions.1 as i32 {
            Some((y as u32 * self.dimensions.0 + x as u32) as usize)
        } else {
            None
        }
    }
}

impl Clone for FlatEncoder {
    fn clone(&self) -> FlatEncoder {
        FlatEncoder {
            dimensions: self.dimensions,
        }
    }
}

#[derive(Copy)]
pub struct ScreenSpaceEncoder {
    pub dimensions: (u32, u32),
}

impl Clone for ScreenSpaceEncoder {
    fn clone(&self) -> ScreenSpaceEncoder {
        ScreenSpaceEncoder {
            dimensions: self.dimensions,
        }
    }
}

impl ScreenSpaceEncoder {
    pub fn updateDimensions(&mut self, width: u32, height: u32) {
        self.dimensions = (width, height);
    }

    pub fn decode(&self, x: f32, y: f32) -> (f32, f32) {
        //from normal space
        return (
            map(x, (-1.0, 1.0), (0.0, self.dimensions.0 as f32)),
            map(y, (-1.0, 1.0), (0.0, self.dimensions.1 as f32)),
        );
    }

    pub fn encode(&self, x: f32, y: f32) -> (f32, f32) {
        //from screenspace
        return (
            map(x, (0.0, self.dimensions.0 as f32), (-1.0, 1.0)),
            map(y, (0.0, self.dimensions.1 as f32), (-1.0, 1.0)),
        );
    }
}

fn map(index: f32, a: (f32, f32), b: (f32, f32)) -> f32 {
    return (index - a.0) / (a.1 - a.0) * (b.1 - b.0) + b.0;
}
