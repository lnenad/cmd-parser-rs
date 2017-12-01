# Command line parser for rust (clpr)

A bare-bones rust command line parser.

## Dependencies

This project has no external dependencies

## Usage

Add the crate to your Cargo.toml file and you can use it to parse incoming arguments 
and flags.

```toml
cmdparser = "0.1.1"
```

And in the file that will consume the crate:

```rust
extern crate cmdparser;
```

Parse incoming arguments and flags by calling "parse()" and receiving a touple.

```rust
let (arguments, flags) = Parser::new().parse();
```

First element of the touple is a HashMap<String, String> that contains all of the 
key value arguments received from the command line. By default both "-" and "--" prefixes
are parsed and accepted.

If you want to limit the user to a certain prefix(es) use the ```strict_prefix(prefixes: Vec<String>)```
method.

```rust
let prefixes = vec!["~".to_owned()];
let (arguments, flags) = Parser::new().strict_prefix(prefixes).parse();
```
The given parser will only accept arguments prefixed with a tilda (~) character.

## Example
Running your program with the following arguments:

```bash
$ cargo run -- -i foo.jpg -o bar.txt -p -r 10 -t -z -l 100
OR
$ ./executable -i foo.jpg -o bar.txt -p -r 10 -t -z -l 100
```
Yields the following:

```js
ArgumentList: {"l": "100", "i": "foo.jpg", "o": "bar.txt", "r": "10"}
Flags: ["p", "t", "z"]
```

To consume them the simplest pattern would be:

```rust
let foo = arguments.get("foo").or(arguments.get("f")).expect("No foo argument provided");
```

The given code would look for -foo or -f and in case they're not found panic with the erro message.

## Contributing

As this is a very basic parser, and I don't have a lot of Rust experience, if you want to contribute and improve something, 
submit a pull request.

## Contributors

Nenad Lukic - [lnenad](http://github.com/lnenad)

## License

[WTFPL](http://www.wtfpl.net/)