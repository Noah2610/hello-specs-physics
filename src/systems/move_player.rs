use super::system_prelude::*;

const SPEED: (f32, f32) = (1000.0, 1000.0);
const MAX_VEL: (f32, f32) = (500.0, 500.0);

#[derive(Default)]
pub struct MovePlayer;

impl<'a> System<'a> for MovePlayer {
    type SystemData = (
        Read<'a, Time>,
        Read<'a, InputManager<Bindings>>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, DecreaseVelocity>,
    );

    fn run(
        &mut self,
        (
            time,
            input_manager,
            players,
            mut velocities,
            mut decrease_velocities,
        ): Self::SystemData,
    ) {
        if let Some((_, player_velocity, player_decr_velocity)) =
            (&players, &mut velocities, &mut decrease_velocities)
                .join()
                .next()
        {
            let dt = time.delta_seconds();
            // Player movement
            if let Some(x) = input_manager.axis_value(AxisBinding::PlayerX) {
                if x != 0.0 {
                    player_velocity
                        .increase_x_with_max(x * SPEED.0 * dt, Some(MAX_VEL.0));
                    match x.signum() as i8 {
                        -1 => player_decr_velocity.dont_decrease_x_when_neg(),
                        1 => player_decr_velocity.dont_decrease_x_when_pos(),
                        _ => (),
                    }
                }
            }
            if let Some(y) = input_manager.axis_value(AxisBinding::PlayerY) {
                if y != 0.0 {
                    player_velocity
                        .increase_y_with_max(y * SPEED.1 * dt, Some(MAX_VEL.1));
                    match y.signum() as i8 {
                        -1 => player_decr_velocity.dont_decrease_y_when_neg(),
                        1 => player_decr_velocity.dont_decrease_y_when_pos(),
                        _ => (),
                    }
                }
            }
        }
    }
}
