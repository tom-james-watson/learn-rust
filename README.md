Learning Rust
=============

My notes and exercises for learning Rust.

## Variables + Mutability

Variables are immutable by default. Use `mut` to allow mutation.

```rust
let x = 5;
x = 6; // Error!

let mut y = 5;
y = 6; // OK.
```

Constant variables can also be created.

* Must be type annotated.
* Can only be set to value of a constant expression, i.e. cannot be set to the result of a function call.

```rust
const MAX_POINTS u32 = 100;
```

Variables can be "shadowed", where you declare a new variable with the same name as a previous one.

This is different to `mut` because it lets you mutate a variable intentionally but have it stay immutable otherwise.

```rust
let x = 5;
let x = x + 1;
println!("x is {}", x); // x is 6
```

You can also use shadowing to change the type of a variable, which is not possible with `mut`.

```rust
let spaces = "     ";
let spaces = spaces.len();
println!("spaces is {}", spaces); // spaces is 5

let mut spaces = "     ";
spaces = spaces.len(); // expected &str, found usize
```

Shadowing a variable will make the previous version of the variable inaccessible. However, note that shadowed variables are limited to the scope in which they were created, so this is not always strictly true.

```rust
let spaces = "     ";
if true {
  let spaces = spaces.len();
  println!("spaces is '{}'", spaces); // spaces is '5'
}
println!("spaces is '{}'", spaces); // spaces is '     '
```

## Data Types

### Scalar Types

#### Integer

| Length  | Signed  | Unsigned |
| ------- | ------- | -------- |
| 8-bit   | `i8`    | `u8`     |
| 16-bit  | `i16`   | `u16`    |
| 32-bit  | `i32`   | `u32`    |
| 64-bit  | `i64`   | `u64`    |
| 128-bit | `i128`  | `u128`   |
| arch    | `isize` | `usize`  |

`i8` is -128 to 127, `u8` is 0 to 255.

`isize`/`usize` is dependent on system - 32-bit on 32-bit machines and 64-bit on 64-bit machines.

#### Float

`f32` and `f64`. `f64` is default as it is more accurate than `f32` and generally about as performant.

```rust
let x = 2.0; // f64
let y: f32 = 3.0; // f32
```

#### Boolean

`bool` - either `true` or `false`.

```rust
let f: bool = false;
```

#### Char

Used to represent a Unicode Scalar Value.

```rust
let c = 'z';
let z = 'ℤ';
let heart_eyed_cat = '😻';
```

### Compound Types

#### Tuple

Fixed length - cannot grow or shrink.

```rust
let values: (i32, f64, u8) = (500, f64, u8);
```

Type annotations are optional.

```rust
let values = (500, 6.4, 1);
```

Can be destructured.

```rust
let (x, y, z) = values;
```

Access values by index with `.<index>`.

```rust
let x = tup.0
```

#### Array

Fixed length. If you want variable length use a Vector. Used to implement stack-based structures.

```rust
let a = [1, 2, 3];
```

Can initialize array with a value.

```rust
let a: [u8; 3] = [0; 3] // initialize array of 3 unsigned 8-bit ints to zero
```

Can access/set values by index

```rust
let a: [u8; 3] = [0; 3]
a[0] = 1;
a[1] = 2;
a[2] = 3;
// a = [1, 2, 3];
```

