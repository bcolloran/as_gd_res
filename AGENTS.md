# Running in Codex environment

The Codex environment runs the steps commands below to clone the `gdext` repository and checks out a specific commit, and create a `.cargo/config.toml` file including
```toml
[patch."https://github.com/bcolloran/gdext"]
godot = { path = "./gdext/godot" }
```
in order to point the Codex env at the required version of the `godot` crate.


```shell
git clone https://github.com/bcolloran/gdext.git ./gdext
cd ./gdext
git checkout f67a1ae43b1e06cd9a58ccb9d6330bf06c6e25ff
cd ..
mkdir -p .cargo
cat > .cargo/config.toml << EOF
[patch."https://github.com/bcolloran/gdext"]
godot = { path = "./gdext/godot" }
EOF
echo "created .cargo/config.toml--"
cat .cargo/config.toml
```

This is necessary because the `godot` crate is not published on crates.io, and the Codex environment has no network access to download it from GitHub. **The files that have been cloned into the `./godot` folder are totally irrelevant to our project, and should always be ignored. We just need to vendor them to get the Codex environment working. In all other circumstances, we use the files directly from git**

The agent should not change the contents of the `.cargo/config.toml` file, or the contents of the `godot` folder. The agent should confirm that the `.cargo/config.toml` file is created correctly and has the contents described, and that the `godot` folder exists and is not empty. **If these conditions are not met, the agent must immediately halt all work and inform the user.**

# Tests

**Before making any changes to the code, the agent must run all tests to ensure that they pass. If any tests fail, the agent must immediately halt all work and inform the user.**

## Running tests

The agent should always run `cargo test` in the workspace root to run all tests in the workspace, not just the tests in the current crate. Do not pass additional flags to `cargo test` unless specifically requested by the user.

