extern crate amethyst;
extern crate nalgebra;
extern crate ncollide;

mod collision;
mod components;
mod data;
mod util;

use amethyst::{Application, Event, State, Trans, VirtualKeyCode, WindowEvent};
use amethyst::asset_manager::AssetManager;
use amethyst::ecs::{Gate, World, Join, RunArg, System};
use amethyst::ecs::components::{Mesh, LocalTransform, Texture, Transform};
use amethyst::gfx_device::DisplayConfig;
use amethyst::renderer::{Pipeline, VertexPosNormal};
use nalgebra::{Isometry2,Vector2,zero};
use ncollide::shape::{Plane,Ball,Cuboid,ShapeHandle2};
use ncollide::world::GeometricQueryType;
use std::cell::Cell;

use collision::ObjectType;

struct Score{
	score_left: i32,
	score_right: i32,
}

//Pong game system
struct PongSystem;
unsafe impl Sync for PongSystem{}
impl System<()> for PongSystem{
	fn run(&mut self, arg: RunArg, _: ()){
		use amethyst::ecs::Gate;
		use amethyst::ecs::resources::{Camera, InputHandler, Projection, Time};

		//Get all needed component storages and resources
		let (mut balls, planks, positions, collisions, (locals, camera, time, input), mut score) = arg.fetch(|w| (
			w.write::<components::Ball>(),
			w.write::<components::Plank>(),
			w.write::<components::Position>(),
			w.write::<components::Collision>(),
			(
				w.write::<LocalTransform>(),
				w.read_resource::<Camera>(),
				w.read_resource::<Time>(),
				w.read_resource::<InputHandler>(),
			),
			w.write_resource::<Score>(),
		));

		//Get left and right boundaries of the screen
		let (left_bound,right_bound,_,_) = match camera.proj{
			Projection::Orthographic{left,right,top,bottom,..} => (left as f64,right as f64,top as f64,bottom as f64),
			_ => (1.0,1.0,1.0,1.0),
		};

		let delta_time = time.delta_time.subsec_nanos() as f64 / 1.0e9;

		let mut planks     = planks.pass();
		let mut locals     = locals.pass();
		let mut positions  = positions.pass();
		let mut collisions = collisions.pass();

		//Process all planks
		for(
			ref mut plank,
			&mut components::Position(ref mut position),
			&mut components::Collision{ref mut velocity,..},
			ref mut local
		) in (
			&mut planks,
			&mut positions,
			&mut collisions,
			&mut locals
		).join(){
			match plank.side{
				//If it is a left plank
				data::Side::Left =>{
					if input.key_down(VirtualKeyCode::W){
						velocity[1]-= 200.0;
					}
					if input.key_down(VirtualKeyCode::A){
						velocity[0]-= 50.0;
					}
					if input.key_down(VirtualKeyCode::D){
						velocity[0]+= 50.0;
					}
				}
				//If it is a right plank
				data::Side::Right =>{
					if input.key_down(VirtualKeyCode::Up){
						velocity[1]-= 200.0;
					}
					if input.key_down(VirtualKeyCode::Left){
						velocity[0]-= 50.0;
					}
					if input.key_down(VirtualKeyCode::Right){
						velocity[0]+= 50.0;
					}
				}
			};
			//Set translation of renderable corresponding to this plank
			local.translation[0] = position[0] as f32;
			local.translation[1] = position[1] as f32;
			//Set scale for renderable corresponding to this plank
			local.scale = [plank.dimensions[0] as f32, plank.dimensions[1] as f32, 1.0];
		}

		//Process the ball
		for (ref mut ball, &mut components::Position(ref mut position), ref mut local) in (&mut balls, &mut positions, &mut locals).join(){
			//Check if the ball is to the left of the right boundary, if it is not reset it's position and score the left player
			if position[0] - ball.size / 2. > right_bound{
				position[0] = 0.;
				score.score_left += 1;
				println!("L: {0} ; R: {1}",
					score.score_left,
					score.score_right
				);
			}

			//Check if the ball is to the right of the left boundary, if it is not reset it's position and score the right player
			if position[0] + ball.size / 2. < left_bound{
				position[0] = 0.;
				score.score_right += 1;
				println!("L: {0} ; R: {1}",
					score.score_left,
					score.score_right
				);
			}

			//Update the renderable corresponding to this ball
			local.translation[0] = position[0] as f32;
			local.translation[1] = position[1] as f32;
			local.scale[0]       = ball.size as f32 *2.0;
			local.scale[1]       = ball.size as f32 *2.0;
		}

		//Process position from velocity
		for(
			&mut components::Position(ref mut position),
			&mut components::Collision{ref mut velocity,ref mut acceleration,..}
		) in (
			&mut positions,
			&mut collisions
		).join(){
			//*velocity *= 0.8; //TODO: Temporary fix for friction

			velocity[0] += acceleration[0] * delta_time;
			velocity[1] += acceleration[1] * delta_time;

			position[0] += velocity[0] * delta_time;
			position[1] += velocity[1] * delta_time;
		}
	}
}

