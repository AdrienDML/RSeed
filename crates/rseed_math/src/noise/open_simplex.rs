/*
 * 2014 OpenSimplex Noise in Java.
 * by Kurt Spencer
 * 
 * Updated Dec 2019 and Feb 2020:
 * - New lattice-symmetric gradient sets
 * - Optional alternate lattice orientation evaluators
 * 
 * This implementation has been updated to slightly improve its output, but it is recommented to first
 * try the newer OpenSimplex2S or OpenSimplex2F noise. These are located in the OpenSimplex2 repo:
 * https://github.com/KdotJPG/OpenSimplex2
 * 
 * In the event that the output of this OpenSimplex continues to better fit your project's needs than
 * either OpenSimplex2 variant, an updated backport of DigitalShadow's optimization is available here:
 * https://github.com/KdotJPG/OpenSimplex2/blob/master/java/legacy/OpenSimplex.java
 * 
 * This is mostly kept here for reference. In particular, the 4D code is very slow.
 */

use crate::vec2d::Vec2D;
use crate::vec3d::Vec3D;
use crate::vec4d::Vec4D;
use crate::fct::fast_floor;
const PSIZE : usize = 2048;
const PMASK : usize = 2047;
const STRETCH_CONSTANT_2D : f64 = -0.211324865405187;
const SQUISH_CONSTANT_2D  : f64 = 0.366025403784439;  
const STRETCH_CONSTANT_3D : f64 = -1.0 / 6f64;          
const SQUISH_CONSTANT_3D  : f64 = 1.0 / 3f64;
#[allow(dead_code)]
const STRETCH_CONSTANT_4D : f64 = -0.138196601125011;
#[allow(dead_code)]
const SQUISH_CONSTANT_4D  : f64 = 0.309016994374947;  
const IN2 : f64 = 1f64 / 7.69084574549313; 
const IN3 : f64 = 1f64 / 26.92263139946168;
const IN4 : f64 = 1f64 / 8.881759591352166;
pub struct OpenSimplex {
	seed : i64,
	perm : [usize; PSIZE],
	perm2d : [Vec2D; PSIZE],
	perm3d : [Vec3D; PSIZE],
#[allow(dead_code)]
	perm4d : [Vec4D; PSIZE],
}

impl OpenSimplex {
	pub fn new(seed : i64) -> Self {
		let mut perm : [usize; PSIZE] = [0_usize; PSIZE];
		let mut s = seed;
		let mut source : [usize; PSIZE] = [0_usize; PSIZE];
		for i in 0..PSIZE {
			source[i] = i;
		} 
		for i in (0..=PSIZE-1).rev() {
			s = s.wrapping_mul(6364136223846793005_i64).wrapping_add(1442695040888963407_i64);
			let mut r = (seed + 31) % (i+1) as i64;
			if r < 0 {
				r += (i+1) as i64;
			} 
			perm[i] = source[r as usize];
		}
		
		let (perm2d, perm3d, perm4d) = Self::getperms(&perm);

		Self {
			seed : seed,
			perm : perm,
			perm2d : perm2d,
			perm3d : perm3d,
			perm4d : perm4d,
		}
	}

	pub fn get_seed(&self) -> i64 {
		self.seed
	}

	pub fn new_from_perm(perm : [usize; PSIZE]) -> Self {
		let (perm2d, perm3d, perm4d) = Self::getperms(&perm);
		Self {
			seed : 0,
			perm : perm,
			perm2d : perm2d,
			perm3d : perm3d,
			perm4d : perm4d,
		}
	}

	pub fn eval2(&self, point : &Vec2D) -> f64 {
		let strech_offset : f64 = (point.x + point.y) * STRETCH_CONSTANT_2D;
		let xs : f64 = point.x + strech_offset;
		let ys : f64 = point.y + strech_offset;

		let mut xsb : i32 = fast_floor(xs);
		let mut ysb : i32 = fast_floor(ys);

		let xins : f64 = xs - xsb as f64;
		let yins : f64 = ys - ysb as f64;
		
		let in_sum : f64 = xins + yins;

		let squish_offset_ins : f64 = in_sum * SQUISH_CONSTANT_2D;
		let mut dx0 : f64 = xins + squish_offset_ins;
		let mut dy0 : f64 = yins + squish_offset_ins;

		let dx_ext : f64; let dy_ext : f64;
		let xsv_ext : i32; let ysv_ext : i32;

		let mut value : f64 = 0_f64;

		let dx1 : f64 = dx0 - 1_f64 - SQUISH_CONSTANT_2D;
		let dy1 : f64 = dy0 - 0_f64 - SQUISH_CONSTANT_2D;
		let mut attn1 : f64 = 2_f64 - dx1 * dx1 - dy1 * dy1;
		if attn1 > 0_f64 {
			attn1 *= attn1;
			value += attn1 * attn1 * self.extrapolate2(xsb + 1, ysb + 0, dx1, dy1);
		}

		let dx2 = dx0 - 0_f64 - SQUISH_CONSTANT_2D;
		let dy2 = dy0 - 1_f64 - SQUISH_CONSTANT_2D;
		let mut attn2 = 2_f64 - dx2 * dx2 - dy2 * dy2;
		if attn2 > 0_f64 {
			attn2 *= attn2;
			value += attn2 * attn2 * self.extrapolate2(xsb + 0, ysb + 1, dx2, dy2);
		}

		if in_sum <= 1_f64 { // We're inside the triangle (2-Simplex) at (0,0)
			let zins : f64 = 1_f64 - in_sum;
			if zins > xins || zins > yins { // (0,0) is one of the closest two triangular vertices
				if xins > yins {
					xsv_ext = xsb + 1_i32;
					ysv_ext = ysb - 1_i32;
					dx_ext = dx0 - 1_f64;
					dy_ext = dy0 + 1_f64;
				} else {
					xsv_ext = xsb - 1_i32;
					ysv_ext = ysb + 1_i32;
					dx_ext = dx0 + 1_f64;
					dy_ext = dy0 - 1_f64;
				}
			} else { // (1,0) and (0,1) are the closest two vertices.
				xsv_ext = xsb + 1_i32;
				ysv_ext = ysb + 1_i32;
				dx_ext = dx0 - 1_f64 - 2_f64 * SQUISH_CONSTANT_2D;
				dy_ext = dy0 - 1_f64 - 2_f64 * SQUISH_CONSTANT_2D;
			}
		} else { // We're inside the triangle (2-Simplex) at (1,1)
			let zins : f64 = 2_f64 - in_sum;
			if zins < xins || zins < yins { // (0,0) is one of the closest two triangular vertices
				if xins > yins {
					xsv_ext = xsb + 2_i32;
					ysv_ext = ysb + 0_i32;
					dx_ext = dx0 - 2_f64 - 2_f64 * SQUISH_CONSTANT_2D;
					dy_ext = dy0 + 0_f64 - 2_f64 * SQUISH_CONSTANT_2D;
				} else {
					xsv_ext = xsb + 0_i32;
					ysv_ext = ysb + 2_i32;
					dx_ext = dx0 + 0_f64 - 2_f64 * SQUISH_CONSTANT_2D;
					dy_ext = dy0 - 2_f64 - 2_f64 * SQUISH_CONSTANT_2D;
				}
			} else { // (1,0) and (0,1) are the closest two vertices.
				dx_ext = dx0;
				dy_ext = dy0;
				xsv_ext = xsb;
				ysv_ext = ysb;
			}
			xsb += 1_32;
			ysb += 1_32;
			dx0 = dx0 - 1_f64 - 2_f64 * SQUISH_CONSTANT_2D;
			dy0 = dy0 - 1_f64 - 2_f64 * SQUISH_CONSTANT_2D;
		}

		// Contribution (0,0) or (1,1)
		let mut attn0 : f64 = 2_f64 - dx0 * dx0 - dy0 * dy0;
		if attn0 > 0_f64 {
			attn0 *= attn0;
			value += attn0 * attn0 * self.extrapolate2(xsb, ysb, dx0, dy0);
		}
		
		// Extra Vertex
		let mut attn_ext : f64 = 2_f64 - dx_ext * dx_ext - dy_ext * dy_ext;
		if attn_ext > 0_f64 {
			attn_ext *= attn_ext;
			value += attn_ext * attn_ext * self.extrapolate2(xsv_ext, ysv_ext, dx_ext, dy_ext);
		}

		value
	}

