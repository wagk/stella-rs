use bevy::prelude::*;

#[derive(Debug, Component)]
struct Bits {}

#[derive(Debug, Component)]
struct Tids {}

fn bits_of_a_system(mut commands: Commands) {
    commands.spawn().insert(Tids {}).insert(Bits {});
}

struct TimeClock(Timer);

fn see_bits_of_a_system(
    time: Res<Time>,
    mut clock: ResMut<TimeClock>,
    query: Query<&Tids, With<Bits>>,
) {
    if clock.0.tick(time.delta()).just_finished() {
        for thing in query.iter() {
            println!("{:?}", thing);
        }
    }
}

struct Tidbit;

impl Plugin for Tidbit {
    fn build(&self, app: &mut App) {
        app.insert_resource(TimeClock(Timer::from_seconds(2.0, true)))
            .add_startup_system(bits_of_a_system)
            .add_system(see_bits_of_a_system);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(Tidbit)
        .run();
}
