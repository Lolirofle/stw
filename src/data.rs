#![allow(dead_code)]

use nalgebra::Vector2;

#[derive(Copy,Clone,PartialEq)]
pub struct Camera{
	pub translate: Vector2<f64>,
	pub size     : Vector2<f64>,
}
impl Camera{
	pub fn new() -> Self{Camera{
		translate: Vector2::new(0.0,0.0),
		size     : Vector2::new(0.0,0.0),
	}}
}

#[derive(Copy,Clone,PartialEq)]
pub struct Score{
	pub points: u32,
	pub start_time: f64,
}
impl Score{
	pub fn new() -> Self{Score{
		points    : 0,
		start_time: 0.0,
	}}
}

/*#[derive(Copy,Clone,Eq,PartialEq,Hash)]
pub enum CollisionType{
	Static,
	Dynamic,
}*/

#[derive(Copy,Clone,Eq,PartialEq,Hash)]
pub enum SolidType{
	Solid,
	FallThrough,
}

/*#[derive(Copy,Clone,PartialEq)]
pub enum MoverState{
	OnGround{normal: Vector2<f64>},
	Ducking{normal: Vector2<f64>},
	Falling,
	Jumping,
}

#[derive(Copy,Clone,PartialEq)]
pub enum MoverMovement{
	Freely{direction: Vector2<f64>},
	Colliding{direction: Vector2<f64>,normal: Vector2<f64>},
}

#[derive(Copy,Clone,PartialEq)]
pub struct MoverData{
	pub state_start_time   : f64,
	pub movement_start_time: f64,
}
impl MoverData{
	pub fn new() -> Self{MoverData{
		state_start_time   : 0.0,
		movement_start_time: 0.0,
	}}
}*/
