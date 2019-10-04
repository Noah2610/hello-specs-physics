use super::system_prelude::*;

const SPEED: (f32, f32) = (100.0, 100.0);
const MAX_VEL: (f32, f32) = (500.0, 500.0);

#[derive(Default)]
pub struct MovePlayer;

impl<'a> System<'a> for MovePlayer {
    type SystemData = (
        Read<'a, Time>,
        Read<'a, InputManager<Bindings>>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Velocity>,
    );

    fn run(
        &mut self,
        (time, input_manager, mut players, mut velocities): Self::SystemData,
    ) {
        if let Some((player, player_velocity)) =
            (&mut players, &mut velocities).join().next()
        {
            let dt = time.delta_seconds();
            // Player movement
            if let Some(x) = input_manager.axis_value(AxisBinding::PlayerX) {
                if x != 0.0 {
                    dbg!("move X {}", x);
                    player_velocity
                        .increase_x_with_max(x * SPEED.0 * dt, Some(MAX_VEL.0));
                }
            }
            if let Some(y) = input_manager.axis_value(AxisBinding::PlayerY) {
                if y != 0.0 {
                    dbg!("move Y {}", y);
                    player_velocity
                        .increase_y_with_max(y * SPEED.1 * dt, Some(MAX_VEL.1));
                }
            }
        }
    }
}
