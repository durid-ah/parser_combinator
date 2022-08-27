# Parser Combinator

An attempt at a Rust implementation of the typescript [Arcsecond](https://github.com/francisrstokes/arcsecond#readme) library.

After following [LowLevelJavascript](https://www.youtube.com/c/LowLevelJavaScript/featured)'s tutorial on parser combinator, I thought it would be a fun exercise to try and rewrite it using Rust.

# Project contents

The project is split up into 4 main parts:

## Models

Contains all the enums,structs and traits used by the parsers.

## Parssers

* **Str**: Match an arbitrary string to the target
* **Digits**: Match at least one digit in a string
* **Letters**: Match at least one letter in a string
* **Between**: Match a parser between two other parsers

## Collection Parsers:

Parsers used to handle a collection of parsers
* **SequenceOf**: Takes in a sequence of parsers and ensures that they are executed in the right order

* **Choice**: Finds the first matching parser in a collection of parsers

* **Many/ManyOne**: Used to find as many instances of the parser in sequence. ManyOne ensures that at least one parser is successful while Many returns successful with zero instances.

* **SepBy/SepByOne**: Takes in a separator and a separated parser and looks for multiple instances of the separated value with separator in between each. Just like **ManyOne**, **SepByOne** is used to catch at least one separated value while **SepBy** only has one.

## Bit Parsers:

Coming Soon...