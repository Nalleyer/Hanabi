use specs::prelude::*;

#[derive(Debug)]
pub struct Vel(pub f32);

impl Component for Vel {
    type Storage = VecStorage<Self>;
}
