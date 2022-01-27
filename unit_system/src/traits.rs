pub trait UnitConvert<To> {
    fn convert(self) -> To;
}

/// `Unit`s define methods to convert themselves to the base unit. For instance, the base unit of kilometer is meter.
///
/// This trait is can be derived for a base unit and it will generate the corresponding
/// structs in the SI.
///
/// ```
/// use unit_system::prelude::*;
///
/// #[derive(Unit, Debug, PartialEq)]
/// pub struct Liter(pub f32);
///
/// // now CLiter, KLiter, NLiter, MuLiter, etc. are available
/// let cliters = CLiter(3.);
/// let liters = cliters.to_base();
///
/// assert_eq!(liters, Liter(0.03));
/// ```
pub trait Unit {
    type BaseUnit;

    fn to_base(self) -> Self::BaseUnit;
    fn from_base(base: Self::BaseUnit) -> Self;
}

pub trait UnitInto<To> {
    fn convert(self) -> To;
}

/// Automatically implemented for all [`Unit`] of the same `BaseUnit`.
impl<T, Mid, To> UnitInto<To> for T
where
    T: Unit<BaseUnit = Mid>,
    To: Unit<BaseUnit = Mid>,
{
    /// Convert between [`Unit`]s of the same `BaseUnit` (like kilograms and nanograms).
    fn convert(self) -> To {
        To::from_base(self.to_base())
    }
}

/// Automatically implemented for all [`Unit`]s that are `BaseUnit`s.
pub trait BaseInto<To> {
    /// Convert a base to an unit of its unit system (e.g., gram to nanogram).
    fn to_non_base(self) -> To;
}

impl<T, To> BaseInto<To> for T
where
    Self: Unit<BaseUnit = Self>,
    To: Unit<BaseUnit = Self>,
{
    fn to_non_base(self) -> To {
        To::from_base(self)
    }
}

/// Convert between two `BaseUnit`, connecting two unit systems.
///
/// This is the only trait that is expected to be implemented by the user. Via
/// [`NonBaseIntoOuter`], allows to convert from any non-base to another non-base
/// if their `BaseUnit`s implement this trait.
pub trait BaseIntoBase<To>
where
    To: Unit<BaseUnit = To>,
{
    fn convert_base(self) -> To;
}

/// Convert between two unit systems.
pub trait NonBaseIntoOuter<To> {
    /// Convert from a non-base in a unit system to a non-base of another.
    fn convert_outer(self) -> To;
}

impl<T, Base, ToBase, To> NonBaseIntoOuter<To> for T
where
    T: Unit<BaseUnit = Base>,
    To: Unit<BaseUnit = ToBase>,
    Base: Unit + BaseIntoBase<ToBase>,
    ToBase: Unit<BaseUnit = ToBase>,
{
    fn convert_outer(self) -> To {
        To::from_base(self.to_base().convert_base())
    }
}
