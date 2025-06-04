# Tests

**Before making any changes to the code, the agent must run all tests to ensure that they pass. If any tests fail, the agent must immediately halt all work and inform the user.**

## Running Rust tests

The agent should always run `cargo test` in the workspace root to run all tests in the workspace, not just the tests in the current crate. Do not pass additional flags to `cargo test` unless specifically requested by the user.

## Running Godot integration tests

There are a handful of integration tests in `resource_test_godot_project/unit_test.gd` that are run using the `gdUnit4` framework. The agent should run these tests to ensure that the integration between the Rust gdextension and the Godot code is functioning as expected. To run the tests, the agent must follow these steps:
- download the Godot executable
- set the `GODOT_BIN` environment variable to the path of the Godot executable.
- build the Rust project using `cargo build`.
- ensure that the `runtest.sh` script in the `resource_test_godot_project/addons/gdUnit4/` directory is executable.
- `cd` to  the Godot project directory `resource_test_godot_project` directory.
- run the `runtest.sh` script with the `-a` flag followed by the path to the `unit_test.gd` file.

The following commands can be used to run the tests:

```
curl https://github.com/godotengine/godot-builds/releases/download/4.4.1-stable/Godot_v4.4.1-stable_linux.x86_64.zip
unzip Godot_v4.4.1-stable_linux.x86_64.zip -d /usr/local/bin
export GODOT_BIN=/usr/local/bin/Godot_v4.4.1-stable_linux.x86_64
cargo build
chmod +x ./resource_test_godot_project/addons/gdUnit4/runtest.sh
cd resource_test_godot_project
./addons/gdUnit4/runtest.sh -a res://unit_test.gd
```
