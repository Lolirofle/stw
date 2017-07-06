extern crate alga;
extern crate amethyst;
extern crate nalgebra;
extern crate ncollide;

mod components;
mod data;
mod util;
mod states;
mod systems;

use amethyst::Application;
use amethyst::gfx_device::DisplayConfig;

fn main(){
	let mut game =
		Application::build(
			states::Ingame,
			{
				let mut cfg = DisplayConfig::default();
				cfg.title          = "STW3".to_owned();
				cfg.dimensions     = Some((640,480));
				cfg.min_dimensions = Some((640,480));
				cfg
			}
		)
		.register::<components::Solid>()
		.register::<components::Player>()
		.register::<components::Position>()
		.register::<components::Collision>()
		.register::<components::CollisionCache>()
		.with::<systems::ingame::PlayerInput>(systems::ingame::PlayerInput, "input_system", 1)
		.with::<systems::ingame::Physics>(systems::ingame::Physics, "physics_system", 1)
		.with::<systems::ingame::Render>(systems::ingame::Render, "render_system", 1)
		.done();
	game.run();
}
