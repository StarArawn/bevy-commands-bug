use bevy::prelude::*;
use bevy::ecs::system::SystemParam;

#[derive(SystemParam)]
pub struct MySystemParam<'a> {
    commands: Commands<'a>,
}

impl<'a> MySystemParam<'a> {
    pub fn create_sprite(&mut self, material: Handle<ColorMaterial>) {
        self.commands.spawn_bundle(SpriteBundle {
            material,
            ..Default::default()
        });
    }
}


fn startup_working(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let texture_handle = asset_server.load("icon.png");
    let material_handle = materials.add(ColorMaterial::texture(texture_handle));

    commands.spawn_bundle(SpriteBundle {
        material: material_handle,
        ..Default::default()
    });
}

fn startup_broken(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut my_system_param: MySystemParam,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let texture_handle = asset_server.load("icon.png");
    let material_handle = materials.add(ColorMaterial::texture(texture_handle));

    my_system_param.create_sprite(material_handle)
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            width: 1270.0,
            height: 720.0,
            title: String::from("Commands Bug"),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup_broken.system()) // Change this to startup_working to see the working example.
        .run();
}
