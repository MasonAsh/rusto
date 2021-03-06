extern crate cgmath;
extern crate assimp;
extern crate gl;
extern crate sdl2;
extern crate rand;
extern crate image;

pub mod common;

pub use common::*;

mod config;
mod renderer;
mod scene;
pub mod game;

pub use game::Game;
