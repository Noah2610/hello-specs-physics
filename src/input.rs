pub mod prelude {
    pub use super::ActionBinding;
    pub use super::AxisBinding;
    pub use super::Bindings;
}

use amethyst::input::{BindingTypes, InputBundle};

#[derive(Debug)]
pub struct Bindings;
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum AxisBinding {}
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionBinding {}

impl BindingTypes for Bindings {
    type Axis = AxisBinding;
    type Action = ActionBinding;
}

pub fn input_bundle() -> InputBundle<Bindings> {
    InputBundle::<Bindings>::new()
}
