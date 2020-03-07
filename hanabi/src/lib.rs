#[macro_use]
extern crate gdnative;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate simple_logger;

use gdnative::*;

mod sin_test;
mod hb_show;

use sin_test::SinDrawer;
use hb_show::HbShow;

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
        HbShow::new().run_world();
    }
}



fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<SinDrawer>();
    handle.add_class::<HelloWorld>();

    simple_logger::init_with_level(log::Level::Info).unwrap();
    info!("simple_logger init");
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();