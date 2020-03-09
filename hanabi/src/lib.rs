#[macro_use]
extern crate gdnative;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate simple_logger;

use gdnative::*;

use std::thread;
use std::rc::Rc;
use std::cell::RefCell;

use specs::prelude::*;

mod sin_test;
mod hb_show;

use sin_test::SinDrawer;
use hb_show::{HbShow, SysTest};

struct A {
    em: RefCell<i32>
}

#[derive(gdnative::NativeClass)]
#[inherit(gdnative::Node)]
struct HelloWorld {
    show: HbShow,
}

#[gdnative::methods]
impl HelloWorld {
    fn _init(_owner: gdnative::Node) -> Self {
        simple_logger::init_with_level(log::Level::Info).expect("init simple logger");
        info!("simple_logger init");

        HelloWorld { show: HbShow::new() }
    }

    #[export]
    fn _ready(&mut self, _owner: gdnative::Node) {
        info!("hello world ready");
        self.show.setup();
    }


    #[export]
    fn /*_physics*/_process(&mut self, owner: Node, delta: f64) {
        // info!("py");
        self.show.dispatch();
    }
}



fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<SinDrawer>();
    handle.add_class::<HelloWorld>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();