use gdnative::*;
use std::marker::PhantomData;
use std::f32::consts::PI;

const N: u32 = 2048;

#[derive(Debug, Clone, PartialEq)]
pub enum ManageErrs {
    CouldNotMakeInstance,
    RootClassNotSpatial(String),
}

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct SinDrawer {
    template: Option<PackedScene>,
    start_time: i64,
}

unsafe impl Send for SinDrawer {}

pub fn load_scene(path: &str) -> Option<PackedScene> {
    let scene = ResourceLoader::godot_singleton().load(
        GodotString::from_str(path), // could also use path.into() here
        GodotString::from_str("PackedScene"),
        false,
    );

    scene.and_then(|s| s.cast::<PackedScene>())
}

unsafe fn instance_scene<Root>(scene: &PackedScene) -> Result<Root, ManageErrs>
where
    Root: gdnative::GodotObject,
{
    let inst_option = scene.instance(0); // 0 - GEN_EDIT_STATE_DISABLED

    if let Some(instance) = inst_option {
        if let Some(instance_root) = instance.cast::<Root>() {
            Ok(instance_root)
        } else {
            Err(ManageErrs::RootClassNotSpatial(
                instance.get_name().to_string(),
            ))
        }
    } else {
        Err(ManageErrs::CouldNotMakeInstance)
    }
}

#[methods]
impl SinDrawer {
    fn _init(_owner: Node2D) -> Self {
        SinDrawer { template: None, start_time: OS::godot_singleton().get_system_time_msecs() }
    }

    #[export]
    unsafe fn _ready(&mut self, owner: Node2D) {
        info!("sin ready");
        self.template = load_scene("res://Item.tscn");
        match &self.template {
            Some(_scene) => {
                info!("Loaded child scene successfully!");
                self.spawn_all(owner);
            }
            None => warn!("sin Could not load child scene. Check name."),
        }
    }

    #[export]
    unsafe fn _process(&mut self, owner: Node2D, _delta: f64) {
        self.update_fps_label(owner);
        // return;
        let t = OS::godot_singleton().get_system_time_msecs() - self.start_time;
        let tt = t as f64 / 100f64;
        let count = owner.get_child_count();
        for i in 1..count {
            if let Some(child) = owner.get_child(i - 1) {
                let mut item: Node2D = child.cast().unwrap();
                let x_: f32 = tt as f32 + (i as f32 / N as f32) * 2f32 * PI;
                let y = x_.sin() * 500f32 + 500f32;
                item.set_position(geom::Vector2{x: (i * 2) as f32, y : y, _unit: PhantomData});
            }
        }
    }


    unsafe fn update_fps_label(&self, owner: Node2D) {
        if let Some(node) = owner.get_parent().and_then(|p| p.find_node(GodotString::from_str("FPS"), true, false)) {
            let mut label: Label = node.cast().unwrap();
            label.set_text(GodotString::from_str(format!("fps: {}", Engine::godot_singleton().get_frames_per_second())));
        }
    }

    unsafe fn spawn_all(&mut self, mut owner: Node2D) {
        let template = if let Some(template) = &self.template {
            template
        } else {
            warn!("Cannot spawn a child because we couldn't load the template scene");
            return;
        };


        for i in 1..N {
            match instance_scene::<Node2D>(&template) {
                Ok(mut item) => {
                    item.set_name(GodotString::from_str(format!("{}", i)));

                    owner.add_child(Some(item.to_node()), false);
                }
                Err(err) => warn!("Could not instance Child : {:?}", err),
            }
        }
    }
}
