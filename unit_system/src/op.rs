use crate::traits::Unit;

pub struct UnitMul<T, U>(T, U);
pub struct UnitDiv<T, U>(T, U);

impl<T, U, TBase, UBase> Unit for UnitMul<T, U>
where
    T: Unit<BaseUnit = TBase>,
    U: Unit<BaseUnit = UBase>,
{
    type BaseUnit = (TBase, UBase);

    fn to_base(self) -> Self::BaseUnit {
        (self.0.to_base(), self.1.to_base())
    }

    fn from_base(base: Self::BaseUnit) -> Self {
        UnitMul(T::from_base(base.0), U::from_base(base.1))
    }
}

impl<T, U, TBase, UBase> Unit for UnitDiv<T, U>
where
    T: Unit<BaseUnit = TBase>,
    U: Unit<BaseUnit = UBase>,
{
    type BaseUnit = (TBase, UBase);

    fn to_base(self) -> Self::BaseUnit {
        (self.0.to_base(), self.1.to_base())
    }

    fn from_base(base: Self::BaseUnit) -> Self {
        UnitDiv(T::from_base(base.0), U::from_base(base.1))
    }
}
