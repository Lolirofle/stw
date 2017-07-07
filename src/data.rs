#[derive(Copy,Clone,PartialEq)]
pub struct Score{
	pub points: u32,
	pub time: f64,
}

#[derive(Copy,Clone,Eq,PartialEq,Hash)]
pub enum CollisionType{
	Static,
	Dynamic,
}

#[derive(Copy,Clone,Eq,PartialEq,Hash)]
pub enum SolidType{
	Solid,
	FallThrough,
}

#[derive(Copy,Clone,Eq,PartialEq,Hash)]
pub enum MoverState{
	Standing,
	Falling,
	MovingFreely,
	MovingColliding,
	Jumping,
}
