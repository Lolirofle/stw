use amethyst::ecs::{HashMapStorage,VecStorage,Component};
use nalgebra::{Vector2,zero};
use ncollide::shape::ShapeHandle2;

use data::*;

pub struct Position(pub Vector2<f64>);
impl Component for Position{
	type Storage = VecStorage<Position>;
}

pub struct CollisionCache{
	pub position_resolve: Vector2<f64>,
	pub velocity_resolve: Vector2<f64>,
	pub friction_total  : f64,
}
impl CollisionCache{
	pub fn new() -> Self{CollisionCache{
		position_resolve: zero(),
		velocity_resolve: zero(),
		friction_total  : zero(),
	}}
}
impl Component for CollisionCache{
	type Storage = VecStorage<CollisionCache>;
}

pub struct Solid{
	pub typ           : SolidType,
	pub check_movement: bool,
	pub gravity       : bool,
	pub friction      : f64,
	pub shape         : ShapeHandle2<f64>,

	//Movement data
	pub velocity      : Vector2<f64>,
	pub acceleration  : Vector2<f64>,//TODO: Consider having a function that calculates acceleration instead from all its components (but I cannot find a way to implement it organized). An alternative could be to have a temporary acceleration variable for each step.

	//Movement data from the previous step
	pub old_position    : Vector2<f64>, //TODO: This may not be neccessary
	pub old_velocity    : Vector2<f64>,
	pub old_acceleration: Vector2<f64>,
}
impl Solid{
	pub fn new(
		typ           : SolidType,
		check_movement: bool,
		gravity       : bool,
		friction      : f64,
		shape         : ShapeHandle2<f64>,
	) -> Self{Solid{
		typ           : typ,
		check_movement: check_movement,
		gravity       : gravity,
		friction      : friction,
		shape         : shape,

		velocity    : zero(),
		acceleration: zero(),

		old_position    : zero(),
		old_velocity    : zero(),
		old_acceleration: zero(),
	}}
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
