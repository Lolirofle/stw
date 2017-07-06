use amethyst::ecs::{VecStorage,Component};
use nalgebra::Vector2;
use ncollide::shape::ShapeHandle2;

use data::*;

pub struct Position(pub Vector2<f64>);
impl Component for Position{
	type Storage = VecStorage<Position>;
}

pub struct Collision{
	pub velocity      : Vector2<f64>,
	pub acceleration  : Vector2<f64>,
	pub shape         : ShapeHandle2<f64>,
	pub check_movement: bool,
}
impl Component for Collision{
	type Storage = VecStorage<Collision>;
}


pub struct CollisionCache{
	pub new_position: Option<Vector2<f64>>,
	pub new_velocity: Option<Vector2<f64>>,
}
impl CollisionCache{
	pub fn new() -> Self{CollisionCache{
		new_position: None,
		new_velocity: None,
	}}
}
impl Component for CollisionCache{
	type Storage = VecStorage<CollisionCache>;
}

pub struct Solid{
	pub typ: SolidType,
	pub friction: f64,
}
impl Component for Solid{
	type Storage = VecStorage<Solid>;
}

pub struct Player{
	pub id: u8,
	//pub state: MoverState,
}
impl Component for Player{
	type Storage = VecStorage<Player>;
}
