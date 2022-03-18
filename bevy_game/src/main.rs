use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

pub struct HelloPlugin;

fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("Elaina Proctor".to_string()));
    commands.spawn().insert(Person).insert(Name("Renzo Hume".to_string()));
    commands.spawn().insert(Person).insert(Name("Zayna Nieves".to_string()));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("hello {}!", name.0);
    }
}

fn hello_world() {
    println!("Hello World")
}

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_people)
            .add_system(hello_world)
            .add_system(greet_people);
    }
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // These are the plugins defaults use
       //.add_plugin(CorePlugin::default())
       // .add_plugin(InputPlugin::default())
       // .add_plugin(WindowPlugin::default())
       // Can implment these indiv or can put it in a plugin
       // .add_startup_system(add_people)
       // .add_system(hello_world)
       // .add_system(greet_people)
        .add_plugin(HelloPlugin)
        .run();
}
