# Chapter 5
---

## Using Structs to Structure related data

The definition of a struct is a custom data type that lets you name and package multiple values that define a group

### Defining and Instancing Structs

Structs are similar from tuples in the way that the data they contain could be from different types, for example it could contain Strings, i32 and f64 in the same struct and tuple
However structs are more flexible than tuples since you don't have to rely on the order of the data to access the values of an instance

The way to declare a struct is

```rust 
struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

```

They are sort of defined in the same way as in Golang

To create or use an instance of the struct we have created 

```rust 
let user1: User {
  email: String::from("ex@example.com"),
  username: String::from("example"),
  active: true.
  sign_in_count: 1,
};

```
The way to get the data from the struct is similar from other languages where we use the dot notation to get an element from the method/instance

which is by using *user1.email*

We could write a function that creates users by the given inputs

```rust 

fn build_user(email: String, username: String) -> User {
User {
  email,//We are using the field init shorthand
  username,
  active: true,
  sign_in_count: 1,
}

}
```
We could also create instances from other instances, this could be useful if we need to copy some data from a previous instance
```rust 
let user2 = User {
  email: String::from("ex2@example.com"),
  username: String::from("example2"),
  ..user1 //This way we copy the data left to finish the user2 definition from user1
}

```
### Defining tuple structs 

Tuple structs could also be created, this types of structs has meaning added to the struct name but doesnt have names associated with their fields,

```rust
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

let black = Color(0,0,0);
let origin = Point(0,0,0);
```
Each structs defined is its own type although sometimes the values on them and even the data types could be identical 

### Method Syntax

Methods are close to functions, since they are declared with the fn keyword and can have parameters and return values however they defer from functions in the context they are 
defined within, which is the same context as the struct and whose first parameter is always self which is an instance of the struct the method is being called on

another way of writing the example from this chapter could be

```rust 
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }
}
fn main() {
let rect1 = Rectangle {
  width: 30,
  height: 50,
};
println!("The area of the rectangle is {} square pixels",rect1.area());
}
```

To define the function within the context of Rectangle we start an impl(Implementation) block, then move the area function within the impl curl braces and change the first
parameter to self in main we declared rect1 and we are passing it as an argument to the area function 

This is similar to how methods work in C# you first spawn an instance of the object and then you can call all of its methods






















































