use amethyst;
use amethyst::assets::{AssetFuture,BoxedErr};
use amethyst_renderer::vertex::PosNormTex;
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

	let norm2 = v.norm_squared();
	if norm2 == 0.0{return v;} //Avoid division by 0 in v.normalize()

	let x2 = x*x;

	if norm2 < x2{
		//Prevent switching signs when lengthening
		Vector2::new(0.0,0.0)
	}else{
		//Add/remove to the length
		v + v.normalize().multiply_by(x)
	}
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
