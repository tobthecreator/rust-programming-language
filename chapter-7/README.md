When you use
`cargo new ...`

You're create a new package, which is signified by the Cargo.toml

Each Cargo crate has a convention that
`src/main.rs` is the root of the binary crate with the same name as the package. It also looks for `src/lib.rs` as its library crate root.

Cargo passes the crate root files to the build the library or binary.

If I run `cargo new my-project` and add both a `main.rs` and a `lib.rs` to it, I have a project with two crates: a library and a binary, both with the name `my-project`.

A project can have multiple binary crates by placing files in the `src/bin` directory

# Files as modules

In `src/lib.ts` we put `mod front_of_house` for the restaurant example.

Then in `src/front_of_house.rs` we put the definitions from the body of the `front_of_house` module.

So it would contain this:

```
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

and lib would contain

```
pub mod front_of_house;
```

Using the semicolon tells Rust to look for a file with that name to find the definitions we're looking for.

## Hosting Module

To further break it down, we need to update `front_of_house.rs` to only say `pub mod hosting;`.

As above, this will tell Rust to go looking for a file with the same name as the module, namely `hosting.rs`.

Now we'd create a new `front_of_house` folder, with a `hosting.rs` inside.

`src/front_of_house/hosting.rs`
