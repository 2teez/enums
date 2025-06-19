# enums

## Name
`enums` A Python "kind" of enumerate function for vector and slice in rust. This rust trait, abstracts methods like _.iter().enumerate().collect()_ into a clean, reusable methods also called `enums`, `enums_mut` and `enum_start_at`. __Read all about it below__.

## Installation
#### Use rust cargo:
    cargo add enums

#### Use github link like so:
In the Cargo.toml file

    [dependancies]
    enums = {git = "https://github.com/2teez/enums"}

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
The above is cleaner and a better expression for developers who find _.enumerate()_ and other chains of functions verbose or annoying.

Further more, like enumerate function in python; you can change the starting index of the collection
#### In Python
```

    langs = ["java", "ocalm", "odin", "c++"]
    for i, lang in enumerate(langs, 2):
      print(i, lang)

  // prints
  2 java
  3 ocalm
  4 odin
  5 c++

```

#### In rust
```

    let langs = ["java", "ocalm", "odin", "c++"];
    for (i, lang) in langs.enums_start_at(2.into()) {
        println!("{}, {}", i, lang);
    }

  // prints
  2, java
  3, ocalm
  4, odin
  5, c++
```

In the enums_start_at method, a tuple struct is also provided that can be used as parameter for the method. It can be imported into scope like so
```
  use enums::enums::Starter
```
Then starter, methods namely: default, new and Starter(usize), where `usize` is any positive value from `0` to `usize::MAX`, can be used to customized the starting index. [See examples on API below]

## enums Methods

`enums` trait uses three methods namely: `enums`, `enums_mut` and `enums_start_at`. The trait also uses associated type.

These methods were implemented for vector and slice like so:

1.  enums

  > **_fn enums(&self) -> Vec<(usize, Self::Output)>_**
  >
  > - it iterates the collection on which is it called neatly without doing any chain linking.

  ```
    let langs = vec!["c", "c++", "zig-lang", "java", "rust"];
    for (ind, lang) in langs.enums() {
        print!("{:?} ", (ind, lang));
    }
  ```

2.  enums_mut

  > **_fn enums_mut(&mut self) -> Vec<(usize, &mut Self::Output)>_**
  >
  > - it both iterates and modifies the collection on which is it called clearly.

  ```
let mut langs = vec![
        String::from("c"),
        String::from("c++"),
        String::from("zig-lang"),
        String::from("java"),
        String::from("rust"),
    ];

    // Use enums_mut to get mutable indexed references
    for (_i, lang) in langs.enums_mut() {
        *lang = lang.to_uppercase();
    }

    println!("{:?}", langs); // prints ["C", "C++", "ZIG-LANG", "JAVA", "RUST"]

  ```

  3. enums_start_at

  > **_fn enums_start_at(&self, at: Starter) -> Vec<(usize, Self::Output)>_**
  >
  > - This method iterates, but can change the index start of a collection. That is, your
      collection doesn't have to start from zero with the method `enums_start_at`.

```

    let langs = ["java", "ocalm", "odin", "c++"];
    for (i, lang) in langs.enums_start_at(2.into()) {
        println!("{}, {}", i, lang);
    }

  // prints
  2, java
  3, ocalm
  4, odin
  5, c++

```

> The method `enums_start_at` has a parameter namely: `Starter`. And it can be used in the method as `Starter::new()` or `Starter::default()` or `Starter(<any-positive-number>)` to `usize::MAX`. The index of the starts from the positive number.

### Versioning
> The first publish version `0.1.5`, was based on `rust edition 2024`. The second publish version i.e version `0.1.6` will be based on `rust edition 2021` to accormodate a much large group.
