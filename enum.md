## Enums

Enumerations allows you to define a type by enumerating its possible variants

### Defining an Enum

A situation where enums could be useful is when you are working with IP addresses, there are 2 standards and you can have an IPv4 or IPv6 but never be both at the same time

this is useful because enum values can only be one of its variants

This is how you define an enum 

```rust
enum IPaddrKind {
  V4,
  V6,
}

let four = IPAddrKind::V4;
let six = IPAddrKind::V6;

struct IpAddr {
  kind: IPAddrKind,
  address: String,
}
let loppback4 = IpAddr {
  kind : IPAddrKind::V4,
  address: String::from("127.0.0.1"),
};

//Another way of doing it without creating another strcut

enum IpAddr {
  V4(u8,u8,u8),
  V6(String),
}

let lo4 = IpAddr::V4(127,0,0,1);
let lo6 = IpAddr::V6(String::from("::1"));

//The standard library defines IpAddr, so the enum for IpAddr could be

enum IpAddr {
  V4(Ipv4Addr),
  V6(Ipv6Addr),
}
```
enums are useful when we cannot define a function in an easy manner if we were using structs for example

```rust
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32,i32,i32),
}

impl Message {
  fn call(&self) {
    println!("{}", &self);
}
}

let m = Message::Write(String::from("hello"));
m.call();
```
Another type of enum is the Option Enum, this is used when you need to encode the scenario where a value could be something or nothing
This functionality could prevent bugs that are very common in other languages


A way of declaring this would be 

```rust
enum Option<T> {
  None,
  Some(T),
}

let some_number = Some(5);
let some_string = Some("a String");
let absent: Option<i32> = None;

```
When we use None instead of some we need to declare the Option<T>, because the compiler cant infer the type that the some variant will hold by looking at a None value
Having none could be same as Null in other programming languages



Also note that Otion<T> and T are different types being T an String or an int or any sort of data structure

### The match control flow operator

match allows you to compare a value against a series of patterns and then execute code on which pattern matches, they could be made of literals, variable names, wildcards, etc ,etc

```rust
enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,

  }
}

```

Match could be similar as the switch statement in other programming languages, in Rust theres also the placeholder _ which is useful if we dont want to have a list of all the
patterns and we only care about a few or some of them



### if let 


This syntax lets you combine if and let into a less verbose approach to handle values that match on pattern while ignoring the rest

```rust 
let some_u8_value = Some(u8);
if let Some(3) = some_u8_value {
  println!("three");
}


```

If let also allows else to handle the other cases 


```rust

let mut count = 0;
if let Coin::Quareter(state) = coin {
  println!("State quarter from {:?}", state);
  } else {
    count += 1;
  }
```










































































































































