Rust will panic if index is out of bounds (as opposed to C, for example, that will let you just read memory that doesn't belong to the structure).

## Functions

Declared with `fn`.

```rust
fn a_function() {
  println!("A function was called");
}
```

Parameters must have type annotations.

```rust
fn another_function(x: i32, y: u8) {
  println!("Another function was called with {}, {}", x, y);
}
```

Can be defined out of order.

### Statements vs Expressions

#### Statements

Statements are instructions that perform some action and do not return a value.

```rust
fn main() {
  let y = 6;
}
```

`let y = 6;` is a statement. Function definitions are also statements; the entire preceding example is a statement in itself.

As statements do not return values, you can't assign a variable to a statement.

```rust
let x = (let y = 6); // Error: expected expression, found statement (`let`)
```

`let x = 5` does not return a value, which is different to languages like C where you can do things like `x = y = 6`.

#### Expressions

Expressions evaluate to a resulting value. Expressions do not have semicolons.

Examples are:

* Calling a function.
* Calling a macro.
* Operations such as `5 + 6`.
* Blocks that are used to create new scopes, `{}`.

The expression

```rust
{
  let x = 3;
  x + 1
}
```

is a block expression which evaluates to `4`, which means you can use it to bind the returning value to a variable in a `let` statement.

```rust
let y = {
  let x = 3;
  x + 1
};
```

Note that `x + 1` does not have a semicolon. Adding a semicolon would turn the expression into a statement, which will then not return a value.

### Functions with Return Values

Return values are not named, but we do declare their type after an arrow `->`. Rust automatically returns the value of the final expression of a function. You can explicitly return early with `return`.

```rust
fn five() -> i32 {
  5 // having no semicolon makes this an expression
} // returns 5
```

## Control Flow

### `if` Expressions

```rust
let number = 6;

if number % 3 == 0 {
  println!("number is divisible by 3");
} else if number % 2 == 0 {
  println!("number is divisible by 2");
} else {
  println!("number is not divisible by 3, or 2");
}
```

Must be provided a expression that evaluates to a `bool`. Does not do "truthiness" checks like e.g. Javascript.

```rust
let number = 3;

if number {
  println!("number was three");
} // Error: expected bool, found integral variable
```

### Using `if` in a `let` Statement

Because if is an expression, we can use it on the right side of a let statement.

```rust
let condition = true;
let number = if condition {
  5
} else {
  6
};
```

> Nicer than ternary operator

### Loops

#### `loop`

Can be used as `while true` equivalent.

```rust
loop {
  println!("again!");
}
```

Can stop a `loop` with `break`. `break` also allows you to return a value.

```rust
let mut counter = 0;

let result = loop {
  counter += 1;

  if counter == 10 {
    break counter * 2;
  }
};

assert_eq!(result, 20);
```

#### `while`

Basically the same as `loop` but let's you omit a lot of unnecessary `loop`, `if`, `else`, and `break`.

```rust
let mut number = 3;

while number != 0 {
  println!("{}!", number);

  number = number - 1;
}

println!("LIFTOFF!!!");
```

#### `for`

For loops can only be used to iterate over collections. There is no C-style for loop in rust.

```rust
let a = [10, 20, 30, 40, 50];

for element in a.iter() {
  println!("the value is: {}", element);
}
```

`for (int i = 0; i < 4; i++) {` equivalent would be

```
for number in (1..4).rev() {
  println!("{}!", number);
}
println!("LIFTOFF!!!");
```

## Ownership

Ownership is Rust’s most unique feature, and it enables Rust to make memory safety guarantees without needing a garbage collector.

Rust automatically calls a `drop` function that frees any heap-allocated variables once their owner goes out of scope. This means you do no have to manually `free` allocated variables, which avoids an entire class of memory deallocation bugs, nor do you need a garbage collector.

```rust
{
    let s = String::from("hello"); // s is valid from this point forward

    // do stuff with s
}                                  // this scope is now over, and s is no
                                   // longer valid
```

A heap-allocated variable can have at most one owner. Assigning a heap-allocated variable to a new variable will invalidate the first reference. For example the following code:

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```

Will throw the following error:

```
error[E0382]: use of moved value: `s1`
 --> src/main.rs:5:28
  |
3 |     let s2 = s1;
  |         -- value moved here
4 |
5 |     println!("{}, world!", s1);
  |                            ^^ value used here after move
  |
  = note: move occurs because `s1` has type `std::string::String`, which does
  not implement the `Copy` trait
```

If you’ve heard the terms shallow copy and deep copy while working with other languages, the concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow copy. But because Rust also invalidates the first variable, instead of being called a shallow copy, it’s known as a move. In this example, we would say that s1 was moved into s2.

In addition, there’s a design choice that’s implied by this: Rust will never automatically create “deep” copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.

### Clone

If you do want a deep copy, you can use the `clone` method.

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);`
```

This is now fine, though this will copy the heap data and is expensive.

### Copy

Rust has a special annotation called the `Copy` trait that we can place on types like integers that are stored on the stack. If a type has the `Copy` trait, an older variable is still usable after assignment. Here are some of the types that are `Copy`:

* All the integer types, such as `u32`.
* The Boolean type, `bool`, with values `true` and `false`.
* All the floating point types, such as `f64`.
* The character type, `char`.
* Tuples, if they only contain types that are also `Copy`. For example, `(i32, i32)` is `Copy`, but `(i32, String)` is not.

### Ownership and Functions

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

### Return Values and Scope

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

## References and Borrowing

Taking ownership and then returning ownership with every function is a bit tedious. What if we want to let a function use a value but not take ownership?

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

These ampersands are references, and they allow you to refer to some value without taking ownership of it. Because a reference does not own it, the value it points to will not be dropped when the reference goes out of scope.

We call having references as function parameters _borrowing_.

References are immutable by default.

### Mutable References

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

First, we had to change `s` to be `mut`. Then we had to create a mutable reference with `&mut s` and accept a mutable reference with `some_string: &mut String`.

But mutable references have one big restriction: you can have only one mutable reference to a particular piece of data in a particular scope. This code will fail:

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);
```

Here's the error:

```
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> src/main.rs:5:10
  |
4 | let r1 = &mut s;
  |          ------ first mutable borrow occurs here
5 | let r2 = &mut s;
  |          ^^^^^^ second mutable borrow occurs here
6 | println!("{}, {}", r1, r2);
  |                    -- borrow later used here
```

This prevents _data races_ at compile time. As always, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:

```rust
let mut s = String::from("hello");

{
    let r1 = &mut s;

} // r1 goes out of scope here, so we can make a new reference with no problems.

let r2 = &mut s;
```

We also cannot have a mutable reference while we have an immutable one. Users of an immutable reference don’t expect the values to suddenly change out from under them. The following code will fail:

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM

println!("{}, {}, and {}", r1, r2, r3);
```

Here's the error:

```
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:10
  |
4 | let r1 = &s; // no problem
  |          -- immutable borrow occurs here
5 | let r2 = &s; // no problem
6 | let r3 = &mut s; // BIG PROBLEM
  |          ^^^^^^ mutable borrow occurs here
7 |
8 | println!("{}, {}, and {}", r1, r2, r3);
  |                            -- borrow later used here
```

## Slices

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];

// or

let hello = &s[0..=4];
let world = &s[6..=10];

// or

let hello = &s[..5];
let world = &s[6..];

let hello_world = &s[..];
```

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

### String Literals Are Slices

Recall that we talked about string literals being stored inside the binary. Now that we know about slices, we can properly understand string literals:

```rust
let s = "Hello, world!";
```

The type of `s` here is `&str`: it’s a slice pointing to that specific point of the binary. This is also why string literals are immutable; &str is an immutable reference.

### String Slices as Parameters

We can update our `first_word` function to accept a string slice instead, which allows it to be called with both `String` and `&str` values.

```rust
fn first_word(s: &str) -> &str {
```

```rust
let my_string = String::from("hello world");

// first_word works on slices of `String`s
let word = first_word(&my_string[..]);

let my_string_literal = "hello world";

// first_word works on slices of string literals
let word = first_word(&my_string_literal[..]);

// Because string literals *are* string slices already,
// this works too, without the slice syntax!
let word = first_word(my_string_literal);
```

### Other Slices

You can also take slices of other types of collections, such as arrays:

```rust
let a = [1, 2, 3, 4, 5];
let slice: &[i32] = &a[1..3];
```

## Structs

```rust
struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}
```

### Creating a Struct Instance

```rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

### Creating a Mutable Struct Instance

```rust
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("anotheremail@example.com");
```

Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable.

### Using the Field Init Shorthand when Variables and Fields Have the Same Name

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

### Creating Instances From Other Instances With Struct Update Syntax

Instead of:

```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};
```

We can do:

```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```

### Tuple Structs

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

### Methods

```rust
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn widen(&mut self, increase: u32) {
        self.width += increase;
    }
}

