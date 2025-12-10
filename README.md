# Null VM

Verifiable VM made in collaboration with Lambdaclass and 3MI

Non-null name TBD

## Testing

### ASM Tests

In order to add a new asm test you should add the `.s` file under `programs/asm`
Then add the corresponding test under `tests/asm.rs`

To run them you can use

`make test`

This will compile them and run the tests

### Rust Tests

In order to add a new rust test you should add the cargo project under `programs/rust` as a new directory.
The folder should have the same name as the `Cargo.toml` program name.
Then add the corresponding test under `tests/rust.rs`

You can run it with

`make test`
