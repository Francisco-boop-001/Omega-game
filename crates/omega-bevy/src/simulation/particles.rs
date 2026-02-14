use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParticleKind {
    Fire,
    Smoke,
    Steam,
    Debris,
    Gust,
}

#[derive(Component)]
pub struct Particle {
    pub logical_pos: Vec3,
    pub velocity: Vec3,
    pub age: f32,
    pub max_age: f32,
    pub weight: f32,
    pub kind: ParticleKind,
}

#[derive(Component)]
pub struct VisualCascade {
    pub glyphs: Vec<char>, // e.g. ['@', '*', '.']
    pub colors: Vec<Color>, // Color gradient
    pub rotate: bool,
    pub initial_scale: f32,
}
