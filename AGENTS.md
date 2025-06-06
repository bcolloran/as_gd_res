
**Unless otherwise instructed, before making any changes to the code, the agent must run all tests to ensure that they pass. If any tests fail, the agent must immediately halt all work and inform the user. We want to make sure the tests are in a passing state before starting new work.**

# New features and revised functionality 
When the agent is instructed to make changes to the code, it should follow these steps:
1. **Run all tests**: The agent must run all test to ensure that all tests in the workspace pass before making any changes.
2. **Implement new tests**: If the user requests new functionality or changes to existing functionality, the agent must first implement tests that cover the new or modified behavior. The agent should not modify any code until the tests are in place.
3. **Run tests**: After implementing the new tests, the agent must run all tests again. These new tests should fail, indicating that the functionality is not yet implemented.
4. **Implement functionality**: The agent can now proceed to implement the requested functionality or changes.
5. **Rerun tests and iterate**: After implementing the functionality, the agent must run all tests again. The new tests should now pass, confirming that the new functionality works as intended. If any tests fail, the agent must address the issues before proceeding.

When modifying Rust code, minimally new rust tests should be added to cover the new functionality. If the new functionality is complex, the agent should also consider adding integration tests in the Godot project to ensure that the Rust code interacts correctly with Godot.



# Testing

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

## Verifying resource extraction output (end to end test)
The repository includes a helper script `codex_scripts/test_resource_extract.sh`
which runs `test_scene.tscn` headlessly and compares the printed output with
`expected_rust_print_output.txt`. The script automatically preloads the
extension's script classes by running the editor once in headless mode.
Run it after building the extension and setting `GODOT_BIN` to verify the
scene output:

```bash
export GODOT_BIN=/usr/local/bin/Godot_v4.4.1-stable_linux.x86_64
./codex_scripts/test_resource_extract.sh
```
