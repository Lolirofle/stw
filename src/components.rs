use amethyst::ecs::{VecStorage,Component};
use nalgebra::Vector2;

pub struct Position(pub Vector2<f64>);
impl Component for Position{
	type Storage = VecStorage<Position>;
}

pub struct Collision{
	pub id          : usize,
	pub velocity    : Vector2<f64>,
	pub acceleration: Vector2<f64>,
}
impl Component for Collision{
	type Storage = VecStorage<Collision>;
}

pub struct Ball{
	pub size: f64,
}
impl Component for Ball{
	type Storage = VecStorage<Ball>;
}

pub struct Plank{
	pub dimensions: Vector2<f64>,
	pub side      : ::data::Side,
}
impl Component for Plank{
	type Storage = VecStorage<Plank>;
}
