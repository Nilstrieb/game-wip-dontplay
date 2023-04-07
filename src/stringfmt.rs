use std::fmt;

use crate::math::M_PER_PX;

pub struct LengthDisp(pub f32);

impl fmt::Display for LengthDisp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let meters = self.0 * M_PER_PX;
        if meters.abs() > 1000. {
            let km = if meters.is_sign_negative() {
                (meters / 1000.).ceil()
            } else {
                (meters / 1000.).floor()
            };
            let m = meters % 1000.;
            write!(f, "{km} km, {m} m")
        } else {
            write!(f, "{meters} m")
        }
    }
}
