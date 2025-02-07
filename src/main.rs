use bevy::{color::palettes::basic::PURPLE, prelude::*};

const TIME_STEP: f32 = 1. / 60.;
const SPEED_MULTIPLIER: f32 = 1.;

#[derive(Component)]
struct Player;

#[derive(Resource)]
struct PlayerState {
    on: bool,
    // last_shot: f64,
}
impl Default for PlayerState {
    fn default() -> Self {
        Self {
            on: false,
            // last_shot: 0.,
        }
    }
}
impl PlayerState {
    // fn shot(&mut self, time: f64) {
    //     self.on = false;
    //     self.last_shot = time
    // }

    fn spawned(&mut self) {
        self.on = true;
        // self.last_shot = 0.
    }
}

#[derive(Component)]
struct Speed(f32);
impl Default for Speed {
    fn default() -> Self {
        Self(500. * SPEED_MULTIPLIER)
    }
}

fn main() {
    App::new()
        .insert_resource(PlayerState::default())
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, player_spawn))
        .add_systems(Update, player_movement)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn player_spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut player_state: ResMut<PlayerState>,
) {
    commands
        .spawn((
            Mesh2d(meshes.add(Rectangle::default())),
            MeshMaterial2d(materials.add(Color::from(PURPLE))),
            Transform::default().with_scale(Vec3::splat(8.)),
        ))
        .insert(Player)
        .insert(Speed::default());
    player_state.spawned();
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
    mut query: Query<(&Speed, &mut Transform), With<Player>>,
) {
    if let Ok(window) = windows.get_single() {
        let width = window.resolution.width() / 2.;
        let height = window.resolution.height() / 2.;
        let dir_x = 1.;
        let dir_y = 0.;

        if let Ok((speed, mut transform)) = query.get_single_mut() {
            // dir_x = if keyboard_input.pressed(KeyCode::ArrowLeft) {
            //     -1.
            // } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            //     1.
            // } else {
            //     0.
            // };
            //
            // dir_y = if keyboard_input.pressed(KeyCode::ArrowUp) {
            //     1.
            // } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            //     -1.
            // } else {
            //     0.
            // };

            // println!("x: {}, y: {}", transform.translation.x, transform.translation.y);

            // playground boundaries, don't move outer this
            if transform.translation.x + (dir_x * speed.0 * TIME_STEP) < width - 8. 
                && transform.translation.x + (dir_x * speed.0 * TIME_STEP) > -width + 8. {
                transform.translation.x += dir_x * speed.0 * TIME_STEP;
            }

            if transform.translation.y + (dir_y * speed.0 * TIME_STEP) < height - 8.
                && transform.translation.y + (dir_y * speed.0 * TIME_STEP) > -height + 8. {
                transform.translation.y += dir_y * speed.0 * TIME_STEP;
            }
        }
    } else {
        println!("Error no window boundaries found.");
    }
}
