
use components;

use amethyst::VirtualKeyCode;
use amethyst::ecs::{World, Join, RunArg, System};
use std::ops::Deref;
use amethyst::ecs::Gate;

pub struct InputSystem;
unsafe impl Sync for InputSystem {}
impl System<()> for InputSystem {
  fn run(&mut self, arg: RunArg, _: ()) {
    use amethyst::ecs::resources::InputHandler;

    let (collisions, players, input) = arg.fetch(|w| {(
      w.write::<components::Collision>(),
      w.write::<components::Player>(),
      w.read_resource::<InputHandler>(),
    )});

    let mut players = players.pass();
    let mut collisions = collisions.pass();
    for(
			ref mut player,
			&mut components::Collision{ref mut velocity,..},
		) in (
			&mut players,
			&mut collisions,
		).join(){
			match player.id{
				1 =>{
				  if input.key_down(VirtualKeyCode::W){
					  velocity[1] = -200.0;
				  }
				  if input.key_down(VirtualKeyCode::A){
					  velocity[0] = -100.0;
				  }
				  if input.key_down(VirtualKeyCode::D){
					  velocity[0] = 100.0;
				  }
			  }
			  0 =>{
			    if input.key_down(VirtualKeyCode::Up){
				    velocity[1] = -200.0;
			    }
			    if input.key_down(VirtualKeyCode::Left){
				    velocity[0] = -100.0;
			    }
			    if input.key_down(VirtualKeyCode::Right){
				    velocity[0] = 100.0;
			    }
		    }
		    _ => {}
      };
		}
  }
}

pub struct RenderSystem;
unsafe impl Sync for RenderSystem {}
impl System<()> for RenderSystem {

  fn run(&mut self, arg: RunArg, _: ()) {
    use amethyst::ecs::components::LocalTransform;
    use nalgebra::Isometry2;

    let (collisions, positions, locals) = arg.fetch(|w| {(
      w.read::<components::Collision>(),
      w.read::<components::Position>(),
      w.write::<LocalTransform>()
    )});

    let collisions = collisions.pass();
    let positions = positions.pass();
    let mut locals = locals.pass();

    for(
			&components::Position(position),
			&components::Collision{ref shape,..},
			ref mut local
		) in (
			&positions,
			&collisions,
			&mut locals
		).join(){
			//Update the renderable corresponding to this entity
			let aabb         = shape.aabb(&Isometry2::new(position,0.0));
			let mins         = aabb.center();
			let len          = aabb.maxs() - aabb.mins();
			local.translation[0] = mins[0] as f32;
			local.translation[1] = mins[1] as f32;
			local.scale = [len[0] as f32, len[1] as f32, 1.0];
		}
  }
}
