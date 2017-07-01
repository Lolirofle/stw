extern crate amethyst;
extern crate nalgebra;
extern crate ncollide;

use amethyst::{Application, Event, State, Trans, VirtualKeyCode, WindowEvent};
use amethyst::asset_manager::AssetManager;
use amethyst::ecs::{Gate, World, Join, RunArg, System};
use amethyst::ecs::components::{Mesh, LocalTransform, Texture, Transform};
use amethyst::gfx_device::DisplayConfig;
use amethyst::renderer::{Pipeline, VertexPosNormal};
use nalgebra::{Isometry2,Point2,Vector2,dot,zero};
use ncollide::narrow_phase::{ProximityHandler,ContactHandler,ContactAlgorithm2};
use ncollide::query::Proximity;
use ncollide::shape::{Plane,Ball,Cuboid,ShapeHandle2};
use ncollide::world::{CollisionWorld2,CollisionGroups,GeometricQueryType,CollisionObject2};

mod data{
	pub enum Side{
		Left,
		Right,
	}
}

mod components{
	use amethyst::ecs::{VecStorage,Component};
	use nalgebra::Vector2;
	use ncollide::shape::ShapeHandle2;

	pub struct Object{
		position: Vector2<f32>,
		shape: ShapeHandle2<f32>
	}

	pub struct Position(pub Vector2<f32>);
	impl Component for Position{
		type Storage = VecStorage<Position>;
	}

	pub struct Velocity(pub Vector2<f32>);
	impl Component for Velocity{
		type Storage = VecStorage<Velocity>;
	}

	pub struct Collision(pub usize);
	impl Component for Collision{
		type Storage = VecStorage<Collision>;
	}

	pub struct Ball{
		pub size: f32,
	}
	impl Component for Ball{
		type Storage = VecStorage<Ball>;
	}