	pub fn eval3(&self, point : &Vec3D) -> f64 {
		// Place input coordinates on simplectic honeycomb.
		let stretch_offset = (point.x + point.y + point.z) * STRETCH_CONSTANT_3D;
		let xs : f64 = point.x + stretch_offset;
		let ys : f64 = point.y + stretch_offset;
		let zs : f64 = point.z + stretch_offset;
		
		// Floor to get simplectic honeycomb coordinates of rhombohedron (stretched cube) super-cell origin.
		let xsb : i32 = fast_floor(xs);
		let ysb : i32 = fast_floor(ys);
		let zsb : i32 = fast_floor(zs);
		
		// Compute simplectic honeycomb coordinates relative to rhombohedral origin.
		let xins : f64 = xs - xsb as f64;
		let yins : f64 = ys - ysb as f64;
		let zins : f64 = zs - zsb as f64;
		
		// Sum those together to get a value that determines which region we're in.
		let in_sum : f64 = xins + yins + zins;

		// Positions relative to origin point.
		let squish_offset_ins : f64 = in_sum * SQUISH_CONSTANT_3D;
		let mut dx0 : f64 = xins + squish_offset_ins;
		let mut dy0 : f64 = yins + squish_offset_ins;
		let mut dz0 : f64 = zins + squish_offset_ins;
		
		// We'll be defining these inside the next block and using them afterwards.
		let dx_ext0 : f64; let mut dy_ext0 : f64; let dz_ext0 : f64;
		let mut dx_ext1 : f64; let mut dy_ext1 : f64; let mut dz_ext1 : f64;
		let xsv_ext0 : i32; let mut ysv_ext0 : i32; let zsv_ext0 : i32;
		let mut xsv_ext1 : i32; let mut ysv_ext1 : i32; let mut zsv_ext1 : i32;
		
		let mut value : f64 = 0f64;
		if in_sum <= 1f64 { // We're inside the tetrahedron (3-Simplex) at (0,0,0)
			
			// Determine which two of (0,0,1), (0,1,0), (1,0,0) are closest.
			let mut a_point : i8 = 0x01;
			let mut a_score : f64 = xins;
			let mut b_point : i8 = 0x02;
			let mut b_score : f64 = yins;
			if a_score >= b_score && zins > b_score {
				b_score = zins;
				b_point = 0x04;
			} else if a_score < b_score && zins > a_score {
				a_score = zins;
				a_point = 0x04;
			}
			
			// Now we determine the two lattice points not part of the tetrahedron that may contribute.
			// This depends on the closest two tetrahedral vertices, including (0,0,0)
			let wins : f64 = 1f64 - in_sum;
			if (wins > a_score) || (wins > b_score) { // (0,0,0) is one of the closest two tetrahedral vertices.
				let c  : i8 = if b_score > a_score{b_point} else {a_point}; // Our other closest vertex is the closest out of a and b.
				
				if (c & 0x01) == 0 {
					xsv_ext0 = xsb - 1;
					xsv_ext1 = xsb;
					dx_ext0 = dx0 + 1f64;
					dx_ext1 = dx0;
				} else {
					xsv_ext0 = xsb + 1;
					xsv_ext1 = xsb + 1;
					dx_ext0 = dx0 - 1f64;
					dx_ext1 = dx0 - 1f64;
				}

				if (c & 0x02) == 0 {
					ysv_ext0 = ysb; ysv_ext1 = ysb;
					dy_ext0 = dy0; dy_ext1 = dy0;

					if (c & 0x01) == 0 {
						ysv_ext1 -= 1;
						dy_ext1 += 1f64;
					} else {
						ysv_ext0 -= 1;
						dy_ext0 += 1f64;
					}
				} else {
					ysv_ext0 = ysb + 1;
					ysv_ext1 = ysb + 1;
					dy_ext0 = dy0 - 1f64;
					dy_ext1 = dy0 - 1f64;
				}

				if (c & 0x04) == 0 {
					zsv_ext0 = zsb;
					zsv_ext1 = zsb - 1;
					dz_ext0 = dz0;
					dz_ext1 = dz0 + 1f64;
				} else {
					zsv_ext0 = zsb + 1;
					zsv_ext1 = zsb + 1;
					dz_ext0 = dz0 - 1f64;
					dz_ext1 = dz0 - 1f64;
				}
			} else { // (0,0,0) is not one of the closest two tetrahedral vertices.
				let c = a_point | b_point; // Our two extra vertices are determined by the closest two.
				
				if (c & 0x01) == 0 {
					xsv_ext0 = xsb;
					xsv_ext1 = xsb - 1;
					dx_ext0 = dx0 - 2f64 * SQUISH_CONSTANT_3D;
					dx_ext1 = dx0 + 1f64 - SQUISH_CONSTANT_3D;
				} else {
					xsv_ext0 = xsb + 1;
					xsv_ext1 = xsb + 1;
					dx_ext0 = dx0 - 1f64 - 2f64 * SQUISH_CONSTANT_3D;
					dx_ext1 = dx0 - 1f64 - SQUISH_CONSTANT_3D;
				}

				if (c & 0x02) == 0 {
					ysv_ext0 = ysb;
					ysv_ext1 = ysb - 1;
					dy_ext0 = dy0 - 2f64 * SQUISH_CONSTANT_3D;
					dy_ext1 = dy0 + 1f64 - SQUISH_CONSTANT_3D;
				} else {
					ysv_ext0 = ysb + 1;
					ysv_ext1 = ysb + 1;
					dy_ext0 = dy0 - 1f64 - 2f64 * SQUISH_CONSTANT_3D;
					dy_ext1 = dy0 - 1f64 - SQUISH_CONSTANT_3D;
				}

				if (c & 0x04) == 0 {
					zsv_ext0 = zsb;
					zsv_ext1 = zsb - 1;
					dz_ext0 = dz0 - 2f64 * SQUISH_CONSTANT_3D;
					dz_ext1 = dz0 + 1f64 - SQUISH_CONSTANT_3D;
				} else {
					zsv_ext0 = zsb + 1;
					zsv_ext1 = zsb + 1;
					dz_ext0 = dz0 - 1f64 - 2f64 * SQUISH_CONSTANT_3D;
					dz_ext1 = dz0 - 1f64 - SQUISH_CONSTANT_3D;
				}
			}

			// Contribution (0,0,0)
			let mut attn0 = 2f64 - dx0 * dx0 - dy0 * dy0 - dz0 * dz0;
			if attn0 > 0f64 {
				attn0 *= attn0;
				value += attn0 * attn0 * self.extrapolate3(xsb + 0, ysb + 0, zsb + 0, dx0, dy0, dz0);
			}

			// Contribution (1,0,0)
			let dx1 : f64 = dx0 - 1f64 - SQUISH_CONSTANT_3D;
			let dy1 : f64 = dy0 - 0f64 - SQUISH_CONSTANT_3D;
			let dz1 : f64 = dz0 - 0f64 - SQUISH_CONSTANT_3D;
			let mut attn1 : f64 = 2f64 - dx1 * dx1 - dy1 * dy1 - dz1 * dz1;
			if attn1 > 0f64 {
				attn1 *= attn1;
				value += attn1 * attn1 * self.extrapolate3(xsb + 1, ysb + 0, zsb + 0, dx1, dy1, dz1);
			}

			// Contribution (0,1,0)
			let dx2 : f64 = dx0 - 0f64 - SQUISH_CONSTANT_3D;
			let dy2 : f64 = dy0 - 1f64 - SQUISH_CONSTANT_3D;
			let dz2 : f64 = dz1;
			let mut attn2 : f64 = 2f64 - dx2 * dx2 - dy2 * dy2 - dz2 * dz2;
			if attn2 > 0f64 {
				attn2 *= attn2;
				value += attn2 * attn2 * self.extrapolate3(xsb + 0, ysb + 1, zsb + 0, dx2, dy2, dz2);
			}

			// Contribution (0,0,1)
			let dx3 : f64 = dx2;
			let dy3 : f64 = dy1;
			let dz3 : f64 = dz0 - 1f64 - SQUISH_CONSTANT_3D;
			let mut attn3 : f64 = 2f64 - dx3 * dx3 - dy3 * dy3 - dz3 * dz3;
			if attn3 > 0f64 {
				attn3 *= attn3;
				value += attn3 * attn3 * self.extrapolate3(xsb + 0, ysb + 0, zsb + 1, dx3, dy3, dz3);
			}
		} else if in_sum >= 2f64 { // We're inside the tetrahedron (3-Simplex) at (1,1,1)
		
			// Determine which two tetrahedral vertices are the closest, out of (1,1,0), (1,0,1), (0,1,1) but not (1,1,1).
			let mut a_point : i8 = 0x06;
			let mut a_score : f64 = xins;
			let mut b_point : i8 = 0x05;
			let mut b_score : f64 = yins;
			if (a_score <= b_score) && (zins < b_score) {
				b_score = zins;
				b_point = 0x03;
			} else if (a_score > b_score) && (zins < a_score) {
				a_score = zins;
				a_point = 0x03;
			}
			
			// Now we determine the two lattice points not part of the tetrahedron that may contribute.
			// This depends on the closest two tetrahedral vertices, including (1,1,1)
			let wins : f64 = 3f64 - in_sum;
			if (wins < a_score) || (wins < b_score) { // (1,1,1) is one of the closest two tetrahedral vertices.
				let c : i8 = if b_score < a_score { b_point } else {a_point}; // Our other closest vertex is the closest out of a and b.
				
				if (c & 0x01) != 0 {
					xsv_ext0 = xsb + 2;
					xsv_ext1 = xsb + 1;
					dx_ext0 = dx0 - 2f64 - 3f64 * SQUISH_CONSTANT_3D;
					dx_ext1 = dx0 - 1f64 - 3f64 * SQUISH_CONSTANT_3D;
				} else {
					xsv_ext0 = xsb;
					xsv_ext1 = xsb;
					dx_ext0 = dx0 - 3f64 * SQUISH_CONSTANT_3D;
					dx_ext1 = dx0 - 3f64 * SQUISH_CONSTANT_3D;
				}

				if (c & 0x02) != 0 {
					ysv_ext0 = ysb + 1;
					ysv_ext1 = ysb + 1;
					dy_ext0 = dy0 - 1f64 - 3f64 * SQUISH_CONSTANT_3D;
					dy_ext1 = dy0 - 1f64 - 3f64 * SQUISH_CONSTANT_3D;
					if (c & 0x01) != 0 {
						ysv_ext1 += 1;
						dy_ext1 -= 1f64;
					} else {
						ysv_ext0 += 1;
						dy_ext0 -= 1f64;
					}
				} else {
					ysv_ext0 = ysb;
					ysv_ext1 = ysb;
					dy_ext0 = dy0 - 3f64 * SQUISH_CONSTANT_3D;
					dy_ext1 = dy0 - 3f64 * SQUISH_CONSTANT_3D;
				}

				if (c & 0x04) != 0 {
					zsv_ext0 = zsb + 1;
					zsv_ext1 = zsb + 2;
					dz_ext0 = dz0 - 1f64 - 3f64 * SQUISH_CONSTANT_3D;
					dz_ext1 = dz0 - 2f64 - 3f64 * SQUISH_CONSTANT_3D;
				} else {
					zsv_ext0 = zsb;
					zsv_ext1 = zsb;
					dz_ext0 = dz0 - 3f64 * SQUISH_CONSTANT_3D;
					dz_ext1 = dz0 - 3f64 * SQUISH_CONSTANT_3D;
				}
			} else { // (1,1,1) is not one of the closest two tetrahedral vertices.
				let c = a_point & b_point; // Our two extra vertices are determined by the closest two.
				
				if (c & 0x01) != 0 {
					xsv_ext0 = xsb + 1;
					xsv_ext1 = xsb + 2;
					dx_ext0 = dx0 - 1f64 - SQUISH_CONSTANT_3D;
					dx_ext1 = dx0 - 2f64 - 2f64 * SQUISH_CONSTANT_3D;
				} else {
					xsv_ext0 = xsb;
					xsv_ext1 = xsb;
					dx_ext0 = dx0 - SQUISH_CONSTANT_3D;
					dx_ext1 = dx0 - 2f64 * SQUISH_CONSTANT_3D;
				}

				if (c & 0x02) != 0 {
					ysv_ext0 = ysb + 1;
					ysv_ext1 = ysb + 2;
					dy_ext0 = dy0 - 1f64 - SQUISH_CONSTANT_3D;
					dy_ext1 = dy0 - 2f64 - 2f64 * SQUISH_CONSTANT_3D;
				} else {
					ysv_ext0 = ysb;
					ysv_ext1 = ysb;
					dy_ext0 = dy0 - SQUISH_CONSTANT_3D;
					dy_ext1 = dy0 - 2f64 * SQUISH_CONSTANT_3D;
				}

				if (c & 0x04) != 0 {
					zsv_ext0 = zsb + 1;
					zsv_ext1 = zsb + 2;
					dz_ext0 = dz0 - 1f64 - SQUISH_CONSTANT_3D;
					dz_ext1 = dz0 - 2f64 - 2f64 * SQUISH_CONSTANT_3D;
				} else {
					zsv_ext0 = zsb;
					zsv_ext1 = zsb;
					dz_ext0 = dz0 - SQUISH_CONSTANT_3D;
					dz_ext1 = dz0 - 2f64 * SQUISH_CONSTANT_3D;
				}
			}
			
			// Contribution (1,1,0)
			let dx3 : f64 = dx0 - 1f64 - 2f64 * SQUISH_CONSTANT_3D;
			let dy3 : f64 = dy0 - 1f64 - 2f64 * SQUISH_CONSTANT_3D;
			let dz3 : f64 = dz0 - 0f64 - 2f64 * SQUISH_CONSTANT_3D;
			let mut attn3 : f64 = 2f64 - dx3 * dx3 - dy3 * dy3 - dz3 * dz3;
			if attn3 > 0f64 {
				attn3 *= attn3;
				value += attn3 * attn3 * self.extrapolate3(xsb + 1, ysb + 1, zsb + 0, dx3, dy3, dz3);
			}

			// Contribution (1,0,1)
			let dx2 : f64 = dx3;
			let dy2 : f64 = dy0 - 0f64 - 2f64 * SQUISH_CONSTANT_3D;
			let dz2 : f64 = dz0 - 1f64 - 2f64 * SQUISH_CONSTANT_3D;
			let mut attn2 : f64 = 2f64 - dx2 * dx2 - dy2 * dy2 - dz2 * dz2;
			if attn2 > 0f64 {
				attn2 *= attn2;
				value += attn2 * attn2 * self.extrapolate3(xsb + 1, ysb + 0, zsb + 1, dx2, dy2, dz2);
			}

			// Contribution (0,1,1)
			let dx1 : f64 = dx0 - 0f64 - 2f64 * SQUISH_CONSTANT_3D;
			let dy1 : f64 = dy3;
			let dz1 : f64 = dz2;
			let mut attn1 : f64 = 2f64 - dx1 * dx1 - dy1 * dy1 - dz1 * dz1;
			if attn1 > 0f64 {
				attn1 *= attn1;
				value += attn1 * attn1 * self.extrapolate3(xsb + 0, ysb + 1, zsb + 1, dx1, dy1, dz1);
			}

			// Contribution (1,1,1)
			dx0 = dx0 - 1f64 - 3f64 * SQUISH_CONSTANT_3D;
			dy0 = dy0 - 1f64 - 3f64 * SQUISH_CONSTANT_3D;
			dz0 = dz0 - 1f64 - 3f64 * SQUISH_CONSTANT_3D;
			let mut attn0 : f64 = 2f64 - dx0 * dx0 - dy0 * dy0 - dz0 * dz0;
			if attn0 > 0f64 {
				attn0 *= attn0;
				value += attn0 * attn0 * self.extrapolate3(xsb + 1, ysb + 1, zsb + 1, dx0, dy0, dz0);
			}
		} else { // We're inside the octahedron (Rectified 3-Simplex) in between.
			let a_score : f64;
			let mut a_point : i8;
			let mut a_is_further_side : bool;
			let b_score : f64;
			let mut b_point : i8;
			let mut b_is_further_side : bool;

			// Decide between point (0,0,1) and (1,1,0) as closest
			let p1 : f64 = xins + yins;
			if p1 > 1f64 {
				a_score = p1 - 1f64;
				a_point = 0x03;
				a_is_further_side = true;
			} else {
				a_score = 1f64 - p1;
				a_point = 0x04;
				a_is_further_side = false;
			}

			// Decide between point (0,1,0) and (1,0,1) as closest
			let p2 : f64 = xins + zins;
			if p2 > 1f64 {
				b_score = p2 - 1f64;
				b_point = 0x05;
				b_is_further_side = true;
			} else {
				b_score = 1f64 - p2;
				b_point = 0x02;
				b_is_further_side = false;
			}
			
			// The closest out of the two (1,0,0) and (0,1,1) will replace the furthest out of the two decided above, if closer.
			let p3 : f64 = yins + zins;
			if p3 > 1f64 {
				let score = p3 - 1f64;
				if (a_score <= b_score) && (a_score < score) {
					a_point = 0x06;
					a_is_further_side = true;
				} else if (a_score > b_score) && (b_score < score) {
					b_point = 0x06;
					b_is_further_side = true;
				}
			} else {
				let score = 1f64 - p3;
				if (a_score <= b_score) && (a_score < score) {
					a_point = 0x01;
					a_is_further_side = false;
				} else if (a_score > b_score) && (b_score < score) {
					b_point = 0x01;
					b_is_further_side = false;
				}
			}
			
			// Where each of the two closest points are determines how the extra two vertices are calculated.
			if a_is_further_side == b_is_further_side {
				if a_is_further_side { // Both closest points on (1,1,1) side

					// One of the two extra points is (1,1,1)
					dx_ext0 = dx0 - 1f64 - 3f64 * SQUISH_CONSTANT_3D;
					dy_ext0 = dy0 - 1f64 - 3f64 * SQUISH_CONSTANT_3D;
					dz_ext0 = dz0 - 1f64 - 3f64 * SQUISH_CONSTANT_3D;
					xsv_ext0 = xsb + 1;
					ysv_ext0 = ysb + 1;
					zsv_ext0 = zsb + 1;

					// Other extra point is based on the shared axis.
					let c : i8 = a_point & b_point;
					if (c & 0x01) != 0 {
						dx_ext1 = dx0 - 2f64 - 2f64 * SQUISH_CONSTANT_3D;
						dy_ext1 = dy0 - 2f64 * SQUISH_CONSTANT_3D;
						dz_ext1 = dz0 - 2f64 * SQUISH_CONSTANT_3D;
						xsv_ext1 = xsb + 2;
						ysv_ext1 = ysb;
						zsv_ext1 = zsb;
					} else if (c & 0x02) != 0 {
						dx_ext1 = dx0 - 2f64 * SQUISH_CONSTANT_3D;
						dy_ext1 = dy0 - 2f64 - 2f64 * SQUISH_CONSTANT_3D;
						dz_ext1 = dz0 - 2f64 * SQUISH_CONSTANT_3D;
						xsv_ext1 = xsb;
						ysv_ext1 = ysb + 2;
						zsv_ext1 = zsb;
					} else {
						dx_ext1 = dx0 - 2f64 * SQUISH_CONSTANT_3D;
						dy_ext1 = dy0 - 2f64 * SQUISH_CONSTANT_3D;
						dz_ext1 = dz0 - 2f64 - 2f64 * SQUISH_CONSTANT_3D;
						xsv_ext1 = xsb;
						ysv_ext1 = ysb;
						zsv_ext1 = zsb + 2;
					}
				} else {// Both closest points on (0,0,0) side

					// One of the two extra points is (0,0,0)
					dx_ext0 = dx0;
					dy_ext0 = dy0;
					dz_ext0 = dz0;
					xsv_ext0 = xsb;
					ysv_ext0 = ysb;
					zsv_ext0 = zsb;

					// Other extra point is based on the omitted axis.
					let c = a_point | b_point;
					if (c & 0x01) == 0 {
						dx_ext1 = dx0 + 1f64 - SQUISH_CONSTANT_3D;
						dy_ext1 = dy0 - 1f64 - SQUISH_CONSTANT_3D;
						dz_ext1 = dz0 - 1f64 - SQUISH_CONSTANT_3D;
						xsv_ext1 = xsb - 1;
						ysv_ext1 = ysb + 1;
						zsv_ext1 = zsb + 1;
					} else if (c & 0x02) == 0 {
						dx_ext1 = dx0 - 1f64 - SQUISH_CONSTANT_3D;
						dy_ext1 = dy0 + 1f64 - SQUISH_CONSTANT_3D;
						dz_ext1 = dz0 - 1f64 - SQUISH_CONSTANT_3D;
						xsv_ext1 = xsb + 1;
						ysv_ext1 = ysb - 1;
						zsv_ext1 = zsb + 1;
					} else {
						dx_ext1 = dx0 - 1f64 - SQUISH_CONSTANT_3D;
						dy_ext1 = dy0 - 1f64 - SQUISH_CONSTANT_3D;
						dz_ext1 = dz0 + 1f64 - SQUISH_CONSTANT_3D;
						xsv_ext1 = xsb + 1;
						ysv_ext1 = ysb + 1;
						zsv_ext1 = zsb - 1;
					}
				}
			} else { // One point on (0,0,0) side, one point on (1,1,1) side
				let c1 : i8; let c2 :i8;
				if a_is_further_side {
					c1 = a_point;
					c2 = b_point;
				} else {
					c1 = b_point;
					c2 = a_point;
				}

				// One contribution is a permutation of (1,1,-1)
				if (c1 & 0x01) == 0 {
					dx_ext0 = dx0 + 1f64 - SQUISH_CONSTANT_3D;
					dy_ext0 = dy0 - 1f64 - SQUISH_CONSTANT_3D;
					dz_ext0 = dz0 - 1f64 - SQUISH_CONSTANT_3D;
					xsv_ext0 = xsb - 1;
					ysv_ext0 = ysb + 1;
					zsv_ext0 = zsb + 1;
				} else if (c1 & 0x02) == 0 {
					dx_ext0 = dx0 - 1f64 - SQUISH_CONSTANT_3D;
					dy_ext0 = dy0 + 1f64 - SQUISH_CONSTANT_3D;
					dz_ext0 = dz0 - 1f64 - SQUISH_CONSTANT_3D;
					xsv_ext0 = xsb + 1;
					ysv_ext0 = ysb - 1;
					zsv_ext0 = zsb + 1;
				} else {
					dx_ext0 = dx0 - 1f64 - SQUISH_CONSTANT_3D;
					dy_ext0 = dy0 - 1f64 - SQUISH_CONSTANT_3D;
					dz_ext0 = dz0 + 1f64 - SQUISH_CONSTANT_3D;
					xsv_ext0 = xsb + 1;
					ysv_ext0 = ysb + 1;
					zsv_ext0 = zsb - 1;
				}

				// One contribution is a permutation of (0,0,2)
				dx_ext1 = dx0 - 2f64 * SQUISH_CONSTANT_3D;
				dy_ext1 = dy0 - 2f64 * SQUISH_CONSTANT_3D;
				dz_ext1 = dz0 - 2f64 * SQUISH_CONSTANT_3D;
				xsv_ext1 = xsb;
				ysv_ext1 = ysb;
				zsv_ext1 = zsb;
				if (c2 & 0x01) != 0 {
					dx_ext1 -= 2f64;
					xsv_ext1 += 2;
				} else if (c2 & 0x02) != 0 {
					dy_ext1 -= 2f64;
					ysv_ext1 += 2;
				} else {
					dz_ext1 -= 2f64;
					zsv_ext1 += 2;
				}
			}

			// Contribution (1,0,0)
			let dx1 : f64 = dx0 - 1f64 - SQUISH_CONSTANT_3D;
			let dy1 : f64 = dy0 - 0f64 - SQUISH_CONSTANT_3D;
			let dz1 : f64 = dz0 - 0f64 - SQUISH_CONSTANT_3D;
			let mut attn1 : f64 = 2f64 - dx1 * dx1 - dy1 * dy1 - dz1 * dz1;
			if attn1 > 0f64 {
				attn1 *= attn1;
				value += attn1 * attn1 * self.extrapolate3(xsb + 1, ysb + 0, zsb + 0, dx1, dy1, dz1);
			}

			// Contribution (0,1,0)
			let dx2 : f64 = dx0 - 0f64 - SQUISH_CONSTANT_3D;
			let dy2 : f64 = dy0 - 1f64 - SQUISH_CONSTANT_3D;
			let dz2 : f64 = dz1;
			let mut attn2 : f64 = 2f64 - dx2 * dx2 - dy2 * dy2 - dz2 * dz2;
			if attn2 > 0f64 {
				attn2 *= attn2;
				value += attn2 * attn2 * self.extrapolate3(xsb + 0, ysb + 1, zsb + 0, dx2, dy2, dz2);
			}

			// Contribution (0,0,1)
			let dx3 : f64 = dx2;
			let dy3 : f64 = dy1;
			let dz3 : f64 = dz0 - 1f64 - SQUISH_CONSTANT_3D;
			let mut attn3 : f64 = 2f64 - dx3 * dx3 - dy3 * dy3 - dz3 * dz3;
			if attn3 > 0f64 {
				attn3 *= attn3;
				value += attn3 * attn3 * self.extrapolate3(xsb + 0, ysb + 0, zsb + 1, dx3, dy3, dz3);
			}

			// Contribution (1,1,0)
			let dx4 : f64 = dx0 - 1f64 - 2f64 * SQUISH_CONSTANT_3D;
			let dy4 : f64 = dy0 - 1f64 - 2f64 * SQUISH_CONSTANT_3D;
			let dz4 : f64 = dz0 - 0f64 - 2f64 * SQUISH_CONSTANT_3D;
			let mut attn4 : f64 = 2f64 - dx4 * dx4 - dy4 * dy4 - dz4 * dz4;
			if attn4 > 0f64 {
				attn4 *= attn4;
				value += attn4 * attn4 * self.extrapolate3(xsb + 1, ysb + 1, zsb + 0, dx4, dy4, dz4);
			}

			// Contribution (1,0,1)
			let dx5 : f64 = dx4;
			let dy5 : f64 = dy0 - 0f64 - 2f64 * SQUISH_CONSTANT_3D;
			let dz5 : f64 = dz0 - 1f64 - 2f64 * SQUISH_CONSTANT_3D;
			let mut attn5 : f64 = 2f64 - dx5 * dx5 - dy5 * dy5 - dz5 * dz5;
			if attn5 > 0f64 {
				attn5 *= attn5;
				value += attn5 * attn5 * self.extrapolate3(xsb + 1, ysb + 0, zsb + 1, dx5, dy5, dz5);
			}

			// Contribution (0,1,1)
			let dx6 : f64 = dx0 - 0f64 - 2f64 * SQUISH_CONSTANT_3D;
			let dy6 : f64 = dy4;
			let dz6 : f64 = dz5;
			let mut attn6 : f64 = 2f64 - dx6 * dx6 - dy6 * dy6 - dz6 * dz6;
			if attn6 > 0f64 {
				attn6 *= attn6;
				value += attn6 * attn6 * self.extrapolate3(xsb + 0, ysb + 1, zsb + 1, dx6, dy6, dz6);
			}
		}
 
		// First extra vertex
		let mut attn_ext0 = 2f64 - dx_ext0 * dx_ext0 - dy_ext0 * dy_ext0 - dz_ext0 * dz_ext0;
		if attn_ext0 > 0f64 {
			attn_ext0 *= attn_ext0;
			value += attn_ext0 * attn_ext0 * self.extrapolate3(xsv_ext0, ysv_ext0, zsv_ext0, dx_ext0, dy_ext0, dz_ext0);
		}

		// Second extra vertex
		let mut attn_ext1 = 2f64 - dx_ext1 * dx_ext1 - dy_ext1 * dy_ext1 - dz_ext1 * dz_ext1;
		if attn_ext1 > 0f64 {
			attn_ext1 *= attn_ext1;
			value += attn_ext1 * attn_ext1 * self.extrapolate3(xsv_ext1, ysv_ext1, zsv_ext1, dx_ext1, dy_ext1, dz_ext1);
		}
		
		return value;
	}

