use amethyst::ecs::{Join, RunArg, System};
use std::ops::Deref;
use amethyst::ecs::Gate;

use components;

pub struct InputSystem;
unsafe impl Sync for InputSystem {}
impl System<()> for InputSystem {
	fn run(&mut self, arg: RunArg, _: ()) {
		use amethyst::ecs::resources::InputHandler;
		use amethyst::VirtualKeyCode;

		let (collisions, players, input) = arg.fetch(|w| {(
			w.write::<components::Collision>(),
			w.write::<components::Player>(),
			w.read_resource::<InputHandler>(),
		)});

		let mut players = players.pass();
		let mut collisions = collisions.pass();
		for(
			ref mut player,
			&mut components::Collision{ref mut velocity,..},
		) in (
			&mut players,
			&mut collisions,
		).join(){
			match player.id{
				1 =>{
					if input.key_down(VirtualKeyCode::W){
						velocity[1] = -200.0;
					}
					if input.key_down(VirtualKeyCode::A){
						velocity[0] = -100.0;
					}
					if input.key_down(VirtualKeyCode::D){
						velocity[0] = 100.0;
					}
				}
				0 =>{
					if input.key_down(VirtualKeyCode::Up){
						velocity[1] = -200.0;
					}
					if input.key_down(VirtualKeyCode::Left){
						velocity[0] = -100.0;
					}
					if input.key_down(VirtualKeyCode::Right){
						velocity[0] = 100.0;
					}
				}
				_ => {}
			};
		}
	}
}

pub struct RenderSystem;
unsafe impl Sync for RenderSystem {}
impl System<()> for RenderSystem {

	fn run(&mut self, arg: RunArg, _: ()) {
		use amethyst::ecs::components::LocalTransform;
		use nalgebra::Isometry2;

		let (collisions, positions, locals) = arg.fetch(|w| {(
			w.read::<components::Collision>(),
			w.read::<components::Position>(),
			w.write::<LocalTransform>()
		)});

		let collisions = collisions.pass();
		let positions = positions.pass();
		let mut locals = locals.pass();

		for(
			&components::Position(position),
			&components::Collision{ref shape,..},
			ref mut local
		) in (
			&positions,
			&collisions,
			&mut locals
		).join(){
			//Update the renderable corresponding to this entity
			let aabb = shape.aabb(&Isometry2::new(position,0.0));
			let mins = aabb.center();
			let len  = aabb.maxs() - aabb.mins();
			local.translation[0] = mins[0] as f32;
			local.translation[1] = mins[1] as f32;
			local.scale = [len[0] as f32, len[1] as f32, 1.0];
		}
	}
}

pub struct PhysicsSystem;
unsafe impl Sync for PhysicsSystem {}
impl System<()> for PhysicsSystem {

	fn run(&mut self, arg: RunArg, _: ()) {
		use alga::general::AbstractModule;
		use amethyst::ecs::resources::Time;
		use nalgebra::{Isometry2,dot,zero};

		use util;

		let (
			collisions,
			collision_caches,
			positions,
			solids,
			time,
		) = arg.fetch(|w| {(
			w.write::<components::Collision>(),
			w.write::<components::CollisionCache>(),
			w.write::<components::Position>(),
			w.read::<components::Solid>(),
			w.read_resource::<Time>(),
		)});

		let delta_time = time.delta_time.subsec_nanos() as f64 / 1.0e9;

		let mut collisions       = collisions.pass();
		let mut collision_caches = collision_caches.pass();
		let mut positions        = positions.pass();
		let     solids           = solids.pass();

		//Process velocity from acceleration
		for(
			&mut components::Collision{ref mut velocity,ref mut acceleration,..},
		) in (
			&mut collisions,
		).join(){
			*velocity+= acceleration.multiply_by(delta_time);//TODO
		}

		//Process collision checking
		for(
			&components::Position(position),
			&components::Collision{mut velocity,ref shape,check_movement,..},
			&mut components::CollisionCache{ref mut new_position,ref mut new_velocity},
		) in (
			&positions,
			&collisions,
			&mut collision_caches,
		).join(){
			for(
				&components::Position(position2),
				&components::Collision{velocity: velocity2,shape: ref shape2,..},
				&components::Solid{friction,..},
			) in (
				&positions,
				&collisions,
				&solids,
			).join(){
				//Skip collision with itself
				if (shape as *const _)==(shape2 as *const _){
					continue;
				}

				//Friction
				velocity = util::vector_lengthen(velocity,-120.0*delta_time);//TODO

				//If this is not a static object (no collision checking) and it made contact to something
				if let (true,Some(contact)) = (check_movement,::ncollide::query::contact(
					&Isometry2::new(position + velocity.multiply_by(delta_time),zero()),
					shape.deref(),
					&Isometry2::new(position2 + velocity2.multiply_by(delta_time),zero()),
					shape2.deref(),
					0.0
				)){
					//let parallel = Vector2::new(-contact.normal[1],contact.normal[0]);
					//*new_velocity = Some(parallel.multiply_by(dot(&velocity,&parallel)));
					//*new_velocity = Some(Vector2::new(0.0,0.0));
					*new_velocity = Some(velocity - dot(&velocity,&contact.normal)*contact.normal);
					*new_position = Some(position + velocity.multiply_by(delta_time) - contact.normal.multiply_by(contact.depth.abs()));
				}else{
					*new_velocity = Some(velocity);
					*new_position = Some(position + velocity.multiply_by(delta_time));
				}
			}
		}

		//Process position after collision checking
		for(
			&mut components::Position(ref mut position),
			&mut components::Collision{ref mut velocity,..},
			&mut components::CollisionCache{ref mut new_position,ref mut new_velocity},
		) in (
			&mut positions,
			&mut collisions,
			&mut collision_caches,
		).join(){
			if let &mut Some(ref mut new_position) = new_position{
				*position = *new_position;
			}
			*new_position = None;

			if let &mut Some(ref mut new_velocity) = new_velocity{
				*velocity = *new_velocity;
			}
			*new_velocity = None;
		}
	}
}
