# Command line parser for rust (clpr)

A bare-bones rust command line parser.

## Dependencies

This project has no external dependencies

## What's new

The most recent version (0.2.0) contains the following:

* The return value is now a Vector
* Under the hood improvements

## Usage

Add the crate to your Cargo.toml file and you can use it to parse incoming arguments 
and flags.

```toml
cmdparser = "0.2.0"
```

And in the file that will consume the crate:

```rust
extern crate cmdparser;
```

Parse incoming arguments and flags by calling "parse()" and receiving a touple.

```rust
let (arguments, flags) = Parser::new().parse();
```

First element of the touple is a HashMap<String, Vec<String>> that contains all of the 
key value arguments received from the command line. By default, "-" and "--" prefixes
are parsed and accepted.

If the user supplies multiple arguments under different positions they will be 
overwritten by default. To disable this behavior use

```rust
merge_values(bool)
```

When merge_values is set to true under a parser instance, and the user passes the 
following arguments

```bash
$ ./executable -i foo.jpg -i bar.jpg
```
The parser will merge "foo.jpg" and "bar.jpg" values under the "i" argument.
If merge_values is set to false (which is the default behavior) the "i" argument
will hold a single value of "bar.jpg".

If you want to limit the user to a certain prefix(es) use the ```strict_prefix(prefixes: Vec<String>)```
method.

```rust
let prefixes = vec!["~".to_owned()];
let (arguments, flags) = Parser::new().strict_prefix(prefixes).parse();
```
The created parser will only accept arguments prefixed with a tilda (~) character.

## Example
Running your program with the following arguments:


```bash
$ cargo run -- -i foo.jpg bar.png -zero true -flag -param 100 -o
OR
$ ./executable -i foo.jpg -o bar.txt -p -r 10 -t -z -l 100
```
Yields the following:

```js
ArgumentList: {"param": ["100"], "zero": ["true"], "i": ["foo.jpg", "bar.png"]}
Flags: ["flag", "o"]
```

To consume them the simplest patterns would be, for an argument:

```rust
let images: Vec<String>= arguments.get("i").or(arguments.get("images")).expect("No image argument provided");
```

For a flag:
```rust
if flags.contains(&"opaque".to_owned()) || flags.contains(&"o".to_owned()) {
    action();
};
```

The given code would look for -images or -i and in case they're not found panic with the erro message.

## Comparison to clap-rs

While clap is a robust, feature rich and tested package, it has 886 reverse dependencies and it's simply an
overkill for most usages. 

## Performance

The parser will parse 100 flags and 100 single value arguments in ~1ms on a 4c4t i5.

## Contributing

As this is a very basic parser, and I don't have a lot of Rust experience, if you want to contribute and improve something, 
submit a pull request.

## Contributors

Nenad Lukic - [lnenad](http://github.com/lnenad)

## License

[WTFPL](http://www.wtfpl.net/)