	pub struct Plank{
		pub dimensions: Vector2<f32>,
		pub side: ::data::Side,
	}
	impl Component for Plank{
		type Storage = VecStorage<Plank>;
	}
}

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
		let (mut balls, planks, positions, velocities, (locals, camera, time, input), mut score) = arg.fetch(|w| (
			w.write::<components::Ball>(),
			w.write::<components::Plank>(),
			w.write::<components::Position>(),
			w.write::<components::Velocity>(),
			(
				w.write::<LocalTransform>(),
				w.read_resource::<Camera>(),
				w.read_resource::<Time>(),
				w.read_resource::<InputHandler>(),
			),
			w.write_resource::<Score>()
		));

		//Get left and right boundaries of the screen
		let (left_bound, right_bound, top_bound, bottom_bound) = match camera.proj{
			Projection::Orthographic{ left, right, top, bottom, .. } => (left, right, top, bottom),
			_ => (1.0, 1.0, 1.0, 1.0),
		};

		//Properties of left paddle.
		let mut left_dimensions = Vector2::new(0.0,0.0);
		let mut left_position = 0.0;

		//Properties of right paddle.
		let mut right_dimensions = Vector2::new(0.0,0.0);
		let mut right_position = 0.0;

		let delta_time = time.delta_time.subsec_nanos() as f32 / 1.0e9;

		let mut locals     = locals.pass();
		let mut positions  = positions.pass();
		let mut velocities = velocities.pass();

		//Process all planks
		for (ref mut plank, &mut components::Position(ref mut position), &mut components::Velocity(ref mut velocity), ref mut local) in (&mut planks.pass(), &mut positions, &mut velocities, &mut locals).join(){
			match plank.side{
				//If it is a left plank
				data::Side::Left =>{
					//Store left plank position for later use in ball processing
					left_position = position[1];
					//Store left plank dimensions for later use in ball processing
					left_dimensions = plank.dimensions;
					//If `W` is pressed and plank is in screen boundaries then move up
					if input.key_down(VirtualKeyCode::W){
						if position[1] + plank.dimensions[1] / 2. < 1.{
							*velocity = Vector2::new(0.0,1.0);
						}
					}
					//If `S` is pressed and plank is in screen boundaries then move down
					if input.key_down(VirtualKeyCode::S){
						if position[1] - plank.dimensions[1] / 2. > -1.{
							*velocity = Vector2::new(0.0,-1.0);
						}
					}
				}
				//If it is a right plank
				data::Side::Right =>{
					//Store right plank position for later use in ball processing
					right_position = position[1];
					//Store right plank dimensions for later use in ball processing
					right_dimensions = plank.dimensions;
					//If `Up` is pressed and plank is in screen boundaries then move down
					if input.key_down(VirtualKeyCode::Up){
						if position[1] + plank.dimensions[1] / 2. < top_bound{
							*velocity = Vector2::new(0.0,1.0);
						}
					}
					//If `Down` is pressed and plank is in screen boundaries then move down
					if input.key_down(VirtualKeyCode::Down){
						if position[1] - plank.dimensions[1] / 2. > bottom_bound{
							*velocity = Vector2::new(0.0,-1.0);
						}
					}
				}
			};
			//Set translation of renderable corresponding to this plank
			local.translation[0] = position[0];
			local.translation[1] = position[1];
			//Set scale for renderable corresponding to this plank
			local.scale = [plank.dimensions[0], plank.dimensions[1], 1.0];
		}

		//Process the ball
		for (ref mut ball, &mut components::Position(ref mut position), &mut components::Velocity(ref mut velocity), ref mut local) in (&mut balls, &mut positions, &mut velocities, &mut locals).join(){
			//Check if the ball has collided with the right plank
			if position[0] + ball.size / 2. > right_bound - left_dimensions[0] &&
			   position[0] + ball.size / 2. < right_bound{
				if position[1] - ball.size / 2. < right_position + right_dimensions[1] / 2. &&
				   position[1] + ball.size / 2. > right_position - right_dimensions[1] / 2.{
					position[0] = right_bound - right_dimensions[0] - ball.size / 2.;
					velocity[0] = -velocity[0];
				}
			}

			//Check if the ball is to the left of the right boundary, if it is not reset it's position and score the left player
			if position[0] - ball.size / 2. > right_bound{
				position[0] = 0.;
				score.score_left += 1;
				println!("Left player score:{0}, Right player score: {1}",
						 score.score_left,
						 score.score_right);
			}

			//Check if the ball has collided with the left plank
			if position[0] - ball.size / 2. < left_bound + left_dimensions[0] &&
			   position[0] + ball.size / 2. > left_bound{
				if position[1] - ball.size / 2. < left_position + left_dimensions[1] / 2. &&
				   position[1] + ball.size / 2. > left_position - left_dimensions[1] / 2.{
					position[0] = left_bound + left_dimensions[0] + ball.size / 2.;
					velocity[0] = -velocity[0];
				}
			}

			//Check if the ball is to the right of the left boundary, if it is not reset it's position and score the right player
			if position[0] + ball.size / 2. < left_bound{
				position[0] = 0.;
				score.score_right += 1;
				println!("Left player score:{0}, Right player score{1}",
						 score.score_left,
						 score.score_right);
			}

			//Check if the ball is below the top boundary, if it is not deflect it
			if position[1] + ball.size / 2. > top_bound{
				position[1] = top_bound - ball.size / 2.;
				velocity[1] = -velocity[1];
			}

			//Check if the ball is above the bottom boundary, if it is not deflect it
			if position[1] - ball.size / 2. < bottom_bound{
				position[1] = bottom_bound + ball.size / 2.;
				velocity[1] = -velocity[1];
			}

			//Update the renderable corresponding to this ball
			local.translation[0] = position[0];
			local.translation[1] = position[1];
			local.scale[0] = ball.size;
			local.scale[1] = ball.size;
		}

		//Process position from velocity
		for (&mut components::Position(ref mut position), &mut components::Velocity(ref mut velocity)) in (&mut positions, &mut velocities).join(){
			position[0] += velocity[0] * delta_time;
			position[1] += velocity[1] * delta_time;
		}
	}
}

