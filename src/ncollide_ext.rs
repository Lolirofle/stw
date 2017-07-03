/*use amethyst::ecs::{Index,UnprotectedStorage};
use ncollide::utils::data::uid_remap::UidRemap;
use ncollide::shape::ShapeHandle2;
use ncollide::world::CollisionObject2;
use std::mem;

use components;

//TODO: I am trying to make UidRemap a valid storage for specs because everything in ncollide needs UidRemaps for everything (e.g. NarrowPhase.update). CollisionObject2<f64,()> must be clone for this to work though

pub struct Object(pub CollisionObject2<f64,()>); //TODO: Would be good to guarantee the same memory layout as the inner stuff (See RFC 1758 #[repr(transparent)])
impl Component for Object{
	type Storage = ObjectUidRemapStorage;
}

pub struct ObjectUidRemapStorage(pub UidRemap<CollisionObject2<f64,()>>);

impl UnprotectedStorage<CollisionObject2<f64,()>> for ObjectUidRemapStorage{
	fn new() -> Self {
		ObjectUidRemapStorage(UidRemap::new(true))
	}

	unsafe fn clean<F>(&mut self, has: F) where
		F: Fn(Index) -> bool
	{
		unimplemented!()
	}

	unsafe fn get(&self, id: Index) -> &CollisionObject2<f64,()>{
		self.0.get(id as usize).unwrap()
	}

	unsafe fn get_mut(&mut self, id: Index) -> &mut CollisionObject2<f64,()>{
		self.0.get_mut(id as usize).unwrap()
	}

	unsafe fn insert(&mut self, id: Index, v: CollisionObject2<f64,()>){
		self.0.update(id as usize,v);
	}

	unsafe fn remove(&mut self, id: Index) -> CollisionObject2<f64,()>{
		self.0.remove(id as usize).unwrap()
	}
}

impl UnprotectedStorage<components::Object> for ObjectUidRemapStorage{
	#[inline(always)]
	fn new() -> Self {
		<ObjectUidRemapStorage as UnprotectedStorage<CollisionObject2<f64,()>>>::new()
	}

	#[inline(always)]
	unsafe fn clean<F>(&mut self, has: F) where
		F: Fn(Index) -> bool
	{
		<ObjectUidRemapStorage as UnprotectedStorage<CollisionObject2<f64,()>>>::clean(self,has)
	}

	#[inline(always)]
	unsafe fn get(&self, id: Index) -> &components::Object{
		mem::transmute(<ObjectUidRemapStorage as UnprotectedStorage<CollisionObject2<f64,()>>>::get(self,id))
	}

	#[inline(always)]
	unsafe fn get_mut(&mut self, id: Index) -> &mut components::Object{
		mem::transmute(<ObjectUidRemapStorage as UnprotectedStorage<CollisionObject2<f64,()>>>::get_mut(self,id))
	}

	#[inline(always)]
	unsafe fn insert(&mut self, id: Index, v: components::Object){
		<ObjectUidRemapStorage as UnprotectedStorage<CollisionObject2<f64,()>>>::insert(self,id,v.0)
	}

	#[inline(always)]
	unsafe fn remove(&mut self, id: Index) -> components::Object{
		components::Object(<ObjectUidRemapStorage as UnprotectedStorage<CollisionObject2<f64,()>>>::remove(self,id))
	}
}*/
