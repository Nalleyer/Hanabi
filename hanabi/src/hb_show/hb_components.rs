use specs::prelude::*;
use specs_derive::{Component};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Vel(pub f32);