use amethyst::renderer::VertexPosNormal;
use nalgebra::Vector2;

pub fn gen_rectangle(w: f32,h: f32) -> Vec<VertexPosNormal>{
	vec![
		VertexPosNormal{
			pos: [-w / 2., -h / 2., 0.],
			normal: [0., 0., 1.],
			tex_coord: [0., 0.],
		},
		VertexPosNormal{
			pos: [w / 2., -h / 2., 0.],
			normal: [0., 0., 1.],
			tex_coord: [1., 0.],
		},
		VertexPosNormal{
			pos: [w / 2., h / 2., 0.],
			normal: [0., 0., 1.],
			tex_coord: [1., 1.],
		},
		VertexPosNormal{
			pos: [w / 2., h / 2., 0.],
			normal: [0., 0., 1.],
			tex_coord: [1., 1.],
		},
		VertexPosNormal{
			pos: [-w / 2., h / 2., 0.],
			normal: [0., 0., 1.],
			tex_coord: [1., 1.],
		},
		VertexPosNormal{
			pos: [-w / 2., -h / 2., 0.],
			normal: [0., 0., 1.],
			tex_coord: [1., 1.],
		}
	]
}

pub fn vector_lengthen(v: Vector2<f64>,x: f64) -> Vector2<f64>{
	use alga::general::AbstractModule;

	let norm2 = v.norm_squared();
	let x2 = x*x;

	if norm2 < x2{
		//Prevent switching signs when lengthening
		Vector2::new(0.0,0.0)
	}else{
		//Add/remove to the length
		v + v.normalize().multiply_by(x)
	}
}
