# mxmlextrema::semas3

<p align="center">
  <a href="https://lib.rs/crates/mxmlextrema-semas3"><img src="https://img.shields.io/badge/lib.rs-green"></a>
  <a href="https://docs.rs/mxmlextrema-semas3"><img src="https://img.shields.io/badge/Rust%20API%20Documentation-gray"></a>
</p>

mxmlextrema::semas3 (Semantic ActionScript 3) is a Rust library for creating, inspecting and modifying the semantic data of the ActionScript 3 language conforming to the Adobe AIR platform.

mxmlextrema::semas3 implements three dimensional names, property lookup, conversion, number representation, interface implementation log, method overriding, applying parameterized types, environment variable cache, unused entity tracking, a factory, and several entities (for example, classes, methods and variables).

## Example

Create a package `foo.bar` and log its fully qualified name:

```rust
let db = Database::new(Default::default());
let foo_bar = db.factory().create_package(["foo", "bar"]);
println!("Package name: {}", foo_bar.fully_qualified_name());
```

## License

Apache 2.0