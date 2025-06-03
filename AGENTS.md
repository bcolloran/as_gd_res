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


