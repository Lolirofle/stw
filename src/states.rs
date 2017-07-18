use amethyst::{Event, State, Trans, VirtualKeyCode, WindowEvent};
use amethyst::asset_manager::AssetManager;
use amethyst::ecs::World;
use amethyst::ecs::components::{Mesh, LocalTransform, Texture, Transform};
use amethyst::renderer::{Pipeline, VertexPosNormal};
use nalgebra::Vector2;
use ncollide::shape::{Cuboid,ShapeHandle2};

use *;

pub struct Ingame;
impl State for Ingame{
	fn on_start(&mut self, world: &mut World, assets: &mut AssetManager, pipe: &mut Pipeline){
		use amethyst::ecs::resources::{Camera, InputHandler, Projection, ScreenDimensions};
		use amethyst::renderer::Layer;
		use amethyst::renderer::pass::{Clear, DrawFlat};

		let layer = Layer::new("main",vec![
			Clear::new([0.0, 0.0, 0.0, 1.0]),
			DrawFlat::new("main","main")
		]);

		pipe.layers.push(layer);

		{
			let dim = world.read_resource::<ScreenDimensions>();
			let mut camera = world.write_resource::<Camera>();
			let eye    = [0.0, 0.0, 0.1];
			let target = [0.0, 0.0, 0.0];
			let up     = [0.0, 1.0, 0.0];

			//Get an Orthographic projection
			let proj = Projection::Orthographic{
				left  :  0.0 ,
				right :  dim.w,
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
		world.add_resource::<data::Score>(data::Score::new());
		world.add_resource::<InputHandler>(InputHandler::new());

		//Generate a square mesh
		assets.register_asset::<Mesh>();
		assets.register_asset::<Texture>();
		assets.load_asset_from_data::<Texture, [f32; 4]>("white", [1.0, 1.0, 1.0, 1.0]);
		assets.load_asset_from_data::<Mesh, Vec<VertexPosNormal>>("square", util::gen_rectangle_glvertices(1.0, 1.0));
		let square = assets.create_renderable("square", "white", "white", "white", 1.0).unwrap();

		//Create a floor
		{
			world.create_entity()
				.with(square.clone())
				.with(components::Solid{typ: data::SolidType::Solid,friction: 240.0})
				.with(components::Position(Vector2::new(200.0,400.0)))
				.with(components::CollisionCache::new())
				.with(components::Collision{
					velocity         : Vector2::new(0.0,0.0),
					acceleration     : Vector2::new(0.0,0.0),
					shape            : ShapeHandle2::new(Cuboid::new(Vector2::new(150.0, 16.0))),
					check_movement   : false,
					gravity          : false,
				})
				.with(LocalTransform::default())
				.with(Transform::default())
				.build();
		}

		//Create a slippery floor
		{
			world.create_entity()
				.with(square.clone())
				.with(components::Solid{typ: data::SolidType::Solid,friction: 60.0})
				.with(components::Position(Vector2::new(420.0,360.0)))
				.with(components::CollisionCache::new())
				.with(components::Collision{
					velocity         : Vector2::new(0.0,0.0),
					acceleration     : Vector2::new(0.0,0.0),
					shape            : ShapeHandle2::new(Cuboid::new(Vector2::new(100.0, 16.0))),
					check_movement   : false,
					gravity          : false,
				})
				.with(LocalTransform::default())
				.with(Transform::default())
				.build();
		}

		//Create player
		{
			world.create_entity()
				.with(square.clone())
				.with(components::Player{id: 0})
				.with(components::Position(Vector2::new(500.0,100.0)))
				.with(components::CollisionCache::new())
				.with(components::Collision{
					velocity         : Vector2::new(10.0,10.0),
					acceleration     : Vector2::new(0.0,0.0),
					shape            : ShapeHandle2::new(Cuboid::new(Vector2::new(16.0, 32.0))),
					check_movement   : true,
					gravity          : true,
				})
				.with(LocalTransform::default())
				.with(Transform::default())
				.build();
		}
	}

	fn handle_events(&mut self,events: &[WindowEvent],world: &mut World,_: &mut AssetManager,_: &mut Pipeline) -> Trans{
		use amethyst::ecs::resources::InputHandler;
		use amethyst::ElementState;

		let mut input = world.write_resource::<InputHandler>();
		input.update(events);

		let mut state_transition = Trans::None;
		for e in events{
			match **e{
				Event::KeyboardInput(_,_,Some(VirtualKeyCode::Escape)) |
				Event::Closed => {
					state_transition = Trans::Quit;
				},
				Event::KeyboardInput(ElementState::Pressed,_,Some(VirtualKeyCode::Return)) => {
					state_transition = Trans::Push(Box::new(states::Pause));
				},
				_ => {},
			}
		}

		state_transition
	}
}

pub struct Pause; //TODO: The world does not suspend when doing a state transition (push)
impl State for Pause{
	fn on_start(&mut self, _world: &mut World, _assets: &mut AssetManager, pipe: &mut Pipeline){
		use amethyst::renderer::Layer;
		use amethyst::renderer::pass::{Clear, DrawFlat};

		let layer = Layer::new("main",vec![
			Clear::new([0.2, 0.2, 0.2, 1.0]),
			DrawFlat::new("main", "main")
		]);

		pipe.layers.push(layer);
	}

	fn handle_events(&mut self,events: &[WindowEvent],world: &mut World,_: &mut AssetManager,pipe: &mut Pipeline) -> Trans{
		use amethyst::ecs::resources::InputHandler;
		use amethyst::ElementState;

		let mut input = world.write_resource::<InputHandler>();
		input.update(events);

		let mut state_transition = Trans::None;
		for e in events{
			match **e{
				Event::KeyboardInput(_,_,Some(VirtualKeyCode::Escape)) |
				Event::Closed => {
					state_transition = Trans::Quit;
				},
				Event::KeyboardInput(ElementState::Pressed,_,Some(VirtualKeyCode::Return)) => {
					state_transition = Trans::Pop;
					pipe.layers.pop();
				},
				_ => {},
			}
		}

		state_transition
	}
}
