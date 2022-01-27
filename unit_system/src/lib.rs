//! Automatically derived typed Unit system with conversions.
//!
//! # Example
//!
//! ```rust
//! use unit_system::prelude::*;
//!
//! #[derive(Unit)]
//! pub struct Meter(pub f32);
//! #[derive(Unit)]
//! pub struct Inch(pub f64);
//!
//! // we can convert between any `Unit`s whose bases define BaseIntoBase
//! impl BaseIntoBase<Inch> for Meter {
//!   fn convert_base(self) -> Inch {
//!      Inch(self.0 as f64 * 40.)
//!   }
//! }
//!
//! // CMeter, DMeter, HMeter, NMeter... have been automatically built
//! let centimeters: CMeter = CMeter(3.);
//! // the same for inches (non-SI units would require manual impls of Unit)
//! let kilo_inches: KInch = centimeters.convert_outer();
//!
//! // KInch{ 0.012. }
//! println!("{:?}", kilo_inches);
//! assert!(f64::abs(kilo_inches.0 - 0.0012) < 1e-7);
//! ```
pub mod conversions;
mod op;
mod traits;
pub mod units;

pub mod prelude {
    pub use crate::op::*;
    pub use crate::traits::*;
    pub use unit_derive::Unit;
}

#[cfg(test)]
mod tests {
    use super::prelude::*;
    use super::*;

    #[test]
    fn test_mmol_into_molecules() {
        let mmol = units::MMol(32.0);
        let molecules: units::MMolecules = mmol.convert_outer();
        assert!(f64::abs(molecules.0 - units::MMolecules(32.0 * 6.022E23).0) < 1E18)
    }

    #[test]
    fn test_nmol_into_base() {
        let nmol = units::MMol(3.0);
        assert!(f32::abs(nmol.to_base().0 - units::Mol(3. / 1E3).0) < 1E-7)
    }
}
