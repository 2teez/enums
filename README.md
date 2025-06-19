# enums

## Name
`enums` A Python "kind" of enumerate function for vector and slice in rust. This rust trait, abstracts methods like _.iter().enumerate().collect()_ into a clean, reusable methods also called `enums`, `enums_mut` and `enum_start_at`. __Read all about it below__.

## Installation
#### Use rust cargo:
    cargo add enums

#### Use github link like so:
In the Cargo.toml file

    [dependancies]
    arraylist = {git = "https://github.com/2teez/enums.git"}

#### To Use in the src/main.rs file:
```
  use enums::enums::Enums;
```

## Description

`enums` - To do simple iterating on a vector or slice in rust to get *both* the _*index*_ and the _*value*_
can be annoyingly some boiler plate codes, especially if you have to write that repeatedly. Either using a for-loop or a function-like expression.
Like so:

```
  let ages = vec![13, 15, 45, 47];
  for (index, value) in ages.iter().enumerate() {
     ....
  }
  // OR
  println!("{:?}", [15, 47, 13, 45].iter().enumerate().map(|(i, v)|(i, v)).collect::<Vec<_>>());
  // OR
  // Someone could say take out the map, since you are *not doing* anything with it.
  // Yet it is still some boiler typing to do
  println!("{:?}", [15, 47, 13, 45].iter().enumerate().collect::<Vec<_>>());
```

All of these are abstracted away, making the programer focus on what matters which is the job of iterating without calling several functions to get the job done using simpler and easy to remember and clear function API.
Like so:

```
 let ages = vec![13, 15, 45, 47];
  for (index, value) in ages.enums() {
     ....
  }
  // OR
    println!("{:?}", [15, 47, 13, 45].enums());
```
The above is a cleaner and better expression for developers who find _.enumerate()_ verbose or annoying.
