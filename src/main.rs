use bevy::{color::palettes::basic::PURPLE, prelude::*};

const TIME_STEP: f32 = 0.05 / 60.;
const SPEED_MULTIPLIER: f32 = 1.;
const BORDER_SPACE: f32 = 8.;

#[derive(Component)]
struct Player;

#[derive(Resource)]
struct PlayerState {
    on: bool,
    dir_x: f32,
    dir_y: f32,
    // last_shot: f64,
}
impl Default for PlayerState {
    fn default() -> Self {
        Self {
            on: false,
            dir_x: 1.,
            dir_y: 0.,
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

    fn changed_direction(&mut self, dir_x: f32, dir_y: f32) -> bool {
        // check if direction changed
        if self.dir_x == dir_x && self.dir_y == dir_y {
            return false;
        }

        self.dir_x = dir_x;
        self.dir_y = dir_y;
        true
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
    mut player_state: ResMut<PlayerState>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
    mut query: Query<(&Speed, &mut Transform), With<Player>>,
) {
    if let Ok(window) = windows.get_single() {
        let width = window.resolution.width() / 2.;
        let height = window.resolution.height() / 2.;

        if let Ok((speed, mut transform)) = query.get_single_mut() {
            let mut changed_dir = false;

            if keyboard_input.pressed(KeyCode::ArrowLeft) {
                changed_dir = player_state.changed_direction(-1., 0.);
            } else if keyboard_input.pressed(KeyCode::ArrowRight) {
                changed_dir = player_state.changed_direction(1., 0.);
            };

            if keyboard_input.pressed(KeyCode::ArrowUp) {
                changed_dir = player_state.changed_direction(0., 1.);
            } else if keyboard_input.pressed(KeyCode::ArrowDown) {
                changed_dir = player_state.changed_direction(0., -1.);
            };

            // println!("x: {}, y: {}", transform.translation.x, transform.translation.y);
            // idea: compare with last direction
            // last direction must not be the same as next direction

            // playground boundaries, don't move outer this
            if transform.translation.x + (player_state.dir_x * speed.0 * TIME_STEP) < width - BORDER_SPACE 
                && transform.translation.x + (player_state.dir_x * speed.0 * TIME_STEP) > -width + BORDER_SPACE {
                transform.translation.x += if changed_dir { player_state.dir_x } else { player_state.dir_x * speed.0 * TIME_STEP };
            }

            if transform.translation.y + (player_state.dir_y * speed.0 * TIME_STEP) < height - BORDER_SPACE
                && transform.translation.y + (player_state.dir_y * speed.0 * TIME_STEP) > -height + BORDER_SPACE {
                transform.translation.y += if changed_dir { player_state.dir_y } else { player_state.dir_y * speed.0 * TIME_STEP };
            }
        }
    } else {
        println!("Error no window boundaries found.");
    }
}