struct Pong{
	collision: collision::Collision,
}
impl Pong{
	fn gen_collision_id(&mut self) -> usize{
		let id = self.collision.next_id;
		self.collision.next_id+= 1;
		id
	}
}
impl State for Pong{
	fn on_start(&mut self, world: &mut World, assets: &mut AssetManager, pipe: &mut Pipeline){
		use amethyst::ecs::Gate;
		use amethyst::ecs::resources::{Camera, InputHandler, Projection, ScreenDimensions};
		use amethyst::renderer::Layer;
		use amethyst::renderer::pass::{Clear, DrawFlat};

		let layer = Layer::new("main",vec![
			Clear::new([0.0, 0.0, 0.0, 1.0]),
			DrawFlat::new("main", "main")
		]);

		pipe.layers.push(layer);

		{
			let dim = world.read_resource::<ScreenDimensions>().pass();
			let mut camera = world.write_resource::<Camera>().pass();
			let aspect_ratio = dim.aspect_ratio;
			let eye    = [0.0, 0.0, 0.1];
			let target = [0.0, 0.0, 0.0];
			let up     = [0.0, 1.0, 0.0];

			//Get an Orthographic projection
			let proj = Projection::Orthographic{
				left  :  0.0 * aspect_ratio,
				right :  dim.w * aspect_ratio,
				bottom:  dim.h,
				top   :  0.0,
				near  :  0.0,
				far   :  1.0,
			};

			camera.proj   = proj;
			camera.eye    = eye;
			camera.target = target;
			camera.up     = up;
		}

		//Add all resources
		world.add_resource::<Score>(Score{
			score_left : 0,
			score_right: 0,
		});
		world.add_resource::<InputHandler>(InputHandler::new());

		//Generate a square mesh
		assets.register_asset::<Mesh>();
		assets.register_asset::<Texture>();
		assets.load_asset_from_data::<Texture, [f32; 4]>("white", [1.0, 1.0, 1.0, 1.0]);
		let square_verts = util::gen_rectangle(1.0, 1.0);
		assets.load_asset_from_data::<Mesh, Vec<VertexPosNormal>>("square", square_verts);
		let square = assets.create_renderable("square", "white", "white", "white", 1.0).unwrap();

		//Add borders to the collision checking
		{
			let plane = ShapeHandle2::new(Plane::new(Vector2::y()));
			let pos   = Vector2::new(0.0,-1.0);
			let id    = self.gen_collision_id();
			self.collision.world.deferred_add(
				id,
				Isometry2::new(pos,zero()),
				plane,
				self.collision.group,
				GeometricQueryType::Contacts(0.0),
				collision::ObjectData{
					..collision::ObjectData::default()
				}
			);
		}

		{
			let plane = ShapeHandle2::new(Plane::new(-Vector2::y()));
			let pos   = Vector2::new(0.0,1.0);
			let id    = self.gen_collision_id();
			self.collision.world.deferred_add(
				id,
				Isometry2::new(pos,zero()),
				plane,
				self.collision.group,
				GeometricQueryType::Contacts(0.0),
				collision::ObjectData{
					..collision::ObjectData::default()
				}
			);
		}

		//Register handlers.
		self.collision.world.register_proximity_handler("ProximityMessage", collision::ProximityMessage);
		self.collision.world.register_contact_handler("VelocityBouncer", collision::VelocityBouncer::new());

		//Create a ball entity
		{
			let ball  = components::Ball{size: 50.0};
			let pos   = Vector2::new(500.0,0.0);
			let vel   = Vector2::new(10.0,10.0);
			let acc   = Vector2::new(0.0,0.0);
			let shape = ShapeHandle2::new(Ball::new(50.0));
			world.create_now()
				.with(square.clone())
				.with(ball)
				.with(components::Position(pos))
				.with(components::Collision{
					velocity: vel,
					acceleration: acc,
					id: {
						let id = self.gen_collision_id();
						self.collision.world.deferred_add(
							id,
							Isometry2::new(pos,zero()),
							shape,
							self.collision.group,
							GeometricQueryType::Contacts(0.0),
							collision::ObjectData{
								typ: ObjectType::Bounce,
								velocity: Cell::new(vel),
								..collision::ObjectData::default()
							}
						);
						id
					}
				})
				.with(LocalTransform::default())
				.with(Transform::default())
				.build();
		}

		//Create a left plank entity
		{
			let plank = components::Plank{
				dimensions: Vector2::new(16.0,32.0),
				side      : data::Side::Left,
			};
			let pos   = Vector2::new(10.0 + plank.dimensions[0] / 2.0,0.0);
			let vel   = Vector2::new(0.0,0.0);
			let acc   = Vector2::new(0.0,0.0);
			let shape = ShapeHandle2::new(Cuboid::new(Vector2::new(16.0/2.0, 32.0/2.0)));
			world.create_now()
				.with(square.clone())
				.with(plank)
				.with(components::Position(pos))
				.with(components::Collision{
					velocity: vel,
					acceleration: acc,
					id: {
						let id = self.gen_collision_id();
						self.collision.world.deferred_add(
							id,
							Isometry2::new(pos,zero()),
							shape,
							self.collision.group,
							GeometricQueryType::Contacts(0.0),
							collision::ObjectData{
								velocity: Cell::new(vel),
								..collision::ObjectData::default()
							}
						);
						id
					}
				})
				.with(LocalTransform::default())
				.with(Transform::default())
				.build();
		}

		//Create right plank entity
		{
			let plank = components::Plank{
				dimensions: Vector2::new(16.0,32.0),
				side      : data::Side::Right,
			};
			let pos   = Vector2::new(100.0 + plank.dimensions[0] / 2.0,0.0);
			let vel   = Vector2::new(0.0,0.0);
			let acc   = Vector2::new(0.0,0.0);
			let shape = ShapeHandle2::new(Cuboid::new(Vector2::new(10.0/2.0, 30.0/2.0)));
			world.create_now()
				.with(square.clone())
				.with(plank)
				.with(components::Position(pos))
				.with(components::Collision{
					velocity: vel,
					acceleration: acc,
					id: {
						let id = self.gen_collision_id();
						self.collision.world.deferred_add(
							id,
							Isometry2::new(pos,zero()),
							shape,
							self.collision.group,
							GeometricQueryType::Contacts(0.0),
							collision::ObjectData{
								velocity: Cell::new(vel),
								..collision::ObjectData::default()
							}
						);
						id
					}
				})
				.with(LocalTransform::default())
				.with(Transform::default())
				.build();
		}
	}

