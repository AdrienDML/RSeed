use crate::fct::fast_floor;
use crate::Vec2D;


const STRETCH_CONSTANT_2D : f64 = -0.211324865405187;
const SQUISH_CONSTANT_2D  : f64 = 0.366025403784439;  

pub struct SimplexVoronoi {
    seed : i64,
}

impl SimplexVoronoi {

    fn eval2(point : Vec2D) {
        let strech_offset : f64 = (point.x + point.y) * STRETCH_CONSTANT_2D;
		let point_streched : Vec2D = point + strech_offset;

		let case_x : f64 = fast_floor(point_streched.x);
		let case_y : f64 = fast_floor(point_streched.y);
        
        let point_ins : f64 = point_streched - Vec::new(case_x as f64, case_y as f64);
                
    }
}