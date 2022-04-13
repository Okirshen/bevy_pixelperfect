use bevy::prelude::Component;
use image::RgbaImage;
use std::ops::{Add, Sub};

/// Position of an entity
#[derive(Debug, Clone, Copy, Component)]
pub struct Position {
    pub x: i16,
    pub y: i16,
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// Image to render
#[derive(Debug, Clone, Component)]
pub struct Image {
    pub data: RgbaImage,
    pub width: u32,
    pub height: u32,
    pub visible: bool,
}

#[derive(Component)]
pub struct Camera;

// Scale of the image
#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub struct Scale(pub u32);