	fn handle_events(&mut self,events: &[WindowEvent],world: &mut World,_: &mut AssetManager,_: &mut Pipeline) -> Trans{
		use amethyst::ecs::Gate;
		use amethyst::ecs::resources::InputHandler;

		let input = world.write_resource::<InputHandler>();
		input.pass().update(events);

		for e in events{
			match **e{
				Event::KeyboardInput(_,_,Some(VirtualKeyCode::Escape)) |
				Event::Closed => {
					return Trans::Quit
				},
				_ => (),
			}
		}

		Trans::None
	}

	fn update(&mut self,world: &mut World,_: &mut AssetManager,_: &mut Pipeline) -> Trans{
		let mut positions  = world.write::<components::Position>().pass();
		let mut objs       = world.write::<components::Collision>().pass();

		//TODO: Surely this much looping and copying cannot be efficient?
		for(
			&components::Collision{
				id: obj_id,
				velocity,
				acceleration,
				..
			},
			&mut components::Position(position)
		) in (
			&objs,
			&mut positions
		).join(){
			self.collision.world.deferred_set_position(obj_id,Isometry2::new(position,zero()));
			if let Some(obj) = self.collision.world.collision_object(obj_id){
				obj.data.velocity.set(velocity);
				obj.data.acceleration.set(acceleration);
			}
		}

		self.collision.world.update();

		for(
			&mut components::Collision{id: obj_id, ref mut velocity,ref mut acceleration,..},
			&mut components::Position(ref mut position),
		) in (&mut objs,&mut positions).join(){
			if let Some(obj) = self.collision.world.collision_object(obj_id){
				position[0] = obj.position.translation.vector[0];
				position[1] = obj.position.translation.vector[1];

				velocity[0] = obj.data.velocity.get()[0];
				velocity[1] = obj.data.velocity.get()[1];

				acceleration[0] = obj.data.acceleration.get()[0];
				acceleration[1] = obj.data.acceleration.get()[1];
			}
		}

		Trans::None
	}
}

fn main(){
	let cfg = DisplayConfig::default();
	let mut game = Application::build(
		Pong{
			collision: collision::Collision::new(),
		},
		cfg
	)
		.register::<components::Ball>()
		.register::<components::Plank>()
		.register::<components::Position>()
		.register::<components::Collision>()
		.with::<PongSystem>(PongSystem, "pong_system", 1)
		.done();
	game.run();
}
