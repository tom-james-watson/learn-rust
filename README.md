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

| Length | Signed | Unsigned |
|--------|--------|----------|
| 8-bit | i8 | u8 |
| 16-bit | i16 | u16 |
| 32-bit | i32 | u32 |
| 64-bit | i64 | u64 |
| 128-bit | i128 | u128 |

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

Return values are not named, but we do declare their type after an arrow `->`. Rust automatically returns the value of the final expression of a function (gross!). You can explicitly return early with `return`.

```rust
fn five() -> i32 {
  5 // having no semicolon makes this an expression
} // returns 5
```

> This is all too magic for me 😕. I prefer things to be more explicit. Lines of different code having different meanings depending on whether they have a semicolon seems like a terrible idea. Also I've never liked having implicit returns, just makes the code hard to read in my opinion.

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
