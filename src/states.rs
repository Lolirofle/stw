use amethyst::{State,Trans,Engine};
use amethyst::ecs::transform::{Transform,LocalTransform,Child,Init};
use amethyst::event::{Event,WindowEvent,VirtualKeyCode,KeyboardInput};
use amethyst::timing::Time;
use amethyst::input::InputHandler;
use amethyst::ecs::rendering::{MeshComponent,MaterialComponent,Factory};
use amethyst_renderer::{Mesh,Texture,Projection,Camera,MaterialBuilder};
use amethyst::assets::BoxedErr;

use *;

pub struct Ingame;
impl Ingame{
	pub fn update_camera(engine : &mut Engine,camera: data::Camera){
		engine.world.add_resource(Camera{
			eye    : [0.0, 0.0, 1.0].into(),
			proj   : Projection::orthographic(
				(camera.translate[0] as f32),
				(camera.translate[0] as f32) + (camera.size[0] as f32),
				(camera.translate[1] as f32),
				(camera.translate[1] as f32) + (camera.size[1] as f32),
			).into(),
			forward: [0.0, 0.0,-1.0].into(),
			right  : [1.0, 0.0, 0.0].into(),
			up     : [0.0, 1.0, 0.0].into(),
		});
	}
}
impl State for Ingame{
	fn on_start(&mut self,engine: &mut Engine){
		use futures::Future;
		use nalgebra::{Vector2,zero};
		use ncollide::shape::{Cuboid,ShapeHandle2};

		//Generate a square mesh
		let tex = Texture::from_color_val([1.0 , 1.0 , 1.0 , 1.0]);
		let mtl = MaterialBuilder::new().with_albedo(tex);

		let square_verts = util::gen_rectangle_glvertices(1.0,1.0);
		let mesh = Mesh::build(square_verts);

		let square = util::load_proc_asset(engine,move |engine|{
			let factory = engine.world.read_resource::<Factory>();
			factory.create_mesh(mesh).map(MeshComponent::new).map_err(
				BoxedErr::new,
			)
		});

		let mtl = util::load_proc_asset(engine,move |engine|{
			let factory = engine.world.read_resource::<Factory>();
			factory
				.create_material(mtl)
				.map(MaterialComponent)
				.map_err(BoxedErr::new)
		});

		//Add all resources
		engine.world.add_resource(data::Score::new());
		engine.world.add_resource(data::Camera::new());
		engine.world.add_resource(InputHandler::new());
		engine.world.add_resource(Time::default());

		engine.world.register::<Child>();
		engine.world.register::<Init>();
		engine.world.register::<LocalTransform>();

		//Create a floor
		{
			engine.world.create_entity()
				.with(square.clone())
				.with(mtl.clone())
				.with(components::Position(zero()))
				.with(components::Solid::new(
					data::SolidType::Solid,
					false,
					false,
					240.0,
					ShapeHandle2::new(Cuboid::new(Vector2::new(300.0,16.0))),
				))
				.with(LocalTransform::default())
				.with(Transform::default())
				.build();
		}

		//Create a floor
		{
			engine.world.create_entity()
				.with(square.clone())
				.with(mtl.clone())
				.with(components::Position(Vector2::new(640.0,480.0)))
				.with(components::Solid::new(
					data::SolidType::Solid,
					false,
					false,
					240.0,
					ShapeHandle2::new(Cuboid::new(Vector2::new(150.0,16.0))),
				))
				.with(LocalTransform::default())
				.with(Transform::default())
				.build();
		}

		//Create a floor
		{
			engine.world.create_entity()
				.with(square.clone())
				.with(mtl.clone())
				.with(components::Position(Vector2::new(200.0,400.0)))
				.with(components::Solid::new(
					data::SolidType::Solid,
					false,
					false,
					240.0,
					ShapeHandle2::new(Cuboid::new(Vector2::new(150.0,16.0))),
				))
				.with(LocalTransform::default())
				.with(Transform::default())
				.build();
		}

		//Create a slippery floor
		{
			engine.world.create_entity()
				.with(square.clone())
				.with(mtl.clone())
				.with(components::Position(Vector2::new(420.0,360.0)))
				.with(components::Solid::new(
					data::SolidType::Solid,
					false,
					false,
					30.0,
					ShapeHandle2::new(Cuboid::new(Vector2::new(100.0,16.0))),
				))
				.with(LocalTransform::default())
				.with(Transform::default())
				.build();
		}

		//Create player
		{
			engine.world.create_entity()
				.with(square.clone())
				.with(mtl.clone())
				.with(components::Player{id: 0})
				.with(components::Position(Vector2::new(500.0,100.0)))
				.with(components::CollisionCache::new())
				.with(components::Solid::new(
					data::SolidType::Solid,
					true,
					true,
					1000.0,
					ShapeHandle2::new(Cuboid::new(Vector2::new(16.0,32.0))),
				))
				.with(LocalTransform::default())
				.with(Transform::default())
				.build();
		}

		//Create player
		/*{
			engine.world.create_entity()
				.with(square.clone())
				.with(mtl.clone())
				.with(components::Player{id: 1})
				.with(components::Position(Vector2::new(600.0,100.0)))
				.with(components::CollisionCache::new())
				.with(components::Solid::new(
					data::SolidType::Solid,
					true,
					true,
					4000.0,
					ShapeHandle2::new(Cuboid::new(Vector2::new(16.0,32.0))),
				))
				.with(LocalTransform::default())
				.with(Transform::default())
				.build();
		}*/
	}

