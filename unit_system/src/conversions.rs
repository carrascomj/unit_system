use crate::traits::*;
use crate::units::*;

impl BaseIntoBase<Molecules> for Mol {
    fn convert_base(self) -> Molecules {
        Molecules((self.0 * 6.022E23) as f64)
    }
}
