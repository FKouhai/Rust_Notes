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

```rust
mod front_of_house {
  mod hosting {
    fn add_to_waitlist(){}
    fn seat_at_table() {}

  }
mod serving {
  fn take_order() {}
  fn serve_order() {}
  fn take_payment(){}
}

}
```

We define a module by using the mod keyword, and then specify the name of the module, inside of the module we can have another modules


### Paths for referring to an item in a Module tree

To show Rust where to find an item in a module tree, we use a path in the same way we use a path when navigating the fs. If we want to call a function we need to know its path



```rust
mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist(){}

  }
}

pub fn eat_at_restaurant(){
  crate::front_of_house::hosting::add_to_waitlist(); //This is how you call the function add_to_waitlist using its absolut path
  front_of_house::hosting::add_to_waitlist(); // This is how you call the function add_to_waitlist using its relative path
}
```
Take into account that this will let us compile the code because the specific function we are calling from our public function is also public

Another way of doing this would be 

```rust
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

```
the super keyword constructs the relative path as well its like starting the file system path with .. 

### Bringing Paths into Scope with the use keyword

We can bring a path into the scope once instead of repeating the name of the module 

for example

  ```rust
  mod front_of_house{
    pub mod hosting {
      pub fn add_to_waitlist(){}
    }
  }

  use crate::front_of_house::hosting;
  pub fn eat_at_restaurant(){
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
  }

  ```

Bringing the parent path to the scope is the idiomatic way in rust of calling the function we could've used 
```rust 
use crate::front_of_house:hosting::add_to_waitlist;
```

There is another solution to the problem of bringing 2 types of the same name into the same scope with use, and its by using the as keyword, this is used so we wont have problems
at compile time due to 2 local variables being called the same 

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
    Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    Ok(())
}

```

Re exporting names with pub use


To enable code that calls our code to refer to the name we brought into the scope we can combine pub and use


```rust
pub use crate::front_of_house::hostings;
```

We can also use nested paths to clean up large use lists

```rust
use std::cmp::Ordering;
use std::io;
use std::io::Write;

//would be better to use

use std::{cmp::Ordering, io};
use std::io::{self, Write};
use std::collections::*; //This is known as the glob parameter it brings everything under Collections into the scope
```

When working with modules from different files we use mod name;

```rust 
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}


```
That way we tell rust to load the module front_of_house from another file with the same name as the module







































































































































+
+
+
+
