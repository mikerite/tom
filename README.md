# TOM

Yet another TOML parser. Preserves whitespace for real this time!

Work in progress, take a look at
[Molten](https://github.com/LeopoldArkham/Molten) or
[toml-edit](https://github.com/ordian/toml_edit) for something
relatively more ready.

The best documentation at the moment is
[./examples/api-walkthrough.rs](./examples/api-walkthrough.rs).


# Contributing

Contributions are very much welcome! Keep in mind that the code is
very much in experimental state, and so good contributing guides are
missing, CI is non-existent, formatting is artisan, etc.  Feel free to
ask questions by creating issues/PRs, or by pinging @matklad at the
[Rust discord](https://discordapp.com/channels/442252698964721669/).

Currently, beta version of Rust is required to build the code.

The project is a fairly-standard Rust crate, so `cargo test` is the
main command. Note, however, that we use code-generation heavily. See
`./.cargo/config` file for available codegen tasks, and the `./tasks`
subcrate for their definitions.
