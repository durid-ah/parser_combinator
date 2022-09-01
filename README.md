# Parser Combinator

An attempt at a Rust implementation of the typescript [Arcsecond](https://github.com/francisrstokes/arcsecond#readme) library.

After following [LowLevelJavascript](https://www.youtube.com/c/LowLevelJavaScript/featured)'s tutorial on parser combinator, I thought it would be a fun exercise to try and rewrite it using Rust.

# Project contents

The project is split up into 4 main parts:

## Models

Contains all the enums,structs and traits used by the parsers.

## Parssers

* **Str**: Match an arbitrary string to the target

```rust
   let parser = Str::new("Test".to_owned());
   let res = parser.run("Test");
   assert!(res.result.unwrap().is_ok());
   assert_eq!(res.index, 4);
```

* **Digits**: Match at least one digit in a string

```rust
   let p = Digits::new();
   let res = p.run("123s");
   assert!(res.result.unwrap().unwrap().unwrap_one() == "123");
   assert!(res.index == 3);
```

* **Letters**: Match at least one letter in a string

```rust
   let p = Letters::new();
   let res = p.run("abcd1s".to_owned());
   assert!(res.result.unwrap().unwrap().unwrap_one() == "abcd");
   assert!(res.index == 4);
```

* **Between**: Match a parser between two other parsers
```rust
   let left = Box::new(Str::new("(".to_owned()));
   let value = Box::new(Str::new("test".to_owned()));
   let right = Box::new(Str::new(")".to_owned()));

   let bet = Between::new(left, right, value);
   let result = bet.run("(test)");

   assert_eq!(result.result.unwrap().unwrap().unwrap_many().len(), 1);
   assert_eq!(result.index, 6);
```

## Collection Parsers:

Parsers used to handle a collection of parsers
* **SequenceOf**: Takes in a sequence of parsers and ensures that they are executed in the right order

```rust
```
* **Choice**: Finds the first matching parser in a collection of parsers

```rust
```

* **Many/ManyOne**: Used to find as many instances of the parser in sequence. ManyOne ensures that at least one parser is successful while Many returns successful with zero instances.

```rust
```

* **SepBy/SepByOne**: Takes in a separator and a separated parser and looks for multiple instances of the separated value with separator in between each. Just like **ManyOne**, **SepByOne** is used to catch at least one separated value while **SepBy** only has one.

```rust
```

## Bit Parsers:

Coming Soon...