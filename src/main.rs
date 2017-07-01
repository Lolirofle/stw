extern crate amethyst;
extern crate nalgebra;
extern crate ncollide;

use amethyst::{Application, Event, State, Trans, VirtualKeyCode, WindowEvent};
use amethyst::asset_manager::AssetManager;
use amethyst::ecs::{World, Join, VecStorage, Component, RunArg, System};
use amethyst::ecs::components::{Mesh, LocalTransform, Texture, Transform};
use amethyst::gfx_device::DisplayConfig;
use amethyst::renderer::{Pipeline, VertexPosNormal};
use nalgebra::{Vector2,Point2,Isometry2,Translation2};

struct Pong;

struct Object{
	pub position: Vector2<f32>,
	pub velocity: Vector2<f32>,
}

impl Component for Object {
	type Storage = VecStorage<Object>;
}

struct Ball {
	pub size: f32,
}

impl Component for Ball {
	type Storage = VecStorage<Ball>;
}

enum Side {
	Left,
	Right,
}

struct Plank {
	pub dimensions: Vector2<f32>,
	pub side: Side,
}

impl Component for Plank {
	type Storage = VecStorage<Plank>;
}

struct PongSystem;

unsafe impl Sync for PongSystem {}

struct Score {
	score_left: i32,
	score_right: i32,
}

// Pong game system
impl System<()> for PongSystem {
	fn run(&mut self, arg: RunArg, _: ()) {
		use amethyst::ecs::Gate;
		use amethyst::ecs::resources::{Camera, InputHandler, Projection, Time};

		// Get all needed component storages and resources
		let (mut balls, planks, mut objs, locals, camera, time, input, mut score) = arg.fetch(|w| (
			w.write::<Ball>(),
			w.write::<Plank>(),
			w.write::<Object>(),
			w.write::<LocalTransform>(),
			w.read_resource::<Camera>(),
			w.read_resource::<Time>(),
			w.read_resource::<InputHandler>(),
			w.write_resource::<Score>()
		));

		// Get left and right boundaries of the screen
		let (left_bound, right_bound, top_bound, bottom_bound) = match camera.proj {
			Projection::Orthographic { left, right, top, bottom, .. } => (left, right, top, bottom),
			_ => (1.0, 1.0, 1.0, 1.0),
		};

		// Properties of left paddle.
		let mut left_dimensions = Vector2::new(0.0,0.0);
		let mut left_position = 0.0;

		// Properties of right paddle.
		let mut right_dimensions = Vector2::new(0.0,0.0);
		let mut right_position = 0.0;

		let delta_time = time.delta_time.subsec_nanos() as f32 / 1.0e9;

		let mut locals = locals.pass();
		let mut objs   = objs.pass();

		// Process all planks
		for (plank, obj, local) in (&mut planks.pass(), &mut objs, &mut locals).join() {
			match plank.side {
				// If it is a left plank
				Side::Left => {
					// Store left plank position for later use in ball processing
					left_position = obj.position[1];
					// Store left plank dimensions for later use in ball processing
					left_dimensions = plank.dimensions;
					// If `W` is pressed and plank is in screen boundaries then move up
					if input.key_down(VirtualKeyCode::W) {
						if obj.position[1] + plank.dimensions[1] / 2. < 1. {
							obj.position += obj.velocity * delta_time;
						}
					}
					// If `S` is pressed and plank is in screen boundaries then move down
					if input.key_down(VirtualKeyCode::S) {
						if obj.position[1] - plank.dimensions[1] / 2. > -1. {
							obj.position -= obj.velocity * delta_time;
						}
					}
				}
				// If it is a right plank
				Side::Right => {
					// Store right plank position for later use in ball processing
					right_position = obj.position[1];
					// Store right plank dimensions for later use in ball processing
					right_dimensions = plank.dimensions;
					// If `Up` is pressed and plank is in screen boundaries then move down
					if input.key_down(VirtualKeyCode::Up) {
						if obj.position[1] + plank.dimensions[1] / 2. < top_bound {
							obj.position += obj.velocity * delta_time;
						}
					}
					// If `Down` is pressed and plank is in screen boundaries then move down
					if input.key_down(VirtualKeyCode::Down) {
						if obj.position[1] - plank.dimensions[1] / 2. > bottom_bound {
							obj.position -= obj.velocity * delta_time;
						}
					}
				}
			};
			// Set translation of renderable corresponding to this plank
			local.translation[0] = obj.position[0];
			local.translation[1] = obj.position[1];
			// Set scale for renderable corresponding to this plank
			local.scale = [plank.dimensions[0], plank.dimensions[1], 1.0];
		}

		// Process the ball
		for (ball, obj, local) in (&mut balls, &mut objs, &mut locals).join() {
			// Move the ball
			obj.position[1] += obj.velocity[1] * delta_time;
			obj.position[0] += obj.velocity[0] * delta_time;

			// Check if the ball has collided with the right plank
			if obj.position[0] + ball.size / 2. > right_bound - left_dimensions[0] &&
			   obj.position[0] + ball.size / 2. < right_bound {
				if obj.position[1] - ball.size / 2. < right_position + right_dimensions[1] / 2. &&
				   obj.position[1] + ball.size / 2. > right_position - right_dimensions[1] / 2. {
					obj.position[0] = right_bound - right_dimensions[0] - ball.size / 2.;
					obj.velocity[0] = -obj.velocity[0];
				}
			}

			// Check if the ball is to the left of the right boundary, if it is not reset it's position and score the left player
			if obj.position[0] - ball.size / 2. > right_bound {
				obj.position[0] = 0.;
				score.score_left += 1;
				println!("Left player score: {0}, Right player score {1}",
						 score.score_left,
						 score.score_right);
			}

			// Check if the ball has collided with the left plank
			if obj.position[0] - ball.size / 2. < left_bound + left_dimensions[0] &&
			   obj.position[0] + ball.size / 2. > left_bound {
				if obj.position[1] - ball.size / 2. < left_position + left_dimensions[1] / 2. &&
				   obj.position[1] + ball.size / 2. > left_position - left_dimensions[1] / 2. {
					obj.position[0] = left_bound + left_dimensions[0] + ball.size / 2.;
					obj.velocity[0] = -obj.velocity[0];
				}
			}

			// Check if the ball is to the right of the left boundary, if it is not reset it's position and score the right player
			if obj.position[0] + ball.size / 2. < left_bound {
				obj.position[0] = 0.;
				score.score_right += 1;
				println!("Left player score: {0}, Right player score {1}",
						 score.score_left,
						 score.score_right);
			}

			// Check if the ball is below the top boundary, if it is not deflect it
			if obj.position[1] + ball.size / 2. > top_bound {
				obj.position[1] = top_bound - ball.size / 2.;
				obj.velocity[1] = -obj.velocity[1];
			}

			// Check if the ball is above the bottom boundary, if it is not deflect it
			if obj.position[1] - ball.size / 2. < bottom_bound {
				obj.position[1] = bottom_bound + ball.size / 2.;
				obj.velocity[1] = -obj.velocity[1];
			}

			// Update the renderable corresponding to this ball
			local.translation[0] = obj.position[0];
			local.translation[1] = obj.position[1];
			local.scale[0] = ball.size;
			local.scale[1] = ball.size;
		}
	}
}

