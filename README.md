Simplify.rs is a high-performance Rust polyline simplification library by Vladimir Agafonkin, extracted from [Leaflet](http://leafletjs.com).

Checkout the demo with docs: http://mourner.github.io/simplify-js/

#### Usage

```rust
use simplify::{simplify, Point2D};

let points = vec![
    Point2D::new(224.55, 250.15),
    Point2D::new(226.91, 244.19),
    Point2D::new(233.31, 241.45),
    Point2D::new(234.98, 236.06),
];

// `None` uses a tolerance of 1; the flag asks for the slower, higher-quality mode.
let simplified = simplify(&points, Some(5.0), false);
```

Any type can be simplified by implementing `Point`, and the points you pass in
are the points you get back, so they can carry whatever else you need:

```rust
use simplify::{simplify, Point};

#[derive(Clone)]
struct Waypoint { lon: f64, lat: f64, name: String }

impl Point for Waypoint {
    fn x(&self) -> f64 { self.lon }
    fn y(&self) -> f64 { self.lat }
}
```

#### Building

```
cargo build --release
cargo test
cargo bench
```

#### Ports

 * JavaScript: [mourner / simplify-js](https://github.com/mourner/simplify-js) (by Vladimir Agafonkin)
 * Python: [omarestrella / simplify.py](https://github.com/omarestrella/simplify.py) (by Omar Estrella)
 * PHP: [AKeN / simplify-php](https://github.com/AKeN/simplify-php) (by Rotari Gheorghe)
 * PHP: [andreychumak / simplify-php](https://github.com/andreychumak/simplify-php) (by Andrey Chumak)
 * Java: [ekeneijeoma / simplify-java](https://github.com/ekeneijeoma/simplify-java) (by Ekene Ijeoma)
 * Java: [hgoebl / simplify-java](https://github.com/hgoebl/simplify-java) (by Heinrich Göbl)
 * Processing: [ekeneijeoma / simplify-processing](https://github.com/ekeneijeoma/simplify-processing) (by Ekene Ijeoma)
 * AS3: [fnicollet / simplify-as3](https://github.com/fnicollet/simplify-as3) (by Fabien Nicollet)
 * Rust: [calvinmetcalf / simplify-rs](https://github.com/calvinmetcalf/simplify-rs) (by Calvin Metcalf)
 * Rust: [kade-robertson / simplify-polyline](https://github.com/kade-robertson/simplify-polyline) (by Kade Robertson)
 * Ruby: [odlp / simplify_rb](https://github.com/odlp/simplify_rb) (by Oliver Peate)
 * Go: [yrsh / simplify_go](https://github.com/yrsh/simplify-go) (by Anton Korotkikh)
 * C# (Portable): [imshz / simplify-net](https://github.com/imshz/simplify-net) (by Shees Ul-Hassan)
 * Swift: [malcommac / SwiftSimplify](https://github.com/malcommac/SwiftSimplify) (by Daniele Margutti)
 * Unreal Engine: [SINTEF-9012 / SimplifyUnreal](https://github.com/SINTEF-9012/SimplifyUnreal) (by Antoine Pultier)
 * Postgres (using PL/Python): [shubhamjain / simplify-coordinates-sql](https://github.com/shubhamjain/simplify-coordinates-sql) (by Shubham Jain)