struct Pong{
	collision        : CollisionWorld2<f32,()>,
	collision_next_id: usize,
	collision_group  : CollisionGroups
}
impl Pong{
	fn gen_collision_id(&mut self) -> usize{
		let id = self.collision_next_id;
		self.collision_next_id+= 1;
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
			let eye = [0., 0., 0.1];
			let target = [0., 0., 0.];
			let up = [0., 1., 0.];

			//Get an Orthographic projection
			let proj = Projection::Orthographic{
				left: -1.0 * aspect_ratio,
				right: 1.0 * aspect_ratio,
				bottom: -1.0,
				top: 1.0,
				near: 0.0,
				far: 1.0,
			};

			camera.proj = proj;
			camera.eye = eye;
			camera.target = target;
			camera.up = up;
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
		let square_verts = gen_rectangle(1.0, 1.0);
		assets.load_asset_from_data::<Mesh, Vec<VertexPosNormal>>("square", square_verts);
		let square = assets.create_renderable("square", "white", "white", "white", 1.0).unwrap();

		//Initialize collision checking
		let contacts_query  = GeometricQueryType::Contacts(0.0);
		let proximity_query = GeometricQueryType::Proximity(0.0);

		//Add borders to the collision checking
		/*
		let plane_top        = ShapeHandle2::new(Plane::new(-Vector2::y()));
		let plane_bottom     = ShapeHandle2::new(Plane::new(Vector2::y()));
		let plane_pos_top    = Isometry2::new(Vector2::new(0.0, -10.0), zero());
		let plane_pos_bottom = Isometry2::new(Vector2::new(0.0,  10.0), zero());
		self.collision.deferred_add(3, planes_top_pos   , plane_top,    collision_group, contacts_query, ());
		self.collision.deferred_add(1, planes_bottom_pos, plane_bottom, collision_group, contacts_query, ());
		*/

		//Register our handlers.
		self.collision.register_proximity_handler("ProximityMessage", ProximityMessage);
		self.collision.register_contact_handler("VelocityBouncer", VelocityBouncer);

		//Create a ball entity
		{
			let ball  = components::Ball{size: 0.1};
			let pos   = Vector2::new(0.0,0.0);
			let shape = ShapeHandle2::new(Ball::new(0.1f32));
			world.create_now()
				.with(square.clone())
				.with(ball)
				.with(components::Position(pos))
				.with(components::Velocity(Vector2::new(1.0,1.0)))
				.with(components::Collision({
					let id = self.gen_collision_id();
					self.collision.deferred_add(
						id,
						Isometry2::new(pos,zero()),
						shape,
						self.collision_group,
						GeometricQueryType::Contacts(0.0),
						()
					);
					id
				}))
				.with(LocalTransform::default())
				.with(Transform::default())
				.build();
		}

		//Create a left plank entity
		{
			let plank = components::Plank{
				dimensions: Vector2::new(0.1,0.3),
				side      : data::Side::Left,
			};
			let pos   = Vector2::new(-1.0 + plank.dimensions[0] / 2.0,0.0);
			let shape = ShapeHandle2::new(Cuboid::new(Vector2::new(0.1f32, 0.3)));
			world.create_now()
				.with(square.clone())
				.with(plank)
				.with(components::Position(pos))
				.with(components::Velocity(Vector2::new(0.0,0.0)))
				.with(components::Collision({
					let id = self.gen_collision_id();
					self.collision.deferred_add(
						id,
						Isometry2::new(pos,zero()),
						shape,
						self.collision_group,
						proximity_query,
						()
					);
					id
				}))
				.with(LocalTransform::default())
				.with(Transform::default())
				.build();
		}

		//Create right plank entity
		{
			let plank = components::Plank{
				dimensions: Vector2::new(0.1,0.3),
				side      : data::Side::Right,
			};
			let pos   = Vector2::new(1.0 + plank.dimensions[0] / 2.0,0.0);
			let shape = ShapeHandle2::new(Cuboid::new(Vector2::new(0.1f32, 0.3)));
			world.create_now()
				.with(square.clone())
				.with(plank)
				.with(components::Position(pos))
				.with(components::Velocity(Vector2::new(0.0,0.0)))
				.with(components::Collision({
					let id = self.gen_collision_id();
					self.collision.deferred_add(
						id,
						Isometry2::new(pos,zero()),
						shape,
						self.collision_group,
						proximity_query,
						()
					);
					id
				}))
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
				Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) => return Trans::Quit,
				Event::Closed => return Trans::Quit,
				_ => (),
			}
		}
		Trans::None
	}

	fn update(&mut self,world: &mut World,_: &mut AssetManager,_: &mut Pipeline) -> Trans{
		let mut positions = world.write::<components::Position>().pass();
		let     objs      = world.read::<components::Collision>().pass();

		for (&components::Collision(obj_id), &mut components::Position(position)) in (&objs, &mut positions).join(){
			self.collision.deferred_set_position(obj_id,Isometry2::new(position,zero()));
		}

		Trans::None
	}
}

