use amethyst::ecs::{VecStorage,Component};
use nalgebra::Vector2;

pub struct Position(pub Vector2<f32>);
impl Component for Position{
	type Storage = VecStorage<Position>;
}

pub struct Velocity(pub Vector2<f32>);
impl Component for Velocity{
	type Storage = VecStorage<Velocity>;
}

pub struct Collision(pub usize);
impl Component for Collision{
	type Storage = VecStorage<Collision>;
}

pub struct Ball{
	pub size: f32,
}
impl Component for Ball{
	type Storage = VecStorage<Ball>;
}

pub struct Plank{
	pub dimensions: Vector2<f32>,
	pub side: ::data::Side,
}
impl Component for Plank{
	type Storage = VecStorage<Plank>;
}
