extern crate alga;
extern crate amethyst;
extern crate nalgebra;
extern crate ncollide;

mod components;
mod data;
mod util;
mod systems;

use amethyst::{Application, Event, State, Trans, VirtualKeyCode, WindowEvent};
use amethyst::asset_manager::AssetManager;
use amethyst::ecs::World;
use amethyst::ecs::components::{Mesh, LocalTransform, Texture, Transform};
use amethyst::gfx_device::DisplayConfig;
use amethyst::renderer::{Pipeline, VertexPosNormal};
use nalgebra::Vector2;
use ncollide::shape::{Cuboid,ShapeHandle2};

use data::*;

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
    .with::<systems::InputSystem>(systems::InputSystem, "input_system", 1)
    .with::<systems::RenderSystem>(systems::RenderSystem, "render_system", 1)
    .with::<systems::PhysicsSystem>(systems::PhysicsSystem, "physics_system", 1)
		.done();
	game.run();
}
