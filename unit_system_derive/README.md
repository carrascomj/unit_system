# unit_system_derive

Proc_macro derivation of `Unit` for the
[`unit_system`](https://github.com/carrascomj/unit_system/tree/trunk/unit_system)
crate.

It derives `Unit` for a struct __B__ and declares structs which also derive
`Unit` and whose `BaseUnit` is the initial struct __B__. They form a typical SI
unit system. Roughly,

```rust
#[derive(Unit)]
struct B(f64);
```

results into

```rust
// The base unit is its own `BaseUnit`
impl Unit for B {
  type BaseUnit = Self;
  fn to_base(self) -> Self {
    self
  }
  fn from_base(base: B) -> Self {
    base
  }
}
struct NB(f64);
struct MuB(f64);
struct MB(f64);
struct CB(f64);
struct DeciB(f64);
struct HB(f64);
struct KB(f64);

/// kiloB is 1000 B
impl Unit for KB {
  type BaseUnit = B;
  fn to_base(self) -> B {
    B(self * 1000.)
  }
  fn from_base(base: Self::BaseUnit) -> Self {
    Self(base / 1000.)
  }
}

// ...the same for the rest of units
```

