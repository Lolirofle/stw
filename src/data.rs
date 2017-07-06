pub struct Score{
	pub points: u32,
	pub time: f64,
}

#[derive(Eq,PartialEq)]
pub enum CollisionType{
	Static,
	Dynamic,
}

pub enum SolidType{
	Solid,
	FallThrough,
}
