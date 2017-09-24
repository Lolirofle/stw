extern crate alga;
extern crate amethyst;
extern crate nalgebra;
extern crate ncollide;
extern crate futures;

mod components;
mod data;
mod util;
mod states;
mod systems;

use amethyst::Application;
use amethyst::assets::Directory;
use amethyst::ecs::rendering::{MeshComponent,MaterialComponent,RenderBundle};
use amethyst::ecs::transform::{Transform,TransformSystem};
use amethyst::renderer::Config as DisplayConfig;
use amethyst::renderer::prelude::*;

type DrawFlat = pass::DrawFlat<PosNormTex,MeshComponent,MaterialComponent,Transform>;

fn main(){
	Application::build(states::Ingame)
		.unwrap()
		.register::<components::Solid>()
		.register::<components::Player>()
		.register::<components::Position>()
		.register::<components::CollisionCache>()
		.with::<systems::ingame::PlayerInput>(systems::ingame::PlayerInput, "input_system", &[])
		.with::<systems::ingame::Physics>(systems::ingame::Physics::new(), "physics_system", &[])
		.with::<systems::ingame::Render>(systems::ingame::Render, "render_system", &[])
		.with::<TransformSystem>(TransformSystem::new(), "transform_system", &["physics_system"])
		.with_store("resources", Directory::new("resources"))
		.with_bundle(
			RenderBundle::new(Pipeline::build().with_stage(
				Stage::with_backbuffer()
					.clear_target([0.05, 0.05, 0.05, 1.0], 1.0)
					.with_pass(DrawFlat::new())
			)).with_config(DisplayConfig{
				title         : "STW3".to_owned(),
				dimensions    : Some((640,480)),
				min_dimensions: Some((640,480)),
				multisampling : 0,
				..DisplayConfig::default()
			})
		)
		.unwrap().build().unwrap().run();
}
