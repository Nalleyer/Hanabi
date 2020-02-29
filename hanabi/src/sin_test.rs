use gdnative::*;

const N: u32 = 1024;

#[derive(Debug, Clone, PartialEq)]
pub enum ManageErrs {
    CouldNotMakeInstance,
    RootClassNotSpatial(String),
}

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct SinDrawer {
    template: Option<PackedScene>,
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
        SinDrawer { template: None }
    }

    #[export]
    unsafe fn _ready(&mut self, owner: Node2D) {
        godot_print!("Hello World");
        self.template = load_scene("res://Item.tscn");
        match &self.template {
            Some(_scene) => {
                godot_print!("Loaded child scene successfully!");
                self.spawn_all(owner);
            }
            None => godot_print!("Could not load child scene. Check name."),
        }
    }

    unsafe fn spawn_all(&mut self, mut owner: Node2D) {
        let template = if let Some(template) = &self.template {
            template
        } else {
            godot_print!("Cannot spawn a child because we couldn't load the template scene");
            return;
        };


        for i in 1..N {
            match instance_scene::<Node2D>(&template) {
                Ok(mut item) => {
                    item.set_name(GodotString::from_str(format!("{}", i)));

                    owner.add_child(Some(item.to_node()), false);
                }
                Err(err) => godot_print!("Could not instance Child : {:?}", err),
            }
        }
    }
}
