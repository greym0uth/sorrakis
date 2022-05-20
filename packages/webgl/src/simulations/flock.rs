use crate::rendering::Instance;
use cgmath::prelude::*;
use rand::prelude::*;
use std::ops::{Add, Div, DivAssign, Mul};
use web_sys::WebGlRenderingContext as GL;

use crate::{
    quadtree::Quadtree, quadtree::Rectangle as Rect, rendering::Rectangle, rendering::Triangle,
    utils::ScreenSpaceEncoder,
};

#[derive(Debug, Copy, Clone)]
pub struct Boid {
    pub position: cgmath::Vector2<f32>,
    pub velocity: cgmath::Vector2<f32>,
    pub acceleration: cgmath::Vector2<f32>,
    pub alignment_force: f32,
    pub cohesion_force: f32,
    pub seperation_force: f32,
    pub perception_size: f32,
    pub max_speed: f32,
    pub index: usize,
}

impl Boid {
    pub fn update(&mut self, width: i32, height: i32, sensed_boids: &Vec<(Boid, f32)>) {
        self.acceleration *= 0.0;

        let alignment = self.align(sensed_boids);
        let cohesion = self.cohesion(sensed_boids);
        let seperation = self.seperation(sensed_boids);

        self.edges(width, height); // wrap space into a torus

        self.acceleration = self.acceleration + seperation + cohesion + alignment;

        self.position = self.position.add(self.velocity);
        self.velocity = self.velocity.add(self.acceleration);
        self.velocity = self.limit(&self.velocity, self.max_speed);
        if self.velocity.magnitude() < self.max_speed * 0.25 {
            self.velocity = self.setMag(self.max_speed * 0.25, &self.velocity);
        }
        //apply cohesion seperation and alignment forces
    }

    //instead of boids draining off the edges wrap space into a torus lmao
    fn edges(&mut self, width: i32, height: i32) {
        if self.position.x > (width + 11) as f32 {
            self.position.x = -11.0;
        } else if self.position.x < -11.0 {
            self.position.x = (width + 11) as f32;
        } // add some arbitrary amount so the triangles are popping in and out of existancel

        if self.position.y > (height + 11) as f32 {
            self.position.y = -11.0;
        } else if self.position.y < -11.0 {
            self.position.y = (height + 11) as f32;
        }
    }

    fn align(&mut self, boids: &Vec<(Boid, f32)>) -> cgmath::Vector2<f32> {
        let mut steering = cgmath::Vector2::new(0.0, 0.0);
        let mut total = 0;
        for boid in boids {
            if boid.0.index == self.index {
                continue;
            }
            let distance = boid.1;

            if distance < self.perception_size {
                steering = steering.add(boid.0.velocity);
                total += 1;
            }
        }

        if total > 0 && steering != cgmath::Vector2::new(0.0, 0.0) {
            steering /= total as f32;
            steering = self.setMag(self.max_speed, &steering);
            steering -= self.velocity;
            steering = self.limit(&steering, self.alignment_force)
        }

        return steering;
    }

    fn cohesion(&mut self, boids: &Vec<(Boid, f32)>) -> cgmath::Vector2<f32> {
        let mut steering = cgmath::Vector2::new(0.0, 0.0);
        let mut total = 0;
        for other in boids {
            let distance = other.1;
            if distance < self.perception_size {
                steering = steering + other.0.position;
                total += 1;
            }
        }
        if total > 0 && steering != cgmath::Vector2::new(0.0, 0.0) {
            steering /= total as f32;
            steering -= self.position;
            steering = self.setMag(self.max_speed, &steering);
            steering -= self.velocity;
            steering = self.limit(&steering, self.cohesion_force);
        }

        return steering;
    }

    fn seperation(&mut self, boids: &Vec<(Boid, f32)>) -> cgmath::Vector2<f32> {
        let perception = 50.0 / 1.75;
        let mut steering = cgmath::Vector2::new(0.0, 0.0);
        let mut total = 0;
        for other in boids {
            let distance = other.1;
            if distance < perception {
                let mut diff = self.position.clone();
                diff -= other.0.position;
                diff /= distance;
                steering += diff;
                total += 1;
            }
        }
        if total > 0 && steering != cgmath::Vector2::new(0.0, 0.0) {
            steering /= total as f32;
            steering = self.setMag(self.max_speed, &steering);
            steering -= self.velocity;
            steering = self.limit(&steering, self.seperation_force);
        }

        return steering;
    }

    fn limit(&self, vec: &cgmath::Vector2<f32>, speed: f32) -> cgmath::Vector2<f32> {
        if vec.magnitude() > speed {
            return self.setMag(speed, vec);
        } else {
            return *vec;
        }
    }

    fn setMag(&self, mag: f32, vec: &cgmath::Vector2<f32>) -> cgmath::Vector2<f32> {
        let currentMag = vec.magnitude();
        let mut newmag = cgmath::Vector2::new(0.0, 0.0);

        if currentMag != 0.0 {
            newmag = (vec * mag) / currentMag;
        }

        return newmag;
    }
}

pub struct Flock {
    dimensions: (u32, u32),
    aspect: f32,
    boids: Vec<Boid>,
    triangle: Triangle,
    quadtree: Quadtree,
    line: Rectangle,
    encoder: ScreenSpaceEncoder,
    count: u32,
}

