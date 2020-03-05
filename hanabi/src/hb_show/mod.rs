use specs::prelude::*;

// test component
#[derive(Debug)]
struct Vel(f32);

impl Component for Vel {
    type Storage = VecStorage<Self>;
}

// test system
struct SysTest;

impl<'a> System<'a> for SysTest {
    type SystemData = (WriteStorage<'a, Vel>);

    fn run(&mut self, (mut vs): Self::SystemData) {
        for (v) in (&mut vs).join() {
            v.0 += 1f32;
            println!("{}", v.0);
        }
    }
}

pub struct HanabiShow;

impl HanabiShow {
    pub fn new() -> Self {
        HanabiShow {}
    }

    pub fn run_world(&self) {
        // TODO: protect multiple call
        let mut world = World::new();
        world.register::<Vel>();
        // test entiry
        world.create_entity().with(Vel(2.0)).build();

        let mut dispatcher = DispatcherBuilder::new().with(SysTest, "sys_a", &[]).build();
        dispatcher.setup(&mut world);
        dispatcher.dispatch(&mut world);
    }
}