struct ProximityMessage;
impl ProximityHandler<Point2<f32>,Isometry2<f32>,()> for ProximityMessage{
	fn handle_proximity(&mut self,co1: &CollisionObject2<f32,()>,co2: &CollisionObject2<f32,()>,_: Proximity,new_proximity: Proximity){
		if new_proximity == Proximity::Intersecting{
			println!("The ball enters");
		}else if new_proximity == Proximity::Disjoint{
			println!("The ball leaves");
		}
	}
}

struct VelocityBouncer;
impl ContactHandler<Point2<f32>, Isometry2<f32>,()> for VelocityBouncer{
	fn handle_contact_started(&mut self,co1: &CollisionObject2<f32,()>,co2: &CollisionObject2<f32,()>,alg: &ContactAlgorithm2<f32>){
		// NOTE: real-life applications would avoid this systematic allocation.
		let mut collector = Vec::new();
		alg.contacts(&mut collector);

		println!("{:?} {:?}",co1.position,co2.position);

		// The ball is the one with a non-None velocity.
		/*if let Some(ref vel) = co1.data.velocity{
			let normal = collector[0].normal;
			vel.set(vel.get() - 2.0 * dot(&vel.get(), &normal) * normal);
		}
		if let Some(ref vel) = co2.data.velocity{
			let normal = -collector[0].normal;
			vel.set(vel.get() - 2.0 * dot(&vel.get(), &normal) * normal);
		}*/
	}

	fn handle_contact_stopped(&mut self,_: &CollisionObject2<f32,()>,_: &CollisionObject2<f32,()>){}
}

fn main(){
	let cfg = DisplayConfig::default();
	let mut game = Application::build(
		Pong{
			collision        : CollisionWorld2::new(0.02,true),
			collision_next_id: 0,
			collision_group  : CollisionGroups::new(), //Every object is part of this group and interacts with everything
		},
		cfg
	)
		.register::<components::Ball>()
		.register::<components::Plank>()
		.register::<components::Position>()
		.register::<components::Velocity>()
		.register::<components::Collision>()
		.with::<PongSystem>(PongSystem, "pong_system", 1)
		.done();
	game.run();
}

fn gen_rectangle(w: f32,h: f32) -> Vec<VertexPosNormal>{
	vec![
		VertexPosNormal{
			pos: [-w / 2., -h / 2., 0.],
			normal: [0., 0., 1.],
			tex_coord: [0., 0.],
		},
		VertexPosNormal{
			pos: [w / 2., -h / 2., 0.],
			normal: [0., 0., 1.],
			tex_coord: [1., 0.],
		},
		VertexPosNormal{
			pos: [w / 2., h / 2., 0.],
			normal: [0., 0., 1.],
			tex_coord: [1., 1.],
		},
		VertexPosNormal{
			pos: [w / 2., h / 2., 0.],
			normal: [0., 0., 1.],
			tex_coord: [1., 1.],
		},
		VertexPosNormal{
			pos: [-w / 2., h / 2., 0.],
			normal: [0., 0., 1.],
			tex_coord: [1., 1.],
		},
		VertexPosNormal{
			pos: [-w / 2., -h / 2., 0.],
			normal: [0., 0., 1.],
			tex_coord: [1., 1.],
		}
	]
}