let mut rect = Rectangle { width: 30, height: 50 };

println!(
    "The area of the rectangle is {} square pixels.",
    rect.area()
);

rect.widen(10);
```

### Associated Functions

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

let sq = Rectangle::square(3);
```

To call this associated function, we use the `::` syntax with the struct name. This function is namespaced by the struct: the `::` syntax is used for both associated functions and namespaces created by modules.

## Enums

Enums work similarly to other languages:

```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
```

Data can also be attached to each variant of the enum. This data can have different types, which is something you couldn't do with a plain struct:

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```

More terse than creating multiple structs. For example the following:

```rust
enum Message {
    Quit, // no data associated with it
    Move { x: i32, y: i32 }, // an anonymous struct
    Write(String), // a single string
    ChangeColor(i32, i32, i32), // three integers
}
```

Would be equivalent to the following structs:

```rust
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
```

But if we used the different structs, which each have their own type, we couldn’t as easily define a function to take any of these kinds of messages as we could with the `Message` enum.

### Enum Methods

We can define methods on enums in a similar way to how we can on structs:

```rust
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

### The `Option` Enum and Its Advantages over Null Values

Rust does not have the concept of `null`. Rust forces you to explicitly opt-in to allowing a `None` value by defining a variable as having the `Option<T>` type, which is defined as so:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

For example:

```rust
let some_number: Option<i32> = Some(5);
let some_string: Option<&str> = Some("a string");

let absent_number: Option<i32> = None;
```

Then, in order to be able to actually to use the value as a variable of type `T`, you need to convert from type `Option<T>` to type `T`, explicitly handling the case where the variable is `None`.

Any variable that is type `T` can therefore be guaranteed to never be `None`, avoiding the whole set of errors where a variable is unexpectedly `null`.

## Match

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => { // curly brackets optional
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### Patterns That Bind To Values

```rust
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

value_in_cents(Coin::Quarter(UsState::Alaska))
```

### Matching With `Option<T>`

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

### Matches Are Exhaustive

Matches must handle all variants of an enum.

### The `_` Placeholder

If you don't want to explicitly handle all variants, you can use the `_` placeholder to match any other value.

```rust
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
```

The `()` is the unit value, so nothing will happen if `_` is matched.

### `if let`

Sometimes the `match` control flow is overly verbose. Instead of:

```rust
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}
```

```rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```
