# friendly-zoo

A friendly zoo! Use it to generate neat animal names.

## Example

```rust
use friendly_zoo::{Species, Zoo};

// default zoo
let animal = Zoo::default().generate();
println!("{}", animal);
// prints e.g. "cheerful_otter"

// build your own zoo!
let animal = Zoo::new(Species::Camel, 3).generate();
println!("{}", animal);
// prints e.g. "PoorBallsyElegantCamel"
```

## FAQ

### Why the stupid name?

Some guy is squatting `zoo` on crates.io :(
