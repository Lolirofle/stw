extern crate alga;
extern crate amethyst;
extern crate nalgebra;
extern crate ncollide;

mod components;
mod data;
mod util;
mod systems;

use alga::general::AbstractModule;
use amethyst::{Application, Event, State, Trans, VirtualKeyCode, WindowEvent};
use amethyst::asset_manager::AssetManager;
use amethyst::ecs::{World, Join, RunArg, System};
use amethyst::ecs::components::{Mesh, LocalTransform, Texture, Transform};
use amethyst::gfx_device::DisplayConfig;
use amethyst::renderer::{Pipeline, VertexPosNormal};
use nalgebra::{Isometry2,Vector2,dot,zero};
use ncollide::shape::{Cuboid,ShapeHandle2};
use std::ops::Deref;

use data::*;

struct MainSystem;
unsafe impl Sync for MainSystem{}
impl System<()> for MainSystem{
	fn run(&mut self, arg: RunArg, _: ()){
		use amethyst::ecs::Gate;
		use amethyst::ecs::resources::{Camera, InputHandler, Projection, Time};

		//Get all needed component storages and resources
		let ((mut solids, players), (positions, collisions, collision_caches), (locals, camera, time, input), mut score) = arg.fetch(|w| (
			(
				w.write::<components::Solid>(),
				w.write::<components::Player>(),
			),(
				w.write::<components::Position>(),
				w.write::<components::Collision>(),
				w.write::<components::CollisionCache>(),
			),(
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

		let mut players          = players.pass();
		let mut locals           = locals.pass();
		let mut positions        = positions.pass();
		let mut collisions       = collisions.pass();
		let mut collision_caches = collision_caches.pass();

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

struct Game;
impl State for Game{
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
			points : 0,
			time: 0.0,
		});
		world.add_resource::<InputHandler>(InputHandler::new());

		//Generate a square mesh
		assets.register_asset::<Mesh>();
		assets.register_asset::<Texture>();
		assets.load_asset_from_data::<Texture, [f32; 4]>("white", [1.0, 1.0, 1.0, 1.0]);
		let square_verts = util::gen_rectangle(1.0, 1.0);
		assets.load_asset_from_data::<Mesh, Vec<VertexPosNormal>>("square", square_verts);
		let square = assets.create_renderable("square", "white", "white", "white", 1.0).unwrap();

		//Create a floor
		{
			world.create_now()
				.with(square.clone())
				.with(components::Solid{typ: SolidType::Solid,friction: 0.5})
				.with(components::Position(Vector2::new(400.0,400.0)))
				.with(components::CollisionCache::new())
				.with(components::Collision{
					velocity      : Vector2::new(0.0,0.0),
					acceleration  : Vector2::new(0.0,0.0),
					shape         : ShapeHandle2::new(Cuboid::new(Vector2::new(150.0, 16.0))),
					check_movement: false,
				})
				.with(LocalTransform::default())
				.with(Transform::default())
				.build();
		}

		//Create player
		{
			world.create_now()
				.with(square.clone())
				.with(components::Player{id: 0})
				.with(components::Position(Vector2::new(500.0,100.0)))
				.with(components::CollisionCache::new())
				.with(components::Collision{
					velocity      : Vector2::new(10.0,10.0),
					acceleration  : Vector2::new(0.0,400.0),
					shape         : ShapeHandle2::new(Cuboid::new(Vector2::new(32.0, 32.0))),
					check_movement: true,
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
}

fn main(){
	let mut game = Application::build(
		Game,
		{
			let mut cfg = DisplayConfig::default();
			cfg.title          = "STW3".to_owned();
			cfg.dimensions = Some((640,480));
			cfg.min_dimensions = Some((640,480));
			cfg
		}
	)
		.register::<components::Solid>()
		.register::<components::Player>()
		.register::<components::Position>()
		.register::<components::Collision>()
		.register::<components::CollisionCache>()
		.with::<MainSystem>(MainSystem, "main_system", 1)
    .with::<systems::InputSystem>(systems::InputSystem, "input_system", 1)
    .with::<systems::RenderSystem>(systems::RenderSystem, "render_system", 1)
		.done();
	game.run();
}
