pub mod ingame{
	use amethyst::ecs::{Join,Fetch,System,ReadStorage,WriteStorage};
	use amethyst::ecs::components::LocalTransform;
	use amethyst::ecs::resources::Time;
	use amethyst::ecs::resources::input::InputHandler;
	use std::ops::Deref;

	use *;

	pub struct PlayerInput;
	impl<'a> System<'a> for PlayerInput{
		type SystemData = (
			WriteStorage<'a,components::Solid>,
			WriteStorage<'a,components::Player>,
			Fetch<'a,InputHandler>,
		);

		fn run(&mut self,(mut collisions,mut players,input): Self::SystemData){
			use amethyst::VirtualKeyCode;
			use amethyst::ecs::resources::input::ButtonState::*;
			use amethyst::ecs::resources::input::ChangeState::*;

			for(
				ref mut player,
				&mut components::Solid{ref mut velocity,..},
			) in (
				&mut players,
				&mut collisions,
			).join(){
				match player.id{
					1 =>{
						if input.key_is(VirtualKeyCode::W,Pressed(ThisFrame)){
							velocity[1] = -200.0;
						}
						if input.key_is(VirtualKeyCode::A,Pressed(Currently)){
							velocity[0] = -100.0;
						}
						if input.key_is(VirtualKeyCode::D,Pressed(Currently)){
							velocity[0] = 100.0;
						}
					}
					0 =>{
						if input.key_is(VirtualKeyCode::Up,Pressed(ThisFrame)){
							velocity[1] = -200.0;
						}
						if input.key_is(VirtualKeyCode::Left,Pressed(Currently)){
							velocity[0] = -100.0;
						}
						if input.key_is(VirtualKeyCode::Right,Pressed(Currently)){
							velocity[0] = 100.0;
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
			ReadStorage<'a,components::Solid>,
			ReadStorage<'a,components::Position>,
			WriteStorage<'a,LocalTransform>,
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
				local.scale = [len[0] as f32, len[1] as f32, 1.0];
			}
		}
	}

	pub struct Physics;
	impl Physics{
		pub const AIR_FRICTION: f64 = 30.0; //pixels/seconds^2
	}
	impl<'a> System<'a> for Physics{
		type SystemData = (
			WriteStorage<'a,components::CollisionCache>,
			WriteStorage<'a,components::Position>,
			WriteStorage<'a,components::Solid>,
			Fetch<'a,Time>,
		);

		fn run(&mut self,(mut collision_caches,mut positions,mut solids,time): Self::SystemData){
			use alga::general::AbstractModule;
			use nalgebra::{Isometry2,Vector2,dot,zero};

			use util;

			let delta_time = time.delta_time.subsec_nanos() as f64 / 1.0e9;

			//Step movement (using something like Velocity Verlet Integration), and process collision checking
			for(
				&components::Position(position),
				&components::Solid{mut velocity,mut acceleration,gravity,ref shape,check_movement,..},
				&mut components::CollisionCache{ref mut new_position,ref mut new_velocity,..},
			) in (
				&positions,
				&solids,
				&mut collision_caches,
			).join(){
				//Check acceleration with gravity (TODO: repeating gravity below)
				if gravity{
					acceleration[1]+= 400.0;
				}

				//The new position it should land on if there are no collisions
				#[inline]
				fn calc_new_position(position: Vector2<f64>,velocity: Vector2<f64>,acceleration: Vector2<f64>,delta_time: f64) -> Vector2<f64>{
					position + velocity.multiply_by(delta_time) + acceleration.multiply_by(delta_time*delta_time / 2.0)
				}
				let maybe_new_position = calc_new_position(position,velocity,acceleration,delta_time);

				//Check for every existing object
				for(
					&components::Position(position2),
					&components::Solid{friction,velocity: velocity2,acceleration: acceleration2,shape: ref shape2,..},
				) in (
					&positions,
					&solids,
				).join(){
					//Skip collision with itself
					if (shape as *const _)==(shape2 as *const _){
						continue;
					}

					//If this is not a static object (no collision checking) and it made contact to something
					if let (true,Some(contact)) = (check_movement,::ncollide::query::contact(
						&Isometry2::new(maybe_new_position,zero()),
						shape.deref(),
						&Isometry2::new(calc_new_position(position2,velocity2,acceleration2,delta_time),zero()),
						shape2.deref(),
						0.0
					)){
						//Friction (Solid)
						velocity = util::vector_lengthen(velocity,-friction*delta_time); //TODO: Should this be included in the integration below? How? By introducing a new variable in CollisionCache which could be called tmp_acceleration that will be calculated in beforehand here?

						//Join with other possible collision resolvements
						//TODO: This is a temporary fix. Also join position resolvements. The effect/bug can be seen when leaning against one solid while falling onto another one (The velocity will slow down before reaching the ground). To get the general feel of the problem, remove the following code block and one will fall through the ground instead.
						/*if let &mut Some(new_velocity) = new_velocity{
							velocity = new_velocity;
						}*/

						//Set the values "to be changed".
						//Resolve the collision so that it does not move into the insides of a solid
						//*new_position = Some(maybe_new_position - contact.normal.multiply_by(contact.depth.abs()));
						//*new_velocity = Some(velocity - dot(&velocity,&contact.normal)*contact.normal);

						// This code is a shitshow. Basically, only change by the lowest value, merge x and y channels separately.
						// Any nice matrix operations for this?
						let mut new_vel = velocity - dot(&velocity,&contact.normal)*contact.normal;
						if let &mut Some(new_velocity) = new_velocity {
							new_vel[0] = if new_velocity[0].abs() < new_vel[0].abs() {new_velocity[0]} else {new_vel[0]};
							new_vel[1] = if new_velocity[1].abs() < new_vel[1].abs() {new_velocity[1]} else {new_vel[1]};
						}

						let mut new_pos = calc_new_position(position,velocity,acceleration,delta_time) - contact.normal.multiply_by(contact.depth.abs());
						if let &mut Some(new_position) = new_position {
							let old_offset = new_position - position;
							let mut new_offset = new_pos - position;
							new_offset[0] = if old_offset[0].abs() < new_offset[0].abs() {old_offset[0]} else {new_offset[0]};
							new_offset[1] = if old_offset[1].abs() < new_offset[1].abs() {old_offset[1]} else {new_offset[1]};
							new_pos = new_offset + position;
						}
						*new_velocity = Some(new_vel);
						*new_position = Some(new_pos);
					}
				}

				//If there are no collisions
				if let &mut None = new_velocity{
					//Friction (Air)
					velocity = util::vector_lengthen(velocity,-Self::AIR_FRICTION*delta_time);

					//Set the values "to be changed"
					//It can move according to plan
					*new_position = Some(maybe_new_position);
					*new_velocity = Some(velocity);
				}
			}

			//Change the variables for real: Positions after collision checking, and velocities from acceleration
			for(
				&mut components::Position(ref mut position),
				&mut components::Solid{ref mut velocity,mut acceleration,gravity,..},
				&mut components::CollisionCache{ref mut new_position,ref mut new_velocity,ref mut old_acceleration,..},
			) in (
				&mut positions,
				&mut solids,
				&mut collision_caches,
			).join(){
				//Apply gravity
				if gravity{
					acceleration[1]+= 400.0;
				}

				//Update all positions
				if let &mut Some(ref mut new_position) = new_position{
					*position = *new_position;
				}
				*new_position = None;

				//Update all velocities
				if let &mut Some(ref mut new_velocity) = new_velocity{
					*velocity = *new_velocity + (acceleration + *old_acceleration).multiply_by(delta_time / 2.0);
				}
				*new_velocity = None;

				//Set the old acceleration to the current one (preparing for the next step)
				*old_acceleration = acceleration;
			}

			//TODO: Collect all collisions, call a collision event function which gives specs::RunArg::fetch as an argument? Consider using the already existing ConctactHandlers (comes with ncollide)?
		}
	}
}
