use amethyst::renderer::VertexPosNormal;
use nalgebra::Vector2;

pub fn gen_rectangle_glvertices(w: f32,h: f32) -> Vec<VertexPosNormal>{
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

/*
use core::mem;

/**
 * The following is not possible:
 *   let a: i32 = 1;
 *   let v = vec![&mut a,&mut a];
 * because of aliasing rules.
 * Therefore, mutable references that iterators return must also be non-aliasing.
 * In other words, the iterator always returns unique (non-aliasing) references.
 * Therefore this should be safe? TODO: But multiple calls to next will return the same reference: (0,1),(0,2),(0,3),(1,2),(1,3),(2,3). Each occurence must go out of scope in the lifetime where `next` is called.
 */
#[derive(Debug)]
pub struct PartialPermutation2IterMut<'l,Iter,T>(Iter,Iter,Option<&'l mut T>) where
	Iter: Iterator<Item = &'l mut T>,
	T: 'l
;
impl<'l,Iter,T> PartialPermutation2IterMut<'l,Iter,T> where
	Iter: Iterator<Item = &'l mut T>,
	T: 'l
{
	pub fn new(mut i0: Iter) -> Self{
		let mut i1: Iter = unsafe{mem::transmute_copy(&i0)};
		let elem = i1.next();
		PartialPermutation2IterMut(i0,i1,elem)
	}
}
impl<'l,Iter,T> Iterator for PartialPermutation2IterMut<'l,Iter,T> where
	Iter: for<'a> Iterator<Item = &'a mut T>,
	T: 'l
{
	type Item = (Iter::Item,Iter::Item);

	fn next(&mut self) -> Option<Self::Item>{
		match (&mut self.1.next(),&mut self.2){
			(&mut Some(ref mut x0),&mut Some(ref mut x1)) => {
				return Some((x0,x1));
			},
			(&mut Some(x0),self2 @ &mut None) => {
				*self2 = self.0.next();
				self.1 = unsafe{mem::transmute_copy(&self.0)};
				self.next()
			},
			(&mut None,_) => {
				return None;
			},
		}
	}
}
*/
