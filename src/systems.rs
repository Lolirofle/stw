pub mod ingame{
	use alga::general::AbstractModule;
	use amethyst::ecs::transform::LocalTransform;
	use amethyst::ecs::input::InputHandler;
	use amethyst::ecs::{self,Join,System};
	use amethyst::timing::Time;
	use std::ops::Deref;

	use *;

	pub struct PlayerInput;
	impl<'a> System<'a> for PlayerInput{
		type SystemData = (
			ecs::WriteStorage<'a,components::Solid>,
			ecs::WriteStorage<'a,components::Player>,
			ecs::ReadStorage<'a,components::CollisionCache>,
			ecs::Fetch<'a,InputHandler>
		);

		fn run(&mut self,(mut collisions,mut players,collision_caches,input): Self::SystemData){
			use amethyst::event::VirtualKeyCode;
			use amethyst::input::ButtonState::*;
			use amethyst::input::ChangeState::*;

			for(
				ref mut player,
				&mut components::Solid{ref mut velocity,ref mut acceleration,..},
				&components::CollisionCache{ref position_resolve,..},
			) in (
				&mut players,
				&mut collisions,
				&collision_caches,
			).join(){
				match player.id{
					0 =>{
						if input.key_is(VirtualKeyCode::Up,Pressed(ThisFrame)){
							//If on ground
							if position_resolve[1] < 0.0{
								velocity[1] = -420.0;
							}
						}
						if input.key_is(VirtualKeyCode::Left,Pressed(Currently)){
							if velocity[0] > -100.0{
								acceleration[0]-= 1200.0;
							}
						}
						if input.key_is(VirtualKeyCode::Right,Pressed(Currently)){
							if velocity[0] < 100.0{
								acceleration[0]+= 1200.0;
							}
						}
					}
					1 =>{
						if input.key_is(VirtualKeyCode::W,Pressed(ThisFrame)){
							//If on ground
							if position_resolve[1] < 0.0{
								velocity[1] = -420.0;
							}
						}
						if input.key_is(VirtualKeyCode::A,Pressed(Currently)){
							velocity[0] = velocity[0].min(-100.0);
						}
						if input.key_is(VirtualKeyCode::D,Pressed(Currently)){
							velocity[0] = velocity[0].max(100.0);
						}
					}
					_ => {}
				};
			}
		}
	}

	pub struct Render;
	impl<'a> System<'a> for Render{
		type SystemData = (
			ecs::ReadStorage<'a,components::Solid>,
			ecs::ReadStorage<'a,components::Position>,
			ecs::WriteStorage<'a,LocalTransform>
		);

		fn run(&mut self,(collisions,positions,mut locals): Self::SystemData){
			use nalgebra::Isometry2;

			for(
				&components::Position(position),
				&components::Solid{ref shape,..},
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
				local.scale = [
					len[0] as f32,
					len[1] as f32,
					1.0
				];
			}
		}
	}

	pub struct Physics;
	impl Physics{
		pub const AIR_FRICTION: f64 = 20.0; //pixels/seconds^2
		pub const GRAVITY: f64 = 600.0;

		#[inline(always)]
		pub fn new() -> Self{Physics}
	}
	impl<'a> System<'a> for Physics{
		type SystemData = (
			ecs::WriteStorage<'a,components::CollisionCache>,
			ecs::WriteStorage<'a,components::Position>,
			ecs::WriteStorage<'a,components::Solid>,
			ecs::Fetch<'a,Time>
		);
		fn run(&mut self,(mut collision_caches,mut positions,mut solids,time) : Self::SystemData){
			use nalgebra::{Isometry2,dot,zero};
			use util;

			let delta_time = time.delta_time.subsec_nanos() as f64 / 1.0e9;

			//Step movement (using something like Velocity Verlet Integration)
			for(
				&mut components::Position(ref mut position),
				&mut components::Solid{ref mut velocity,ref mut acceleration,ref mut old_position,ref mut old_velocity,ref mut old_acceleration,gravity,..},
			) in (
				&mut positions,
				&mut solids,
			).join(){
				//Update acceleration with gravity
				if gravity{
					acceleration[1]+= Self::GRAVITY;
				}

				*old_position = *position;
				*position+= velocity.multiply_by(delta_time) + acceleration.multiply_by(delta_time*delta_time / 2.0)
				;

				*old_velocity = *velocity;
				*velocity+= (*acceleration + *old_acceleration).multiply_by(delta_time / 2.0);

				*old_acceleration = *acceleration;
				*acceleration = zero();
			}

			//Process collision checking
			for(
				&components::Position(this_pos),
				&components::Solid{velocity: ref this_vel,shape: ref this_shape,check_movement,friction: this_friction,..},
				&mut components::CollisionCache{ref mut position_resolve,ref mut velocity_resolve,ref mut friction_total,..},
			) in (
				&positions,
				&solids,
				&mut collision_caches,
			).join(){
				//Reset the resolvement data
				*position_resolve = zero();
				*velocity_resolve = zero();
				*friction_total   = zero();

				//If this is not a static object (no collision checking)
				if check_movement{
					//Check for every other existing object
					for(
						&components::Position(other_pos),
						&components::Solid{friction: other_friction,shape: ref other_shape,velocity: ref other_vel, old_position: ref other_old_pos,check_movement: other_check_movement,..},
					) in (
						&positions,
						&solids,
					).join(){
						//Skip collision with itself
						if (this_shape as *const _)==(other_shape as *const _){
							continue;
						}

						//If it made contact to something
						if let Some(contact) = ::ncollide::query::contact(
							&Isometry2::new(this_pos,zero()),
							this_shape.deref(),
							&Isometry2::new(other_pos,zero()),
							other_shape.deref(),
							0.0
						){
							if contact.depth != 0.0{
								//Friction (Solid)
								*friction_total+= this_friction + other_friction;

								//Combine with other possible collision resolvements
								//Subtracts the velocity projected on the contact normal (TODO: Stops when moving towards edge while falling/jumping)
								*velocity_resolve+= -dot(&(this_vel - other_vel),&contact.normal)*contact.normal;
								//Subtracts the position by the contact depth.
								//Both object tries to resolve the contact, and how much each of them resolves depends on the ratio of how much each contributed to the contact based on the velocity
								//TODO: When one player walks into a wall, and the other walks into the player walking into a wall, they both get pushed into solids. This is probably because collision checking checks objects with the new position that have not been collision checked yet, and when they resolve, others will not be able to follow.
								let k = {
									let this_projected  = dot(this_vel,&contact.normal).abs();
									let other_projected = dot(other_vel,&contact.normal).abs();
									if this_projected==0.0 && other_projected==0.0{
										1.0
									}else{
										this_projected / (this_projected + other_projected)
									}
								};
								*position_resolve+= -contact.normal.multiply_by((k * contact.depth).abs());

								//Touching moving solids that do not have collision checking
								if !other_check_movement{
									*position_resolve+= other_pos - other_old_pos;
								}
							}
						}
					}
				}
			}

			//Apply resolvement from collision checking
			for(
				&mut components::Position(ref mut position),
				&mut components::Solid{ref mut velocity,..},
				&mut components::CollisionCache{ref mut position_resolve,ref mut velocity_resolve,ref mut friction_total,..},
			) in (
				&mut positions,
				&mut solids,
				&mut collision_caches,
			).join(){
				*position = *position + *position_resolve;
				*velocity = util::vector_lengthen(
					*velocity + *velocity_resolve,
					-(Self::AIR_FRICTION + *friction_total)*delta_time
				);
			}
		}
	}
}