	fn getperms(permtable : &[usize; PSIZE]) -> ([Vec2D; PSIZE], [Vec3D; PSIZE], [Vec4D; PSIZE]) {
		
		let mut grad2 = [
			Vec2D::new( 0.130526192220052,  0.99144486137381),
			Vec2D::new( 0.38268343236509,   0.923879532511287),
			Vec2D::new( 0.608761429008721,  0.793353340291235),
			Vec2D::new( 0.793353340291235,  0.608761429008721),
			Vec2D::new( 0.923879532511287,  0.38268343236509),
			Vec2D::new( 0.99144486137381,   0.130526192220051),
			Vec2D::new( 0.99144486137381,  -0.130526192220051),
			Vec2D::new( 0.923879532511287, -0.38268343236509),
			Vec2D::new( 0.793353340291235, -0.60876142900872),
			Vec2D::new( 0.608761429008721, -0.793353340291235),
			Vec2D::new( 0.38268343236509,  -0.923879532511287),
			Vec2D::new( 0.130526192220052, -0.99144486137381),
			Vec2D::new(-0.130526192220052, -0.99144486137381),
			Vec2D::new(-0.38268343236509,  -0.923879532511287),
			Vec2D::new(-0.608761429008721, -0.793353340291235),
			Vec2D::new(-0.793353340291235, -0.608761429008721),
			Vec2D::new(-0.923879532511287, -0.38268343236509),
			Vec2D::new(-0.99144486137381,  -0.130526192220052),
			Vec2D::new(-0.99144486137381,   0.130526192220051),
			Vec2D::new(-0.923879532511287,  0.38268343236509),
			Vec2D::new(-0.793353340291235,  0.608761429008721),
			Vec2D::new(-0.608761429008721,  0.793353340291235),
			Vec2D::new(-0.38268343236509,   0.923879532511287),
			Vec2D::new(-0.130526192220052,  0.99144486137381)
		];
		for i in 0..grad2.len() {
			grad2[i] *= IN2;
		}
		let mut perm2d : [Vec2D; PSIZE] = [Vec2D::null(); PSIZE];
		for i in 0..PSIZE {
			perm2d[i] = grad2[permtable[i] % grad2.len()];
		}

		let mut grad3 = [
			Vec3D::new(-1.4082482904633333,    -1.4082482904633333,    -2.6329931618533333),
			Vec3D::new(-0.07491495712999985,   -0.07491495712999985,   -3.29965982852),
			Vec3D::new( 0.24732126143473554,   -1.6667938651159684,    -2.838945207362466),
			Vec3D::new(-1.6667938651159684,     0.24732126143473554,   -2.838945207362466),
			Vec3D::new(-1.4082482904633333,    -2.6329931618533333,    -1.4082482904633333),
			Vec3D::new(-0.07491495712999985,   -3.29965982852,         -0.07491495712999985),
			Vec3D::new(-1.6667938651159684,    -2.838945207362466,      0.24732126143473554),
			Vec3D::new( 0.24732126143473554,   -2.838945207362466,     -1.6667938651159684),
			Vec3D::new( 1.5580782047233335,     0.33333333333333337,   -2.8914115380566665),
			Vec3D::new( 2.8914115380566665,    -0.33333333333333337,   -1.5580782047233335),
			Vec3D::new( 1.8101897177633992,    -1.2760767510338025,    -2.4482280932803),
			Vec3D::new( 2.4482280932803,        1.2760767510338025,    -1.8101897177633992),
			Vec3D::new( 1.5580782047233335,    -2.8914115380566665,     0.33333333333333337),
			Vec3D::new( 2.8914115380566665,    -1.5580782047233335,    -0.33333333333333337),
			Vec3D::new( 2.4482280932803,       -1.8101897177633992,     1.2760767510338025),
			Vec3D::new( 1.8101897177633992,    -2.4482280932803,       -1.2760767510338025),
			Vec3D::new(-2.6329931618533333,    -1.4082482904633333,    -1.4082482904633333),
			Vec3D::new(-3.29965982852,         -0.07491495712999985,   -0.07491495712999985),
			Vec3D::new(-2.838945207362466,      0.24732126143473554,   -1.6667938651159684),
			Vec3D::new(-2.838945207362466,     -1.6667938651159684,     0.24732126143473554),
			Vec3D::new( 0.33333333333333337,    1.5580782047233335,    -2.8914115380566665),
			Vec3D::new(-0.33333333333333337,    2.8914115380566665,    -1.5580782047233335),
			Vec3D::new( 1.2760767510338025,     2.4482280932803,       -1.8101897177633992),
			Vec3D::new(-1.2760767510338025,     1.8101897177633992,    -2.4482280932803),
			Vec3D::new( 0.33333333333333337,   -2.8914115380566665,     1.5580782047233335),
			Vec3D::new(-0.33333333333333337,   -1.5580782047233335,     2.8914115380566665),
			Vec3D::new(-1.2760767510338025,    -2.4482280932803,        1.8101897177633992),
			Vec3D::new( 1.2760767510338025,    -1.8101897177633992,     2.4482280932803),
			Vec3D::new( 3.29965982852,          0.07491495712999985,    0.07491495712999985),
			Vec3D::new( 2.6329931618533333,     1.4082482904633333,     1.4082482904633333),
			Vec3D::new( 2.838945207362466,     -0.24732126143473554,    1.6667938651159684),
			Vec3D::new( 2.838945207362466,      1.6667938651159684,    -0.24732126143473554),
			Vec3D::new(-2.8914115380566665,     1.5580782047233335,     0.33333333333333337),
			Vec3D::new(-1.5580782047233335,     2.8914115380566665,    -0.33333333333333337),
			Vec3D::new(-2.4482280932803,        1.8101897177633992,    -1.2760767510338025),
			Vec3D::new(-1.8101897177633992,     2.4482280932803,        1.2760767510338025),
			Vec3D::new(-2.8914115380566665,     0.33333333333333337,    1.5580782047233335),
			Vec3D::new(-1.5580782047233335,    -0.33333333333333337,    2.8914115380566665),
			Vec3D::new(-1.8101897177633992,     1.2760767510338025,     2.4482280932803),
			Vec3D::new(-2.4482280932803,       -1.2760767510338025,     1.8101897177633992),
			Vec3D::new( 0.07491495712999985,    3.29965982852,          0.07491495712999985),
			Vec3D::new( 1.4082482904633333,     2.6329931618533333,     1.4082482904633333),
			Vec3D::new( 1.6667938651159684,     2.838945207362466,     -0.24732126143473554),
			Vec3D::new(-0.24732126143473554,    2.838945207362466,      1.6667938651159684),
			Vec3D::new( 0.07491495712999985,    0.07491495712999985,    3.29965982852),
			Vec3D::new( 1.4082482904633333,     1.4082482904633333,     2.6329931618533333),
			Vec3D::new(-0.24732126143473554,    1.6667938651159684,     2.838945207362466),
			Vec3D::new( 1.6667938651159684,    -0.24732126143473554,    2.838945207362466)
		];
		for i in 0..grad3.len() {
			grad3[i] *= IN3;
		}
		let mut perm3d : [Vec3D; PSIZE] = [Vec3D::null(); PSIZE];
		for i in 0..PSIZE {
			perm3d[i] = grad3[permtable[i] % grad3.len()];
		}

		let mut grad4 = [
			Vec4D::new(-0.753341017856078,    -0.37968289875261624,  -0.37968289875261624,  -0.37968289875261624),
			Vec4D::new(-0.7821684431180708,   -0.4321472685365301,   -0.4321472685365301,    0.12128480194602098),
			Vec4D::new(-0.7821684431180708,   -0.4321472685365301,    0.12128480194602098,  -0.4321472685365301),
			Vec4D::new(-0.7821684431180708,    0.12128480194602098,  -0.4321472685365301,   -0.4321472685365301),
			Vec4D::new(-0.8586508742123365,   -0.508629699630796,     0.044802370851755174,  0.044802370851755174),
			Vec4D::new(-0.8586508742123365,    0.044802370851755174, -0.508629699630796,     0.044802370851755174),
			Vec4D::new(-0.8586508742123365,    0.044802370851755174,  0.044802370851755174, -0.508629699630796),
			Vec4D::new(-0.9982828964265062,   -0.03381941603233842,  -0.03381941603233842,  -0.03381941603233842),
			Vec4D::new(-0.37968289875261624,  -0.753341017856078,    -0.37968289875261624,  -0.37968289875261624),
			Vec4D::new(-0.4321472685365301,   -0.7821684431180708,   -0.4321472685365301,    0.12128480194602098),
			Vec4D::new(-0.4321472685365301,   -0.7821684431180708,    0.12128480194602098,  -0.4321472685365301),
			Vec4D::new( 0.12128480194602098,  -0.7821684431180708,   -0.4321472685365301,   -0.4321472685365301),
			Vec4D::new(-0.508629699630796,    -0.8586508742123365,    0.044802370851755174,  0.044802370851755174),
			Vec4D::new( 0.044802370851755174, -0.8586508742123365,   -0.508629699630796,     0.044802370851755174),
			Vec4D::new( 0.044802370851755174, -0.8586508742123365,    0.044802370851755174, -0.508629699630796),
			Vec4D::new(-0.03381941603233842,  -0.9982828964265062,   -0.03381941603233842,  -0.03381941603233842),
			Vec4D::new(-0.37968289875261624,  -0.37968289875261624,  -0.753341017856078,    -0.37968289875261624),
			Vec4D::new(-0.4321472685365301,   -0.4321472685365301,   -0.7821684431180708,    0.12128480194602098),
			Vec4D::new(-0.4321472685365301,    0.12128480194602098,  -0.7821684431180708,   -0.4321472685365301),
			Vec4D::new( 0.12128480194602098,  -0.4321472685365301,   -0.7821684431180708,   -0.4321472685365301),
			Vec4D::new(-0.508629699630796,     0.044802370851755174, -0.8586508742123365,    0.044802370851755174),
			Vec4D::new( 0.044802370851755174, -0.508629699630796,    -0.8586508742123365,    0.044802370851755174),
			Vec4D::new( 0.044802370851755174,  0.044802370851755174, -0.8586508742123365,   -0.508629699630796),
			Vec4D::new(-0.03381941603233842,  -0.03381941603233842,  -0.9982828964265062,   -0.03381941603233842),
			Vec4D::new(-0.37968289875261624,  -0.37968289875261624,  -0.37968289875261624,  -0.753341017856078),
			Vec4D::new(-0.4321472685365301,   -0.4321472685365301,    0.12128480194602098,  -0.7821684431180708),
			Vec4D::new(-0.4321472685365301,    0.12128480194602098,  -0.4321472685365301,   -0.7821684431180708),
			Vec4D::new( 0.12128480194602098,  -0.4321472685365301,   -0.4321472685365301,   -0.7821684431180708),
			Vec4D::new(-0.508629699630796,     0.044802370851755174,  0.044802370851755174, -0.8586508742123365),
			Vec4D::new( 0.044802370851755174, -0.508629699630796,     0.044802370851755174, -0.8586508742123365),
			Vec4D::new( 0.044802370851755174,  0.044802370851755174, -0.508629699630796,    -0.8586508742123365),
			Vec4D::new(-0.03381941603233842,  -0.03381941603233842,  -0.03381941603233842,  -0.9982828964265062),
			Vec4D::new(-0.6740059517812944,   -0.3239847771997537,   -0.3239847771997537,    0.5794684678643381),
			Vec4D::new(-0.7504883828755602,   -0.4004672082940195,    0.15296486218853164,   0.5029860367700724),
			Vec4D::new(-0.7504883828755602,    0.15296486218853164,  -0.4004672082940195,    0.5029860367700724),
			Vec4D::new(-0.8828161875373585,    0.08164729285680945,   0.08164729285680945,   0.4553054119602712),
			Vec4D::new(-0.4553054119602712,   -0.08164729285680945,  -0.08164729285680945,   0.8828161875373585),
			Vec4D::new(-0.5029860367700724,   -0.15296486218853164,   0.4004672082940195,    0.7504883828755602),
			Vec4D::new(-0.5029860367700724,    0.4004672082940195,   -0.15296486218853164,   0.7504883828755602),
			Vec4D::new(-0.5794684678643381,    0.3239847771997537,    0.3239847771997537,    0.6740059517812944),
			Vec4D::new(-0.3239847771997537,   -0.6740059517812944,   -0.3239847771997537,    0.5794684678643381),
			Vec4D::new(-0.4004672082940195,   -0.7504883828755602,    0.15296486218853164,   0.5029860367700724),
			Vec4D::new( 0.15296486218853164,  -0.7504883828755602,   -0.4004672082940195,    0.5029860367700724),
			Vec4D::new( 0.08164729285680945,  -0.8828161875373585,    0.08164729285680945,   0.4553054119602712),
			Vec4D::new(-0.08164729285680945,  -0.4553054119602712,   -0.08164729285680945,   0.8828161875373585),
			Vec4D::new(-0.15296486218853164,  -0.5029860367700724,    0.4004672082940195,    0.7504883828755602),
			Vec4D::new( 0.4004672082940195,   -0.5029860367700724,   -0.15296486218853164,   0.7504883828755602),
			Vec4D::new( 0.3239847771997537,   -0.5794684678643381,    0.3239847771997537,    0.6740059517812944),
			Vec4D::new(-0.3239847771997537,   -0.3239847771997537,   -0.6740059517812944,    0.5794684678643381),
			Vec4D::new(-0.4004672082940195,    0.15296486218853164,  -0.7504883828755602,    0.5029860367700724),
			Vec4D::new( 0.15296486218853164,  -0.4004672082940195,   -0.7504883828755602,    0.5029860367700724),
			Vec4D::new( 0.08164729285680945,   0.08164729285680945,  -0.8828161875373585,    0.4553054119602712),
			Vec4D::new(-0.08164729285680945,  -0.08164729285680945,  -0.4553054119602712,    0.8828161875373585),
			Vec4D::new(-0.15296486218853164,   0.4004672082940195,   -0.5029860367700724,    0.7504883828755602),
			Vec4D::new( 0.4004672082940195,   -0.15296486218853164,  -0.5029860367700724,    0.7504883828755602),
			Vec4D::new( 0.3239847771997537,    0.3239847771997537,   -0.5794684678643381,    0.6740059517812944),
			Vec4D::new(-0.6740059517812944,   -0.3239847771997537,    0.5794684678643381,   -0.3239847771997537),
			Vec4D::new(-0.7504883828755602,   -0.4004672082940195,    0.5029860367700724,    0.15296486218853164),
			Vec4D::new(-0.7504883828755602,    0.15296486218853164,   0.5029860367700724,   -0.4004672082940195),
			Vec4D::new(-0.8828161875373585,    0.08164729285680945,   0.4553054119602712,    0.08164729285680945),
			Vec4D::new(-0.4553054119602712,   -0.08164729285680945,   0.8828161875373585,   -0.08164729285680945),
			Vec4D::new(-0.5029860367700724,   -0.15296486218853164,   0.7504883828755602,    0.4004672082940195),
			Vec4D::new(-0.5029860367700724,    0.4004672082940195,    0.7504883828755602,   -0.15296486218853164),
			Vec4D::new(-0.5794684678643381,    0.3239847771997537,    0.6740059517812944,    0.3239847771997537),
			Vec4D::new(-0.3239847771997537,   -0.6740059517812944,    0.5794684678643381,   -0.3239847771997537),
			Vec4D::new(-0.4004672082940195,   -0.7504883828755602,    0.5029860367700724,    0.15296486218853164),
			Vec4D::new( 0.15296486218853164,  -0.7504883828755602,    0.5029860367700724,   -0.4004672082940195),
			Vec4D::new( 0.08164729285680945,  -0.8828161875373585,    0.4553054119602712,    0.08164729285680945),
			Vec4D::new(-0.08164729285680945,  -0.4553054119602712,    0.8828161875373585,   -0.08164729285680945),
			Vec4D::new(-0.15296486218853164,  -0.5029860367700724,    0.7504883828755602,    0.4004672082940195),
			Vec4D::new( 0.4004672082940195,   -0.5029860367700724,    0.7504883828755602,   -0.15296486218853164),
			Vec4D::new( 0.3239847771997537,   -0.5794684678643381,    0.6740059517812944,    0.3239847771997537),
			Vec4D::new(-0.3239847771997537,   -0.3239847771997537,    0.5794684678643381,   -0.6740059517812944),
			Vec4D::new(-0.4004672082940195,    0.15296486218853164,   0.5029860367700724,   -0.7504883828755602),
			Vec4D::new( 0.15296486218853164,  -0.4004672082940195,    0.5029860367700724,   -0.7504883828755602),
			Vec4D::new( 0.08164729285680945,   0.08164729285680945,   0.4553054119602712,   -0.8828161875373585),
			Vec4D::new(-0.08164729285680945,  -0.08164729285680945,   0.8828161875373585,   -0.4553054119602712),
			Vec4D::new(-0.15296486218853164,   0.4004672082940195,    0.7504883828755602,   -0.5029860367700724),
			Vec4D::new( 0.4004672082940195,   -0.15296486218853164,   0.7504883828755602,   -0.5029860367700724),
			Vec4D::new( 0.3239847771997537,    0.3239847771997537,    0.6740059517812944,   -0.5794684678643381),
			Vec4D::new(-0.6740059517812944,    0.5794684678643381,   -0.3239847771997537,   -0.3239847771997537),
			Vec4D::new(-0.7504883828755602,    0.5029860367700724,   -0.4004672082940195,    0.15296486218853164),
			Vec4D::new(-0.7504883828755602,    0.5029860367700724,    0.15296486218853164,  -0.4004672082940195),
			Vec4D::new(-0.8828161875373585,    0.4553054119602712,    0.08164729285680945,   0.08164729285680945),
			Vec4D::new(-0.4553054119602712,    0.8828161875373585,   -0.08164729285680945,  -0.08164729285680945),
			Vec4D::new(-0.5029860367700724,    0.7504883828755602,   -0.15296486218853164,   0.4004672082940195),
			Vec4D::new(-0.5029860367700724,    0.7504883828755602,    0.4004672082940195,   -0.15296486218853164),
			Vec4D::new(-0.5794684678643381,    0.6740059517812944,    0.3239847771997537,    0.3239847771997537),
			Vec4D::new(-0.3239847771997537,    0.5794684678643381,   -0.6740059517812944,   -0.3239847771997537),
			Vec4D::new(-0.4004672082940195,    0.5029860367700724,   -0.7504883828755602,    0.15296486218853164),
			Vec4D::new( 0.15296486218853164,   0.5029860367700724,   -0.7504883828755602,   -0.4004672082940195),
			Vec4D::new( 0.08164729285680945,   0.4553054119602712,   -0.8828161875373585,    0.08164729285680945),
			Vec4D::new(-0.08164729285680945,   0.8828161875373585,   -0.4553054119602712,   -0.08164729285680945),
			Vec4D::new(-0.15296486218853164,   0.7504883828755602,   -0.5029860367700724,    0.4004672082940195),
			Vec4D::new( 0.4004672082940195,    0.7504883828755602,   -0.5029860367700724,   -0.15296486218853164),
			Vec4D::new( 0.3239847771997537,    0.6740059517812944,   -0.5794684678643381,    0.3239847771997537),
			Vec4D::new(-0.3239847771997537,    0.5794684678643381,   -0.3239847771997537,   -0.6740059517812944),
			Vec4D::new(-0.4004672082940195,    0.5029860367700724,    0.15296486218853164,  -0.7504883828755602),
			Vec4D::new( 0.15296486218853164,   0.5029860367700724,   -0.4004672082940195,   -0.7504883828755602),
			Vec4D::new( 0.08164729285680945,   0.4553054119602712,    0.08164729285680945,  -0.8828161875373585),
			Vec4D::new(-0.08164729285680945,   0.8828161875373585,   -0.08164729285680945,  -0.4553054119602712),
			Vec4D::new(-0.15296486218853164,   0.7504883828755602,    0.4004672082940195,   -0.5029860367700724),
			Vec4D::new( 0.4004672082940195,    0.7504883828755602,   -0.15296486218853164,  -0.5029860367700724),
			Vec4D::new( 0.3239847771997537,    0.6740059517812944,    0.3239847771997537,   -0.5794684678643381),
			Vec4D::new( 0.5794684678643381,   -0.6740059517812944,   -0.3239847771997537,   -0.3239847771997537),
			Vec4D::new( 0.5029860367700724,   -0.7504883828755602,   -0.4004672082940195,    0.15296486218853164),
			Vec4D::new( 0.5029860367700724,   -0.7504883828755602,    0.15296486218853164,  -0.4004672082940195),
			Vec4D::new( 0.4553054119602712,   -0.8828161875373585,    0.08164729285680945,   0.08164729285680945),
			Vec4D::new( 0.8828161875373585,   -0.4553054119602712,   -0.08164729285680945,  -0.08164729285680945),
			Vec4D::new( 0.7504883828755602,   -0.5029860367700724,   -0.15296486218853164,   0.4004672082940195),
			Vec4D::new( 0.7504883828755602,   -0.5029860367700724,    0.4004672082940195,   -0.15296486218853164),
			Vec4D::new( 0.6740059517812944,   -0.5794684678643381,    0.3239847771997537,    0.3239847771997537),
			Vec4D::new( 0.5794684678643381,   -0.3239847771997537,   -0.6740059517812944,   -0.3239847771997537),
			Vec4D::new( 0.5029860367700724,   -0.4004672082940195,   -0.7504883828755602,    0.15296486218853164),
			Vec4D::new( 0.5029860367700724,    0.15296486218853164,  -0.7504883828755602,   -0.4004672082940195),
			Vec4D::new( 0.4553054119602712,    0.08164729285680945,  -0.8828161875373585,    0.08164729285680945),
			Vec4D::new( 0.8828161875373585,   -0.08164729285680945,  -0.4553054119602712,   -0.08164729285680945),
			Vec4D::new( 0.7504883828755602,   -0.15296486218853164,  -0.5029860367700724,    0.4004672082940195),
			Vec4D::new( 0.7504883828755602,    0.4004672082940195,   -0.5029860367700724,   -0.15296486218853164),
			Vec4D::new( 0.6740059517812944,    0.3239847771997537,   -0.5794684678643381,    0.3239847771997537),
			Vec4D::new( 0.5794684678643381,   -0.3239847771997537,   -0.3239847771997537,   -0.6740059517812944),
			Vec4D::new( 0.5029860367700724,   -0.4004672082940195,    0.15296486218853164,  -0.7504883828755602),
			Vec4D::new( 0.5029860367700724,    0.15296486218853164,  -0.4004672082940195,   -0.7504883828755602),
			Vec4D::new( 0.4553054119602712,    0.08164729285680945,   0.08164729285680945,  -0.8828161875373585),
			Vec4D::new( 0.8828161875373585,   -0.08164729285680945,  -0.08164729285680945,  -0.4553054119602712),
			Vec4D::new( 0.7504883828755602,   -0.15296486218853164,   0.4004672082940195,   -0.5029860367700724),
			Vec4D::new( 0.7504883828755602,    0.4004672082940195,   -0.15296486218853164,  -0.5029860367700724),
			Vec4D::new( 0.6740059517812944,    0.3239847771997537,    0.3239847771997537,   -0.5794684678643381),
			Vec4D::new( 0.03381941603233842,   0.03381941603233842,   0.03381941603233842,   0.9982828964265062),
			Vec4D::new(-0.044802370851755174, -0.044802370851755174,  0.508629699630796,     0.8586508742123365),
			Vec4D::new(-0.044802370851755174,  0.508629699630796,    -0.044802370851755174,  0.8586508742123365),
			Vec4D::new(-0.12128480194602098,   0.4321472685365301,    0.4321472685365301,    0.7821684431180708),
			Vec4D::new( 0.508629699630796,    -0.044802370851755174, -0.044802370851755174,  0.8586508742123365),
			Vec4D::new( 0.4321472685365301,   -0.12128480194602098,   0.4321472685365301,    0.7821684431180708),
			Vec4D::new( 0.4321472685365301,    0.4321472685365301,   -0.12128480194602098,   0.7821684431180708),
			Vec4D::new( 0.37968289875261624,   0.37968289875261624,   0.37968289875261624,   0.753341017856078),
			Vec4D::new( 0.03381941603233842,   0.03381941603233842,   0.9982828964265062,    0.03381941603233842),
			Vec4D::new(-0.044802370851755174,  0.044802370851755174,  0.8586508742123365,    0.508629699630796),
			Vec4D::new(-0.044802370851755174,  0.508629699630796,     0.8586508742123365,   -0.044802370851755174),
			Vec4D::new(-0.12128480194602098,   0.4321472685365301,    0.7821684431180708,    0.4321472685365301),
			Vec4D::new( 0.508629699630796,    -0.044802370851755174,  0.8586508742123365,   -0.044802370851755174),
			Vec4D::new( 0.4321472685365301,   -0.12128480194602098,   0.7821684431180708,    0.4321472685365301),
			Vec4D::new( 0.4321472685365301,    0.4321472685365301,    0.7821684431180708,   -0.12128480194602098),
			Vec4D::new( 0.37968289875261624,   0.37968289875261624,   0.753341017856078,     0.37968289875261624),
			Vec4D::new( 0.03381941603233842,   0.9982828964265062,    0.03381941603233842,   0.03381941603233842),
			Vec4D::new(-0.044802370851755174,  0.8586508742123365,   -0.044802370851755174,  0.508629699630796),
			Vec4D::new(-0.044802370851755174,  0.8586508742123365,    0.508629699630796,    -0.044802370851755174),
			Vec4D::new(-0.12128480194602098,   0.7821684431180708,    0.4321472685365301,    0.4321472685365301),
			Vec4D::new( 0.508629699630796,     0.8586508742123365,   -0.044802370851755174, -0.044802370851755174),
			Vec4D::new( 0.4321472685365301,    0.7821684431180708,   -0.12128480194602098,   0.4321472685365301),
			Vec4D::new( 0.4321472685365301,    0.7821684431180708,    0.4321472685365301,   -0.12128480194602098),
			Vec4D::new( 0.37968289875261624,   0.753341017856078,     0.37968289875261624,   0.37968289875261624),
			Vec4D::new( 0.9982828964265062,    0.03381941603233842,   0.03381941603233842,   0.03381941603233842),
			Vec4D::new( 0.8586508742123365,   -0.044802370851755174, -0.044802370851755174,  0.508629699630796),
			Vec4D::new( 0.8586508742123365,   -0.044802370851755174,  0.508629699630796,    -0.044802370851755174),
			Vec4D::new( 0.7821684431180708,   -0.12128480194602098,   0.4321472685365301,    0.4321472685365301),
			Vec4D::new( 0.8586508742123365,    0.508629699630796,    -0.044802370851755174, -0.044802370851755174),
			Vec4D::new( 0.7821684431180708,    0.4321472685365301,   -0.12128480194602098,   0.4321472685365301),
			Vec4D::new( 0.7821684431180708,    0.4321472685365301,    0.4321472685365301,   -0.12128480194602098),
			Vec4D::new( 0.753341017856078,     0.37968289875261624,   0.37968289875261624,   0.37968289875261624)
		];
		for i in 0..grad4.len() {
			grad4[i] *= IN4;
		}
		let mut perm4d : [Vec4D; PSIZE] = [Vec4D::null(); PSIZE];
		for i in 0..PSIZE {
			perm4d[i] = grad4[permtable[i] % grad3.len()];
		}
		return (perm2d, perm3d, perm4d)
	}
	
	fn extrapolate2(&self, xsb:i32, ysb:i32, dx:f64, dy:f64) -> f64 {
		let grad : Vec2D = self.perm2d[self.perm[xsb as usize & PMASK] ^ (ysb as usize & PMASK)];
		return grad.x * dx + grad.y * dy;
	}

	fn extrapolate3(&self, xsb:i32, ysb:i32, zsb:i32, dx:f64, dy:f64, dz:f64) -> f64 {
		let grad : Vec3D = self.perm3d[self.perm[self.perm[xsb as usize & PMASK] ^ (ysb as usize & PMASK)] ^ (zsb as usize & PMASK) ];
		return grad.x * dx + grad.y * dy + grad.z * dz;
	}

#[allow(dead_code)]
	fn extrapolate4(&self, xsb:i32, ysb:i32, zsb:i32, wsb:i32, dx:f64, dy:f64, dz:f64, dw:f64) -> f64 {
			let grad : Vec4D = self.perm4d[self.perm[self.perm[self.perm[xsb as usize & PMASK] ^ (ysb as usize & PMASK)] ^ (zsb as usize & PMASK)] ^ (wsb as usize & PMASK)];
			return grad.x * dx + grad.y * dy + grad.z * dz + grad.w * dw;
		}
}