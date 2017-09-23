#![feature(associated_consts)]

extern crate alga;
extern crate amethyst;
extern crate amethyst_renderer;
extern crate nalgebra;
extern crate ncollide;
extern crate futures;

mod components;
mod data;
mod util;
mod states;
mod systems;

use amethyst::Application;
use amethyst_renderer::Config as DisplayConfig;
use amethyst_renderer::{Stage, Pipeline, pass};
use amethyst_renderer::vertex::PosNormTex;
use amethyst::ecs::transform::TransformSystem;

fn main(){
	let cfg = {
		let mut cfg = DisplayConfig::default();
		cfg.title          = "STW3".to_owned();
		cfg.dimensions     = Some((640,480));
		cfg.min_dimensions = Some((640,480));
		cfg.multisampling  = 0;
		cfg
	};
	let mut game =
		Application::build(
			states::Ingame
		)
		.unwrap()
		.register::<components::Solid>()
		.register::<components::Player>()
		.register::<components::Position>()
		.register::<components::Collision>()
		.register::<components::CollisionCache>()
		.with::<systems::ingame::PlayerInput>(systems::ingame::PlayerInput, "input_system", &[])
		.with::<systems::ingame::Physics>(systems::ingame::Physics, "physics_system", &[])
		.with::<systems::ingame::Render>(systems::ingame::Render, "render_system", &[])
		.with::<TransformSystem>(TransformSystem::new(), "transform_system", &["physics_system"])
		.with_renderer(
			Pipeline::build().with_stage(
				Stage::with_backbuffer()
					.clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
					.with_model_pass(pass::DrawFlat::<PosNormTex>::new())
					.with_model_pass(pass::DrawShaded::<PosNormTex>::new())
			),
			Some(cfg)
		)
		.unwrap();
	game.build().unwrap().run();
}
