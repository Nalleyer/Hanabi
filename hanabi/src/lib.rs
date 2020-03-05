use gdnative::*;

mod sin_test;
mod hb_show;

use sin_test::SinDrawer;
use hb_show::HanabiShow;

#[derive(gdnative::NativeClass)]
#[inherit(gdnative::Node)]
struct HelloWorld;

#[gdnative::methods]
impl HelloWorld {
    fn _init(_owner: gdnative::Node) -> Self {
        HelloWorld
    }

    #[export]
    fn _ready(&self, _owner: gdnative::Node) {
        godot_print!("hello, world.");

        HanabiShow::new().run_world();

        println!("rust log");
    }
}



fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<SinDrawer>();
    handle.add_class::<HelloWorld>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();