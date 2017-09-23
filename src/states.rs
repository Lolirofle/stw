use amethyst::{State, Trans, Engine};
use amethyst::ecs::World;
use amethyst::ecs::transform::{Transform, LocalTransform, Child, Init, TransformSystem};
use amethyst::event::{Event, WindowEvent, VirtualKeyCode, KeyboardInput};//, Camera, InputHandler, Projection, ScreenDimensions};
use amethyst::timing::Time;
use amethyst::input::{InputHandler};
use amethyst::ecs::rendering::{MeshComponent, MaterialComponent, Factory};
use amethyst_renderer::{Mesh, Texture, Pipeline, VertexFormat, Projection, Camera, MaterialBuilder};
use amethyst::assets::{AssetFuture, BoxedErr};
use nalgebra::Vector2;
use ncollide::shape::{Cuboid,ShapeHandle2};
use futures::{Future, IntoFuture};
use std::sync::Arc;

use *;
use util::gen_rectangle_glvertices;

fn load_proc_asset<T, F>(engine: &mut Engine, f: F) -> AssetFuture<T::Item>
where
    T: IntoFuture<Error = BoxedErr>,
    T::Future: 'static,
    F: FnOnce(&mut Engine) -> T,
{
    let future = f(engine).into_future();
    let future: Box<Future<Item = T::Item, Error = BoxedErr>> = Box::new(future);
    AssetFuture(future.shared())
}

pub struct Ingame;
impl State for Ingame{
	fn on_start(&mut self, engine: &mut Engine){
		//Generate a square mesh
		let tex = Texture::from_color_val([1.0, 1.0, 1.0, 1.0]);
		let mtl = MaterialBuilder::new().with_albedo(tex);
		let square_verts = gen_rectangle_glvertices(1.0, 1.0);
		let mesh = Mesh::build(square_verts);

		let square = load_proc_asset(engine, move |engine| {
            let factory = engine.world.read_resource::<Factory>();
            factory.create_mesh(mesh).map(MeshComponent::new).map_err(
                BoxedErr::new,
            )
        });

		let mtl = load_proc_asset(engine, move |engine| {
            let factory = engine.world.read_resource::<Factory>();
            factory
                .create_material(mtl)
                .map(MaterialComponent)
                .map_err(BoxedErr::new)
		});

		let world = &mut engine.world;
		let camera = {
			let eye    = [0.0, 0.0, 0.1];
			let forward = [0.0, 0.0, -1.0];
			let up     = [0.0, 1.0, 0.0];
			let right  = [1.0, 0.0, 0.0];

			//Get an Orthographic projection
			let proj = Projection::orthographic(0.0, 1.0, 1.0, 0.0).into();

			Camera {
				eye: eye.into(),
				proj: proj,
				up: up.into(),
				right: right.into(),
				forward: forward.into()
			}
		};

		//Add all resources
		world.add_resource(data::Score::new());
		world.add_resource(InputHandler::new());
		world.add_resource(Time::default());

		world.register::<Child>();
		world.register::<Init>();
		world.register::<LocalTransform>();

		//Create a floor
		{
			world.create_entity()
				.with(square.clone())
				.with(mtl.clone())
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
				.with(mtl.clone())
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
				.with(mtl.clone())
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

	fn handle_event(&mut self, _ : &mut Engine, event: Event) -> Trans{
		match event {
			Event::WindowEvent { event, .. } => {
				match event {
					WindowEvent::KeyboardInput {
						input: KeyboardInput { virtual_keycode: Some(VirtualKeyCode::Escape), ..}, ..
					} |
					WindowEvent::Closed => Trans::Quit,
					WindowEvent::KeyboardInput {
						input: KeyboardInput { virtual_keycode: Some(VirtualKeyCode::Return), ..}, ..
					} => Trans::Push(Box::new(states::Pause)),
					_ => Trans::None,
				}
			},
			_ => Trans::None,
		}
	}
}

pub struct Pause; //TODO: The world does not suspend when doing a state transition (push)
impl State for Pause{
	fn on_start(&mut self, _: &mut Engine) {
	}

	fn handle_event(&mut self, _ : &mut Engine, event: Event) -> Trans {
		match event {
			Event::WindowEvent { event, .. } => {
				match event {
					WindowEvent::KeyboardInput {
						input: KeyboardInput { virtual_keycode: Some(VirtualKeyCode::Escape), ..}, ..
					} |
					WindowEvent::Closed => Trans::Quit,
					WindowEvent::KeyboardInput {
						input: KeyboardInput { virtual_keycode: Some(VirtualKeyCode::Return), ..}, ..
					} => Trans::Pop,
					_ => Trans::None,
				}
			},
			_ => Trans::None,
		}
	}
}