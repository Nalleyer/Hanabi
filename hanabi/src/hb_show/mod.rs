use specs::prelude::*;

// test component
#[derive(Debug)]
pub struct Vel(f32);

impl Component for Vel {
    type Storage = VecStorage<Self>;
}

// test system
pub struct SysTest;

impl<'a> System<'a> for SysTest {
    type SystemData = (WriteStorage<'a, Vel>);

    fn run(&mut self, (mut vs): Self::SystemData) {
        for (v) in (&mut vs).join() {
            v.0 += 1f32;
            // info!("{}", v.0);
        }
    }
}

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

        let mut dispatcher = DispatcherBuilder::new()
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