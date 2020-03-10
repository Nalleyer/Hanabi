use specs::prelude::*;

pub mod hb_components;
pub mod hb_systems;

use hb_components::*;
use hb_systems::*;

pub struct HbShow {
    world: Option<specs::World>,
    dispatcher: Option<specs::Dispatcher<'static, 'static>>,
}

impl HbShow {
    pub fn new() -> Self {
        HbShow { world: None, dispatcher: None }
    }

    pub fn setup(&mut self) {
        let mut world = World::new();
        world.register::<Vel>();
        world.create_entity().with(Vel(2.0)).build();
        self.world.replace(world);

        let dispatcher = DispatcherBuilder::new()
            .with(SysTest, "sys_test", &[])
            .build();

        self.dispatcher.replace(dispatcher);
    }

    pub fn dispatch(&mut self) {
        if let Some(d) = &mut self.dispatcher {
            d.dispatch(self.world.as_mut().unwrap());
        }
    }

    pub fn has_world(&self) -> bool {
        self.world.is_some()
    }

    pub fn get_world_mut(&mut self) -> Option<&mut World> {
        self.world.as_mut()
    }
}