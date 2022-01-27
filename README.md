[![Crates.io](https://img.shields.io/crates/v/unit_system.svg)](https://crates.io/crates/unit_system)
[![Documentation](https://docs.rs/unit_system/badge.svg)](https://docs.rs/unit_system/)
[![CI](https://github.com/carrascomj/unit_system/workflows/ci/badge.svg)](https://github.com/carrascomj/unit_system/actions)

# unit_system

Automatically derived typed Unit system with conversions.

## Example

```rust
use unit_system::prelude::*;

#[derive(Unit)]
pub struct Meter(pub f32);
#[derive(Unit)]
pub struct Inch(pub f64);

// we can convert between any `Unit`s whose bases define BaseIntoBase
impl BaseIntoBase<Inch> for Meter {
  fn convert_base(self) -> Inch {
     Inch(self.0 as f64 * 40.)
  }
}

// CMeter, DMeter, HMeter, NMeter... have been automatically built
let centimeters: CMeter = CMeter(3.);
// the same for inches (non-SI units would require manual impls of Unit)
let kilo_inches: KInch = centimeters.convert_outer();

// KInch{ 0.012. }
println!("{:?}", kilo_inches);
assert!(f64::abs(kilo_inches.0 - 0.0012) < 1e-7);
```

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

> README.md is automatically generated on CI using [cargo-readme](https://github.com/livioribeiro/cargo-readme). Please, modify README.tpl or lib.rs instead (check [the github worflow](https://github.com/carrascomj/unit_system/blob/trunk/.github/workflows/readme.yml) for more details).