impl State for Pong {
	fn on_start(&mut self, world: &mut World, assets: &mut AssetManager, pipe: &mut Pipeline) {
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

			// Get an Orthographic projection
			let proj = Projection::Orthographic {
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

		// Add all resources
		world.add_resource::<Score>(Score{
			score_left : 0,
			score_right: 0,
		});
		world.add_resource::<InputHandler>(InputHandler::new());

		// Generate a square mesh
		assets.register_asset::<Mesh>();
		assets.register_asset::<Texture>();
		assets.load_asset_from_data::<Texture, [f32; 4]>("white", [1.0, 1.0, 1.0, 1.0]);
		let square_verts = gen_rectangle(1.0, 1.0);
		assets.load_asset_from_data::<Mesh, Vec<VertexPosNormal>>("square", square_verts);
		let square = assets.create_renderable("square", "white", "white", "white", 1.0).unwrap();

		// Create a ball entity
		{
			let ball = Ball{size: 0.1};
			world.create_now()
				.with(square.clone())
				.with(ball)
				.with(Object{
					position: Vector2::new(0.0,0.0),
					velocity: Vector2::new(1.0,1.0),
				})
				.with(LocalTransform::default())
				.with(Transform::default())
				.build();
		}

		// Create a left plank entity
		{
			let plank = Plank{
				dimensions: Vector2::new(0.1,0.3),
				side      : Side::Left,
			};
			let obj = Object{
				position: Vector2::new(-1.0 + plank.dimensions[0] / 2.0,0.0),
				velocity: Vector2::new(0.0,1.0),
			};
			world.create_now()
				.with(square.clone())
				.with(plank)
				.with(obj)
				.with(LocalTransform::default())
				.with(Transform::default())
				.build();
		}

		// Create right plank entity
		{
			let plank = Plank{
				dimensions: Vector2::new(0.1,0.3),
				side      : Side::Right,
			};
			let obj = Object{
				position: Vector2::new(1.0 + plank.dimensions[0] / 2.0,0.0),
				velocity: Vector2::new(0.0,1.0),
			};
			world.create_now()
				.with(square.clone())
				.with(plank)
				.with(obj)
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

		for e in events {
			match **e {
				Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) => return Trans::Quit,
				Event::Closed => return Trans::Quit,
				_ => (),
			}
		}
		Trans::None
	}
}

fn main() {
	let cfg = DisplayConfig::default();
	let mut game = Application::build(Pong, cfg)
		.register::<Ball>()
		.register::<Plank>()
		.register::<Object>()
		.with::<PongSystem>(PongSystem, "pong_system", 1)
		.done();
	game.run();
}

fn gen_rectangle(w: f32, h: f32) -> Vec<VertexPosNormal> {
	vec![
		VertexPosNormal {
			pos: [-w / 2., -h / 2., 0.],
			normal: [0., 0., 1.],
			tex_coord: [0., 0.],
		},
		VertexPosNormal {
			pos: [w / 2., -h / 2., 0.],
			normal: [0., 0., 1.],
			tex_coord: [1., 0.],
		},
		VertexPosNormal {
			pos: [w / 2., h / 2., 0.],
			normal: [0., 0., 1.],
			tex_coord: [1., 1.],
		},
		VertexPosNormal {
			pos: [w / 2., h / 2., 0.],
			normal: [0., 0., 1.],
			tex_coord: [1., 1.],
		},
		VertexPosNormal {
			pos: [-w / 2., h / 2., 0.],
			normal: [0., 0., 1.],
			tex_coord: [1., 1.],
		},
		VertexPosNormal {
			pos: [-w / 2., -h / 2., 0.],
			normal: [0., 0., 1.],
			tex_coord: [1., 1.],
		}
	]
}
