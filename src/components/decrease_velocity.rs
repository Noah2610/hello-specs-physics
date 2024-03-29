use super::component_prelude::*;

/// Entities, which have `Velocity` and `DecreaseVelocity`, will decrease their velocity
/// every frame, by a certain amount (`x`, `y`).
/// Note that, the given `x` and `y` fields must _always_ be positive.
#[derive(Serialize, Deserialize)]
pub struct DecreaseVelocity {
    pub x: f32,
    pub y: f32,
    /// Should decrease X velocity, when X velocity is POSITIVE
    #[serde(default)]
    pub(crate) should_decrease_x_pos: bool,
    /// Should decrease X velocity, when X velocity is NEGATIVE
    #[serde(default)]
    pub(crate) should_decrease_x_neg: bool,
    /// Should decrease Y velocity, when X velocity is POSITIVE
    #[serde(default)]
    pub(crate) should_decrease_y_pos: bool,
    /// Should decrease Y velocity, when X velocity is NEGATIVE
    #[serde(default)]
    pub(crate) should_decrease_y_neg: bool,
}

impl DecreaseVelocity {
    /// Create a new `DecreaseVelocity` component with the given `x` and `y` values.
    /// `x` and `y` _must_ be positive.
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            should_decrease_x_pos: true,
            should_decrease_x_neg: true,
            should_decrease_y_pos: true,
            should_decrease_y_neg: true,
        }
    }

    /// Call this method, when you don't want this entity to decrease
    /// its `x` velocity at all, in the next frame.
    pub fn dont_decrease_x(&mut self) {
        self.dont_decrease_x_when_pos();
        self.dont_decrease_x_when_neg();
    }

    /// Call this method, when you don't want this entity to decrease
    /// its `y` velocity at all, in the next frame.
    pub fn dont_decrease_y(&mut self) {
        self.dont_decrease_y_when_pos();
        self.dont_decrease_y_when_neg();
    }

    /// Call this method, when you don't want this entity to decrease
    /// its `x` velocity, when the `x` velocity is _positive_, in the next frame.
    pub fn dont_decrease_x_when_pos(&mut self) {
        self.should_decrease_x_pos = false;
    }

    /// Call this method, when you don't want this entity to decrease
    /// its `x` velocity, when the `x` velocity is _negative_, in the next frame.
    pub fn dont_decrease_x_when_neg(&mut self) {
        self.should_decrease_x_neg = false;
    }

    /// Call this method, when you don't want this entity to decrease
    /// its `y` velocity, when the `y` velocity is _positive_, in the next frame.
    pub fn dont_decrease_y_when_pos(&mut self) {
        self.should_decrease_y_pos = false;
    }

    /// Call this method, when you don't want this entity to decrease
    /// its `y` velocity, when the `y` velocity is _positive_, in the next frame.
    pub fn dont_decrease_y_when_neg(&mut self) {
        self.should_decrease_y_neg = false;
    }
}

impl Component for DecreaseVelocity {
    type Storage = VecStorage<Self>;
}

impl Default for DecreaseVelocity {
    fn default() -> Self {
        Self {
            x:                     0.0,
            y:                     0.0,
            should_decrease_x_pos: true,
            should_decrease_x_neg: true,
            should_decrease_y_pos: true,
            should_decrease_y_neg: true,
        }
    }
}
