pub mod defines;
pub mod raycaster;
pub mod world;
pub mod renderer;
pub mod math;
pub mod utilities;
pub mod camera;
pub mod color;

pub mod prelude {
    pub use crate::raycaster::Raycaster;
    pub use crate::raycaster::MapData;
    pub use crate::world::World;
    pub use crate::math;
    pub use crate::utilities;
}