impl Flock {
    pub fn new(gl: &GL, width: u32, height: u32) -> Self {
        let encoder = ScreenSpaceEncoder {
            dimensions: (width, height),
        };

        let boidshape = [0.0, 0.5, 0.34, -0.5, -0.34, -0.5];
        let mut rng = rand::thread_rng();
        let mut boids = Vec::<Boid>::new();
        let mut qt = Quadtree::new(
            2,
            Rect {
                x: 0.0,
                y: 0.0,
                width: width as f32,
                height: height as f32,
            },
        );

        for index in 0..300 {
            boids.push(Boid {
                position: cgmath::Vector2::new(
                    rng.gen::<f32>() * encoder.dimensions.0 as f32,
                    rng.gen::<f32>() * encoder.dimensions.1 as f32,
                ),
                velocity: cgmath::Vector2::new(
                    (rng.gen::<f32>() * 2.0) - 1.0,
                    (rng.gen::<f32>() * 2.0) - 1.0,
                ),
                acceleration: cgmath::Vector2::new(0.0, 0.0),
                alignment_force: 0.4,
                cohesion_force: 0.2,
                seperation_force: 0.4,
                perception_size: 75.0 / 2.0,
                max_speed: 7.0 / 2.0,
                index,
            });

            //TODO vectorlib
            let _ = qt.insert(boids[index].position, index);
        }

        Self {
            dimensions: (width, height),
            aspect: width as f32 / height as f32,
            triangle: Triangle::new(&gl, boidshape),
            boids,
            quadtree: qt,
            line: Rectangle::new(&gl),
            encoder,
            count: 0,
        }
    }

    fn wrapped_distance(
        vec1: cgmath::Vector2<f32>,
        vec2: cgmath::Vector2<f32>,
        width: u32,
        height: u32,
    ) -> f32 {
        let mut dx = (vec1.x - vec2.x).abs();
        let mut dy = (vec1.y - vec2.y).abs();

        if dx > width as f32 {
            dx = width as f32 - dx;
        }
        if dy > height as f32 {
            dy = height as f32 - dy;
        }

        return (dx.powi(2) + dy.powi(2)).sqrt();
    }

    fn getLocalBoids(&self, circle: (f32, f32, f32)) {
        let mut boid_indexs: Vec<usize> = Vec::new();

        if circle.0 + circle.2 > 2.0 {
            boid_indexs.extend(self.quadtree.query(circle));
        }
    }

    fn distance(vec1: (f32, f32), vec2: (f32, f32)) -> f32 {
        return ((vec2.0 - vec1.0).powi(2) + (vec2.1 - vec1.1).powi(2)).sqrt();
    }

    pub fn update(&mut self, width: i32, height: i32) {
        self.encoder.updateDimensions(width as u32, height as u32);
        self.count = (self.count + 1) % 101;
        self.aspect = width as f32 / height as f32;
        self.dimensions = (width as u32, height as u32);
        self.quadtree.set_dimensions(width as f32, height as f32);
        let mut newquadtree = Quadtree::new(
            2,
            Rect {
                x: 0.0,
                y: 0.0,
                width: width as f32,
                height: height as f32,
            },
        );
        //self.quadtree.reset();

        //got a feeling this needs to be in the loop, p sure it causes ghost boids or something when it isnt
        let test = self.boids.clone();
        for (pos, boid) in self.boids.iter_mut().enumerate() {
            let mut sensed: Vec<(Boid, f32)> = Vec::new();

            let selected =
                self.quadtree
                    .query((boid.position.x, boid.position.y, boid.perception_size));

            for i in selected {
                if boid.index != test[i].index {
                    sensed.push((
                        test[i],
                        crate::Flock::wrapped_distance(
                            boid.position,
                            test[i].position,
                            self.dimensions.0,
                            self.dimensions.1,
                        ),
                    ))
                }
            }

            boid.update(width, height, &sensed);

            newquadtree.insert(boid.position, pos);
        }
        self.quadtree = newquadtree;
    }

    pub fn render(&self, gl: &GL) {
        /*let selected = self.quadtree.query((
            self.dimensions.0 as f32 / 2.0,
            self.dimensions.1 as f32 / 2.0,
            100.0,
        ));*/
        self.quadtree.renderroot(&gl, &self.line, self.encoder);

        let mut color = [1.0, 1.0, 1.0, 1.0];
        let mut instances = Vec::<Instance>::with_capacity(self.boids.len());
        for (index, boid) in self.boids.iter().enumerate() {
            let ang = boid.velocity.y.atan2(boid.velocity.x);

            //if selected.iter().any(|&i| i == index) {
            //    color = [0.0, 1.0, 0.0, 1.0];
            //} else {
            color = [0.37, 0.22, 0.40, 1.0];
            //}
            let test = self.encoder.encode(boid.position.x, boid.position.y);

            instances.push(Instance {
                x: test.0,
                y: test.1,
                width: 0.05,
                height: 0.05,
                angle: ang - std::f32::consts::FRAC_PI_2,
                color,
            });

            /*self.triangle.render(
                &gl,
                test.0,
                test.1,
                0.05,
                0.05, // self.aspect,
                ang - std::f32::consts::FRAC_PI_2,
                color,
            );*/
        }
        self.triangle.render_instances(&gl, instances)
    }
}
