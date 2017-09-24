#![allow(dead_code)]

use amethyst;
use amethyst::assets::{AssetFuture,BoxedErr,Context,Format,Loader};
use amethyst::ecs::rendering::{MaterialComponent,TextureContext};
use amethyst::renderer::vertex::PosNormTex;
use futures::{Future,IntoFuture};
use nalgebra::Vector2;

pub fn gen_rectangle_glvertices(w: f32,h: f32) -> Vec<PosNormTex>{
	vec![
		PosNormTex {
			a_position: [-w / 2.0, -h / 2.0, 0.0],
			a_normal: [0.0, 0.0, 1.0],
			a_tex_coord: [0.0, 0.0],
		},
		PosNormTex {
			a_position: [w / 2.0, h / 2.0, 0.0],
			a_normal: [0.0, 0.0, 1.0],
			a_tex_coord: [1.0, 1.0],
		},
		PosNormTex {
			a_position: [w / 2.0, -h / 2.0, 0.0],
			a_normal: [0.0, 0.0, 1.0],
			a_tex_coord: [1.0, 0.0],
		},

		PosNormTex {
			a_position: [w / 2.0, h / 2.0, 0.0],
			a_normal: [0.0, 0.0, 1.0],
			a_tex_coord: [1.0, 1.0],
		},
		PosNormTex {
			a_position: [-w / 2.0, -h / 2.0, 0.0],
			a_normal: [0.0, 0.0, 1.0],
			a_tex_coord: [0.0, 0.0],
		},
		PosNormTex {
			a_position: [-w / 2.0, h / 2.0, 0.0],
			a_normal: [0.0, 0.0, 1.0],
			a_tex_coord: [0.0, 1.0],
		},
	]
}

pub fn vector_lengthen(v: Vector2<f64>,x: f64) -> Vector2<f64>{
	use alga::general::AbstractModule;
	use nalgebra::zero;

	if v.norm_squared() <= x*x{
		//Prevent switching signs when lengthening and avoids division by 0 in v.normalize()
		zero()
	}else{
		//Add/remove to the length
		v + v.normalize().multiply_by(x)
	}
}

#[inline(always)]
pub fn vector_perpendicular(v: Vector2<f64>) -> Vector2<f64>{
	Vector2::new(-v[1],v[0])
}

pub fn load_proc_asset<T,F>(engine: &mut amethyst::Engine,f: F) -> AssetFuture<T::Item> where
	T: IntoFuture<Error = BoxedErr>,
	T::Future: 'static,
	F: FnOnce(&mut amethyst::Engine) -> T,
{
	let future = f(engine).into_future();
	let future: Box<Future<Item = T::Item , Error = BoxedErr>> = Box::new(future);
	AssetFuture(future.shared())
}

pub fn load_material<F>(engine: &mut amethyst::Engine,albedo: &str,format: F) -> AssetFuture<MaterialComponent> where
	F: Format + 'static,
	F::Data: Into<<TextureContext as Context>::Data>,
{
	use amethyst::ecs::rendering::{Factory,TextureComponent};
	use amethyst::renderer::MaterialBuilder;

	let future = engine.world.read_resource::<Factory>()
		.create_material(MaterialBuilder::new())
		.map_err(BoxedErr::new)
		.join(engine.world.read_resource::<Loader>().load_from::<TextureComponent,_,_,_>(albedo,format,"resources"))
		.map(|(mut mtl,albedo)|{
			mtl.albedo = albedo.0.inner();
			MaterialComponent(mtl)
		});
	AssetFuture::from_future(future)
}
