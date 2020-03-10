use specs::prelude::*;

pub struct SysTest;

use crate::hb_show::hb_components::Vel;

impl<'a> System<'a> for SysTest {
    type SystemData = WriteStorage<'a, Vel>;

    fn run(&mut self, mut vs: Self::SystemData) {
        for v in (&mut vs).join() {
            v.0 += 1f32;
            // info!("{}", v.0);
        }
    }
}

