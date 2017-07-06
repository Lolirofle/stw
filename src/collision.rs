use nalgebra::{Isometry2,Point2,Vector2,dot,norm};
use ncollide::narrow_phase::{ProximityHandler,ContactHandler,ContactAlgorithm2};
use ncollide::query::{Contact,Proximity};
use ncollide::world::{CollisionWorld2,CollisionGroups,CollisionObject2};
use std::cell::Cell;

pub struct Collision{
	pub world  : CollisionWorld2<f64,ObjectData>,
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
pub enum ObjectType{
	Bounce,
	Still,
}

#[derive(Clone)]
pub struct ObjectData{
	pub velocity    : Cell<Vector2<f64>>,
	pub acceleration: Cell<Vector2<f64>>,
	pub typ         : ObjectType,
}
impl Default for ObjectData{
	fn default() -> Self{ObjectData{
		velocity    : Cell::new(Vector2::new(0.0,0.0)),
		acceleration: Cell::new(Vector2::new(0.0,160.0)),
		typ         : ObjectType::Still,
	}}
}

pub struct ProximityMessage;
impl ProximityHandler<Point2<f64>,Isometry2<f64>,ObjectData> for ProximityMessage{
	fn handle_proximity(&mut self,co1: &CollisionObject2<f64,ObjectData>,co2: &CollisionObject2<f64,ObjectData>,_: Proximity,new_proximity: Proximity){
		if new_proximity == Proximity::Intersecting{
			//println!("Intersection start: {:?} , {:?}",co1.position,co2.position);
		}else if new_proximity == Proximity::Disjoint{
			//println!("Intersection stop: {:?} , {:?}",co1.position,co2.position);
		}
	}
}

pub struct VelocityBouncer{
	pub tmp_collector: Vec<Contact<Point2<f64>>>
}
impl VelocityBouncer{
	pub fn new() -> Self{
		VelocityBouncer{
			tmp_collector: Vec::new()
		}
	}
}
impl ContactHandler<Point2<f64>, Isometry2<f64>,ObjectData> for VelocityBouncer{
	fn handle_contact_started(&mut self,co1: &CollisionObject2<f64,ObjectData>,co2: &CollisionObject2<f64,ObjectData>,alg: &ContactAlgorithm2<f64>){
		self.tmp_collector.clear();
		alg.contacts(&mut self.tmp_collector);

		//println!("Contact start: {:?} {:?} {:?}",co1.position,co2.position,self.tmp_collector);

		{
			let normal = self.tmp_collector[0].normal;
			let vel = co1.data.velocity.get();
			match co1.data.typ{
				ObjectType::Bounce => {co1.data.velocity.set(vel - 2.0*dot(&vel,&normal)*normal);}
				ObjectType::Still  => {co1.data.velocity.set(vel - dot(&vel,&normal)*normal);}
			}
		}{
			let normal = -self.tmp_collector[0].normal;
			let vel = co2.data.velocity.get();
			match co2.data.typ{
				ObjectType::Bounce => {co2.data.velocity.set(vel - 2.0*dot(&vel,&normal)*normal);}
				ObjectType::Still  => {co2.data.velocity.set(vel - dot(&vel,&normal)*normal);}
			}
		}
	}

	fn handle_contact_stopped(&mut self,co1: &CollisionObject2<f64,ObjectData>,co2: &CollisionObject2<f64,ObjectData>){
		//println!("Contact stop: {:?} {:?}",co1.position,co2.position);
	}
}
