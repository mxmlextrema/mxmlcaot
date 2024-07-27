# mxmlextrema::as3query

<p align="center">
  <a href="https://lib.rs/crates/mxmlextrema-as3query"><img src="https://img.shields.io/badge/lib.rs-green"></a>
  <a href="https://docs.rs/mxmlextrema-as3query"><img src="https://img.shields.io/badge/Rust%20API%20Documentation-gray"></a>
</p>

mxmlextrema::as3query is a Rust library for creating, inspecting and modifying the semantic data of the ActionScript 3 language conforming to the Adobe AIR platform.

mxmlextrema::as3query implements three dimensional names, property lookup, type conversion mechanism, variable number representations, interface implementation mechanism, method overriding mechanism, type substitution for parameterized types, an item factory, and several item representations (for example, classes, methods and variables).

## Example

Create a package `foo.bar` and log its fully qualified name:

```rust
let db = Database::new(Default::default());
let foo_bar = db.factory().create_package(["foo", "bar"]);
println!("Package name: {}", foo_bar.fully_qualified_name());
```

## License

Apache 2.0