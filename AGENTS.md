# Tests

**Before making any changes to the code, the agent must run all tests to ensure that they pass. If any tests fail, the agent must immediately halt all work and inform the user.**

## Running Rust tests

The agent should always run `cargo test` in the workspace root to run all tests in the workspace, not just the tests in the current crate. Do not pass additional flags to `cargo test` unless specifically requested by the user.

## Running Godot integration tests
The integration tests live in `resource_test_godot_project/unit_test.gd` and use the `gdUnit4` framework.  Running them requires the Godot binary and a working headless setup.  Follow the steps below whenever the integration tests need to be executed.

1. Download Godot and set the `GODOT_BIN` environment variable:
   ```bash
   curl -L https://github.com/godotengine/godot-builds/releases/download/4.4.1-stable/Godot_v4.4.1-stable_linux.x86_64.zip -o godot.zip
   unzip godot.zip -d /usr/local/bin
   export GODOT_BIN=/usr/local/bin/Godot_v4.4.1-stable_linux.x86_64
   chmod +x "$GODOT_BIN"
   ```
2. Ensure `xvfb-run` is available (install `xvfb` if necessary).
3. Build the Rust extension so the `.gdextension` file can locate the library:
   ```bash
   cargo build
   ```
4. Make sure the test runner script is executable:
   ```bash
   chmod +x ./resource_test_godot_project/addons/gdUnit4/runtest.sh
   ```
5. Preload the plugin's script classes by running the Godot editor once in headless mode:
   ```bash
   xvfb-run "$GODOT_BIN" --headless --editor --path resource_test_godot_project --quit
   ```
6. Execute the tests:
   ```bash
   cd resource_test_godot_project
   xvfb-run ./addons/gdUnit4/runtest.sh -a res://unit_test.gd
   ```
