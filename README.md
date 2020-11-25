WIP program aiming to replicate the functionality of the Echo discord bot: https://proxikal.github.io/Echo/Commands/

If you have any interest in this project, any PRs are welcome

The project's discord: https://discord.gg/84xRW98

# Building
If you want to build all keys as separate, loadable files, install `cargo-make` and then just run `cargo make`. This will use a lot of space (about 1.5GB)

If you want to bundle everything into one file, you can just run `cargo build` and it will compile with all the keys statically in the binary. You can add `--feature=loader` to the build command to add loading support

The current key list: https://adamski234.github.io/ars-parser/key_list