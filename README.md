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

4.  enums_iter

> **_fn enums_iter(&self) -> Box<dyn Iterator<Item = (usize, &Self::Output)> + '_>
    where
        Self::Output: std::fmt::Debug;_**
>
> - returns a non-consuming, index-aware iterator over the collection — suitable for debugging or inspection thanks to the Debug bound.

```
let vec = vec!["apple", "banana", "cherry"];
for (i, item) in vec.enums_iter() {
    println!("{i}: {:?}", item); // i: "apple", etc.
}
```

> The method `enums_start_at` has a parameter namely: `Starter`. And it can be used in the method as `Starter::new()` or `Starter::default()` or `Starter(<any-positive-number>)` to `usize::MAX`. The index of the starts from the positive number.

## Other Traits and Methods included.

### Nums

> _Nums_ Traits, namely so, primarily gets the indices from a collection as returns it as a collection. It does it using two methods: `nums` and `nums_starting_at`.

1. `nums`

_**fn nums(&self) -> Vec<Self::Output>**_
>
`nums` returns a Vec of the `first elements` (i.e., the indices) from a collection of tuple-like items returned using `self.enums_iter()`. It maps over each (index, value) pair produced by the iterator and collects just the index (`.first()`) into a new vector. In effect, it extracts and returns all the positional indices of the original collection.

#### before nums
```
    // to get the indices
    let mut indices = vec![];
    let data = ["house", "bulb", "towel", "bath", "table"];
    for (i, value) in data.iter().enumerate() {
        indices.push(i)
    }
    println!("{:#?}", indices); // prints [0,1,2,3,4]
    // OR
    let one_lined_indices: Vec<u32> = data.iter().enumerate().map(|(i, _)| i as u32).collect();
    println!("{:?}", one_lined_indices);
```

#### but with nums
```
    // to get the indices
    // cleaner and better. Boilerplate code done away with.
    let one_lined_indices = data.nums();
    println!("{:?}", one_lined_indices);
```


2. `nums_starting_at`

_**fn nums_starting_at(&self, at: Starter) -> Vec<Self::Output>**_

>
`nums_starting_at` returns a Vec of indices for the elements in the collection, but instead of starting from 0, it begins from the custom index provided by the `Starter` value (at). It uses `enums_start_at(at)` to enumerate the collection starting at that index, then go over each (index, value) pair to collect just the index part into a new vector.

```
    let custom_indices = one_lined_indices.nums_starting_at(6.into());
    println!("{:?}", custom_indices); // prints [6, 7, 8, 9, 10]
```

Method *enums_start_at* couldn't make this possible, though it does something similar.


### Partition

_Partition_ trait provides two methods namely; `first()` and `second()`. These, as the name suggested provides the programmer with the first and second values.
Instead of using the `<dot><number>`, using partition could be clearer and better.


### Versioning
> The first published version `0.1.5`, was based on `rust edition 2024`. The second publish version i.e version `0.1.6` will be based on `rust edition 2021` to accormodate a much large group.

> Third published version `0.2.0`, implements enums_iter; which returns iterator and the usage is left to the developer.

> The Fourth published version `0.4.0`, implements and includes two traits namely `Nums` and `Partition`. The methods these two traits gives are described are above.
