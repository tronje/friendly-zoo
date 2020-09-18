# friendly-zoo

A friendly zoo! Use it to generate neat animal names.

## Example

```rust
use friendly_zoo::Zoo;

// default zoo
let animal = Zoo::default().generate();
println!("{}", animal);
// prints e.g. "cheerful_otter"

// build your own zoo!
let animal = Zoo::new('-', 3).generate();
// prints e.g. "poor-ballsy-elegant-camel"
```

## FAQ

### Why the stupid name?

Some guy is squatting `zoo` on crates.io :(
