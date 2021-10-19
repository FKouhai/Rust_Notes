## Managing Growing Projects with Crates, Packages and Modules

- Packages -> A cargo feature that lets you build, test and share crates
- Crates -> A tree of modules that produces a library or executable
- Modules and use -> Let you control the organization, scope and privacy of paths
- Paths -> A way of naming an item, such as struct, function, or module

### Packages and crates

The crate root is a source file that the rust compiler starts and makes up the root module of your crate, a package is one or more crates that provide a set of functionality. A
package contains a Cargo.toml file that describes how to build those crates.

A lot of rules determine what a package can contain, it can contain at most 1 library crate and it can contain as many binary crates as needed but it must contain at least 1 crate
either if its a binary or a library

for example when running

```bash
cargo new project
ls -la project/src
main.rs
lib.rs 
```

This creates the package project, which contains the crate binary of project, if that directory (src) contains lib.rs, the package contains a library crate with the same name as
the package and src/lib.rs is its crate root
A package can contain multiple binary crates by placing files in the src/bin directory, each file will be a separated binary crate


### Defining modules to control scope and privacy

Modules let us organize code and control the privacy of items, which means whether an item is public(can be used outside the code) or private (in an internal implementation)

















































+
+
+
+
