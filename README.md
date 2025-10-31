# gd_bind_sync

A Rust workspace that generates Godot `Resource` wrappers for your data types.

This project defines derive macros and traits for converting between pure Rust
structs/enums and Godot's runtime types. The included example demonstrates how
these macros can be used inside a small Godot extension library.

## Workspace Layout

- **`as_gd_res`** – Library providing the `AsGdRes`, `AsGdEnumSimple` and
  `ExtractGd` traits, along with helpers for common engine types.
- **`as_gd_res_derive`** – Procedural macro for `#[derive(AsGdRes)]`.
- **`as_simple_gd_enum_derive`** – Procedural macro for `#[derive(AsGdEnumSimple)]`.
- **`resource_test_rust`** – Example crate compiled as a `cdylib` to test the
  derives from Godot. It is paired with the `resource_test_godot_project`
  directory which contains a minimal Godot project.

## Building

- Ensure you have the nightly Rust toolchain installed

- Build all crates with Cargo:

   ```bash
   cargo build --workspace --all-targets
   ```

- To run the example Godot project, open the `resource_test_godot_project`
   folder with the Godot editor and enable the compiled extension library.

## Updating Godot version

**Make sure to update the download path in `.github/workflows/resource_output_test.yml`!!!!!**

## Testing Godot Integration locally

Run the script `test_resource_extract_local.sh` to execute the end to end test
that runs the `test_scene.tscn` and compares the printed output with the expected
output. Make sure to set the `GODOT_BIN` environment variable to point to your Godot
binary before running the script.

**Latest is:**
```shell
export GODOT_BIN=/home/bc/Desktop/Godot_v4.5.1-stable_linux.x86_64
./test_resource_extract_local.sh
```

## Usage Overview

Implement the traits or use the derive macros to bridge between Rust and Godot
resources. For example:

```rust
#[derive(as_gd_res::AsGdRes, Debug, Clone)]
struct MyData {
    pub name: String,
    pub value: i32,
}

#[derive(as_gd_res::AsGdEnumSimple, Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Element {
    #[default]
    Fire,
    Water,
}
```

The macros generate `Resource` structs compatible with Godot and implement
`ExtractGd` so you can convert the generated resources back into your original
Rust types.

### Limitations

- The derive macros do **not** support types with generic parameters.
- `#[derive(AsGdRes)]` only works on structs with named fields or enums where
  every variant is unit-like or a single-tuple variant.

## License

This project is licensed under the MIT License.
