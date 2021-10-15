# Ownership

---

_The compiler checks at compile time a set of rules that manages memory_ 


- stack -> The stack is the fastest way to store a value since it fetches the fixed size in bits of a variable
- heap -> The heap is the slowest way to store a value since it has to allocate data, this data is accessed by a pointer

    The act of removing data from the stack is called pop, when you pop data out of the stack it removes the last element that was in, using a LIFO approach(Last In First Out),
    when you add data in to the stack its called push, when you push the data into the stack it goes to the top of the pile, this is a faster approach because the allocator never
    has to search for a place to store new data since it since that location is always at the top of the stack
    When you call a function the values passed into it the values get pushed into the stack(this values may include pointers to data on the heap), and when the function is over it
    pops out of the stack

### Ownership rules
  - Each value in Rust has a variable that's called owner
  - There can only be one owner at a time
  - When the owner goes out of scope the value will be dropped

### Variable Scope

The variable scope is fixed to the function, so when the function ends the variable doesn't have a value perse anymore or the variable isn't valid anymore 

### The String Type

Strings are stored in the heap instead of the stack, a string can be created using
```rust
let s = String::from("hello");
```
This calls the method String and uses the from function to create the string

When the variable s goes out of the scope there's a function called in the background called drop to free the memory used in that string, its called automatically after the closing
curly bracket

#### Ways variables and data interact: *Move*
```rust
let x = 5;
let y = x;
```

In the previous example x is bind the value 5 and then y is a copy from x, this only happens with values that has a known,fixed size 

```rust
let s1 = String::from("hello");
let s2 = s1;
```
In the previous example the variable s1 has the following data on it, a pointer to the heap where it is stored "hello",the length of the data and its capacity, when s2 is bind the
value of s1, s1 gets shallow copied into s2 making s1 obsolete, this means that the data on the heap doesn't get copied/cloned but instead it still has the same pointer only that
s1 is aiming at it

![[Pasted image 20211014182528.png]]


#### Ways variables and data interact: *Clone*

If instead of making a shallow copy we want to duplicate/clone the data in heap we have to use

```rust
let s1 = String::from("hello");
let s2 = s1.clone();
println!("s1={}, s2 ={}",s1,s2);
```

This way we will have our schema like this

![[Pasted image 20211014182957.png]]


### Ownership and Functions
The ownership with functions works very similarly as with variables

```rust
fn main(){
let s = String::from("hello"); //S gets into the scope

takes_ownership(s); //S value moves into the function, so its no longer valid on main 
let x = 5; // x comes into the scope 
makes_copy(x);// X moves into the function but still stays valid on main 
}//X goes out of the scope 

fn takes_ownership(some_string: String) { //some_string comes into the scope
println!("{}", some_string);
}//some_string goes out of the scope and drop is called 
//Memory is free 

fn makes_copy(some_integer: i32){ //some integer comes into the scope 
println!("{}",some_integer);
}//some_integer goes out of the scope. Nothing special happens

```

If s was called after calling takes_ownership, rust would've thrown a compile time error
Returning values can also transfer ownership

```rust
fn main(){
let s1 = gives_ownership(); //gives_ownership moves its return value to s1
let s2 = string::from("hello"); //s2 comes into the scope 
let s3 = take_and_gives_back(s2); // s2 is moved into take_and_gives_back which also moves its return value to s3 
}// s3 goes out of the scope and its dropped, s2 goes out but was moved so it still the same and s1 gets dropped
fn gives_ownership() -> String { //Moves its return value into the function that calls it 
let some_string = String::from("hello");//some_string comes into scope 

some_string //some string gets returned and moves out to the calling function 
}

fn take_and_gives_back(a_string: String) -> String { //Will take a string and return a String 
a_string //a_string is returned and moves out to the calling function 

}

```
It is also possible to return multiple values using a tuple

```rust
fn main(){
let s1 = String::from("hello");
let (s2, len) = calculate_lenght(s1);
println!("The lenght of {} is {}",s2,len);
}

fn calculate_lenght(s: String) -> (String, usize) {
let lenght = s.len();
(s,lenght);
}

```
## References and Borrowing

A better approach instead of having to return the string to the calling function we could make calculate_length to take a reference as a parameter instead of taking the ownership
of the value

```rust
fn main(){
let s1 = String::from("hello");
let len = calculate_length(&s1); //&s1 refers to the value of s1, since the reference doesnt own the value it wont be dropped when the reference goes out of the scope
println!("The length of {} is {}",s1, len);

}
fn calculate_length(s: &string) -> usize { //s is a reference to a string
s.len()
}//Here s goes out of the scope

```

Using the ampersand allows you to refer to some value without taking ownership of it 

![[Pasted image 20211014193359.png]]

You cannot modify data that its not owned by you for example 

```rust
fn main() {
let s = String::from("hello");
change(&s);
}
fn change(some_string: &string) {
some_string.push_str(",world");
}

```
In that example you will get an error at run time
You could make s mutable and then create a mutable reference when calling the function and after that you will need to accept it

But mutable references has one restriction which is that you are unable to reference multiple times a particular piece of that within a particular scope

\i.e
```rust
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s;

println!("{}, {} ", r1, r2);
```
This hard restrictions are useful to avoid data races which could cause undefined behavior and 


One valid way of solving that issue would be doing 

```rust
let mut s = String::from("hello");
let r1 = &s;
let r2 = &s;
println("{} and {}", r1 ,r2);
let r3 = &mut s;
println!("{}",r3);

```
This doesn't cause any problem because r1 and r2 scopes end after the println! which is before the mutable reference r3 is created so these scopes wont overlap

#### Dangling References

In other languages with pointer its easy to mistakenly create a dangling pointer which is a pointer that references a location in memory that may have been given to someone else
and by freeing some memory while preserving a pointer to that memory
In Rust the compile ensures that no data goes out of its scope so a dangling reference cannot happen

### The rule of References 

- At any given time you can have either one mutable reference or any number of immutable references
- References must be always valid

## Slices

Slices does not have ownership, they allow you to reference a contiguous sequence of elements in a collection

An example of a string slice could be

```rust
let s = String::from("hello World");
let hello = &s[0..5];
let world = &s[6..11];

```
In reality this is a reference to a portion of string 

![[Pasted image 20211015220902.png]]

To get the first word of a string:

```rust
fn main() {
  let mut s = String::from("hello World");
  let word = first_word(&s);
  println!("The first word is {}",word);
}

fn first_word(s: &String) -> &str {
  let bytes = s.as_bytes();
  for (i,&item) in bytes.iter().enumerate(){
    if item == b' ' { //We check for anything before any whitespace 
      return &s[0..i]; 
    }
  }
  &s[..];
}
```
### Other slices

Another kind of slice that could be taken is a slice from an array for example

```rust
let a = [1,2,3,4,5,6];
let slice = &a[1..3];
assert_eq!(slice, &[2,3]);

```