	fn handle_event(&mut self,engine : &mut Engine,event: Event) -> Trans{
		match event{
			Event::WindowEvent{ event,..} =>{
				use amethyst::event::ElementState::*;
				match event{
					WindowEvent::KeyboardInput{input: KeyboardInput{ virtual_keycode: Some(VirtualKeyCode::Escape),..},..} |
					WindowEvent::Closed =>
						Trans::Quit,

					WindowEvent::KeyboardInput{input: KeyboardInput{ virtual_keycode: Some(VirtualKeyCode::Return),state: Pressed,..},..} =>
						Trans::Push(Box::new(states::Pause)),

					WindowEvent::KeyboardInput{input: KeyboardInput{ virtual_keycode: Some(VirtualKeyCode::Home),state: Pressed,..},..} => {
						let data = {
							let mut camera_data = engine.world.write_resource::<data::Camera>();
							camera_data.translate.y-= 16.0;
							camera_data.clone()
						};
						Self::update_camera(engine,data);
						Trans::None
					},

					WindowEvent::KeyboardInput{input: KeyboardInput{ virtual_keycode: Some(VirtualKeyCode::End),state: Pressed,..},..} => {
						let data = {
							let mut camera_data = engine.world.write_resource::<data::Camera>();
							camera_data.translate.y+= 16.0;
							camera_data.clone()
						};
						Self::update_camera(engine,data);
						Trans::None
					},

					WindowEvent::KeyboardInput{input: KeyboardInput{ virtual_keycode: Some(VirtualKeyCode::Delete),state: Pressed,..},..} => {
						let data = {
							let mut camera_data = engine.world.write_resource::<data::Camera>();
							camera_data.translate.x-= 16.0;
							camera_data.clone()
						};
						Self::update_camera(engine,data);
						Trans::None
					},

					WindowEvent::KeyboardInput{input: KeyboardInput{ virtual_keycode: Some(VirtualKeyCode::PageDown),state: Pressed,..},..} => {
						let data = {
							let mut camera_data = engine.world.write_resource::<data::Camera>();
							camera_data.translate.x+= 16.0;
							camera_data.clone()
						};
						Self::update_camera(engine,data);
						Trans::None
					},

					WindowEvent::Resized(w,h) => {
						let data = {
							let mut camera_data = engine.world.write_resource::<data::Camera>();
							camera_data.size.x = w as f64;
							camera_data.size.y = h as f64;
							camera_data.clone()
						};
						Self::update_camera(engine,data);
						Trans::None
					}

					_ => Trans::None,
				}
			},
			_ => Trans::None,
		}
	}
}

pub struct Pause; //TODO: The world does not suspend when doing a state transition (push)
impl State for Pause{
	fn on_start(&mut self,_: &mut Engine){}

	fn on_stop(&mut self,_: &mut Engine){}

	fn handle_event(&mut self,_ : &mut Engine,event: Event) -> Trans{
		match event{
			Event::WindowEvent{ event,..} =>{
				use amethyst::event::ElementState::*;
				match event{
					WindowEvent::KeyboardInput{ input: KeyboardInput{ virtual_keycode: Some(VirtualKeyCode::Escape),..},..} |
					WindowEvent::Closed =>
						Trans::Quit,
					WindowEvent::KeyboardInput{ input: KeyboardInput{ virtual_keycode: Some(VirtualKeyCode::Return),state: Pressed,..},..} =>
						Trans::Pop,
					_ => Trans::None,
				}
			},
			_ => Trans::None,
		}
	}
}
