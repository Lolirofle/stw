use nalgebra::{Isometry2,Point2,Vector2,dot};
use ncollide::narrow_phase::{ProximityHandler,ContactHandler,ContactAlgorithm2};
use ncollide::query::{Contact,Proximity};
use ncollide::world::{CollisionWorld2,CollisionGroups,CollisionObject2};
use std::cell::Cell;

pub struct Collision{
	pub world  : CollisionWorld2<f32,ObjectData>,
	pub next_id: usize,
	pub group  : CollisionGroups,
}
impl Collision{
	pub fn new() -> Self{Collision{
		world          : CollisionWorld2::new(0.02,true),
		next_id        : 0,
		group          : CollisionGroups::new(), //Every object is part of this group and interacts with everything
	}}
}

#[derive(Clone)]
pub struct ObjectData{
	pub position: Cell<Vector2<f32>>,
	pub velocity: Cell<Vector2<f32>>,
}
impl Default for ObjectData{
	fn default() -> Self{ObjectData{
		position: Cell::new(Vector2::new(0.0,0.0)),
		velocity: Cell::new(Vector2::new(0.0,0.0)),
	}}
}

pub struct ProximityMessage;
impl ProximityHandler<Point2<f32>,Isometry2<f32>,ObjectData> for ProximityMessage{
	fn handle_proximity(&mut self,co1: &CollisionObject2<f32,ObjectData>,co2: &CollisionObject2<f32,ObjectData>,_: Proximity,new_proximity: Proximity){
		if new_proximity == Proximity::Intersecting{
			//println!("Intersection start: {:?} , {:?}",co1.position,co2.position);
		}else if new_proximity == Proximity::Disjoint{
			//println!("Intersection stop: {:?} , {:?}",co1.position,co2.position);
		}
	}
}

pub struct VelocityBouncer{
	pub tmp_collector: Vec<Contact<Point2<f32>>>
}
impl VelocityBouncer{
	pub fn new() -> Self{
		VelocityBouncer{
			tmp_collector: Vec::new()
		}
	}
}
impl ContactHandler<Point2<f32>, Isometry2<f32>,ObjectData> for VelocityBouncer{
	fn handle_contact_started(&mut self,co1: &CollisionObject2<f32,ObjectData>,co2: &CollisionObject2<f32,ObjectData>,alg: &ContactAlgorithm2<f32>){
		self.tmp_collector.clear();
		alg.contacts(&mut self.tmp_collector);

		//println!("Contact start: {:?} {:?} {:?}",co1.position,co2.position,self.tmp_collector);

		{
			let normal = self.tmp_collector[0].normal;
			co1.data.velocity.set(co1.data.velocity.get() - 2.0*dot(&co1.data.velocity.get(),&normal)*normal);
		}{
			let normal = -self.tmp_collector[0].normal;
			co2.data.velocity.set(co2.data.velocity.get() - 2.0*dot(&co2.data.velocity.get(),&normal)*normal);
		}
	}

	fn handle_contact_stopped(&mut self,co1: &CollisionObject2<f32,ObjectData>,co2: &CollisionObject2<f32,ObjectData>){
		//println!("Contact stop: {:?} {:?}",co1.position,co2.position);
	}
}
