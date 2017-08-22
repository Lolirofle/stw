use amethyst::ecs::{HashMapStorage,VecStorage,Component};
use nalgebra::Vector2;
use ncollide::shape::ShapeHandle2;

use data::*;

pub struct Position(pub Vector2<f64>);
impl Component for Position{
	type Storage = VecStorage<Position>;
}

pub struct CollisionCache{
	//Values to change to after collision checking (Temporary storage)
	pub new_position    : Option<Vector2<f64>>,
	pub new_velocity    : Option<Vector2<f64>>,

	//Acceleration in the previous step
	pub old_acceleration: Vector2<f64>,
}
impl CollisionCache{
	pub fn new() -> Self{CollisionCache{
		new_position    : None,
		new_velocity    : None,
		old_acceleration: Vector2::new(0.0,0.0),
	}}
}
impl Component for CollisionCache{
	type Storage = VecStorage<CollisionCache>;
}

pub struct Solid{
	pub typ           : SolidType,
	pub friction      : f64,
	pub velocity      : Vector2<f64>,
	pub acceleration  : Vector2<f64>,//TODO: Consider having a function that calculates acceleration instead from all its components (but I cannot find a way to implement it organized). An alternative could be to have a temporary acceleration variable for each step.
	pub shape         : ShapeHandle2<f64>,
	pub check_movement: bool,
	pub gravity       : bool,
}
impl Component for Solid{
	type Storage = VecStorage<Solid>;
}

pub struct Player{
	pub id: u8,
	//pub state: MoverState,
}
impl Component for Player{
	type Storage = HashMapStorage<Player>;
}
