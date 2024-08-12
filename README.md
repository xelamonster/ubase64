# ubase64

Lightweight, simple, and configurable base64 encode and decode.

⚠️ **WARNING: this is alpha-stage software and has not been extensively tested. Use at your own
risk.**

Contributions (tests and benchmarks especially) are very welcome!

## Overview

### Another one?

Yes, indeed. There's a wide array of base64 Rust implementations out there, some likely work
very well, but none of them are _perfect_. Neither is this I'm sure, as much as I tried to
address the issues I saw there's always room for improvement (PRs welcome! 😜), but it's done
quite nicely for my needs so far.

### Design goals

I had four main goals in designing `ubase64`:

1. **Intuitive**: The base64 specification and usages in the wild include a good deal more
   variation than one might expect. Different alphabet options, whether to use padding, line
   wrapping options, etc. Thing is, most users
   [neither know nor care about these options](https://github.com/marshallpierce/rust-base64/issues/233),
   and in my opinion forcing them to is bad design. They want the default, without arcane
   setups and invocations; and odds are the default will work just fine.
2. **Configurable**: Given the wide array of options as mentioned in the first point, the
   second goal of `ubase64` is to provide enough options to support as many potential base64
   use cases as possible while keeping the "basic" API clean and simple.
3. **Performance**: `ubase64` tries to strike a balance between the best possible performance
   and the smallest possible binary size.
4. **Versatility**: Finally, `ubase64` should work in a wide variety of contexts. In addition
   to library usage, it provides a command-line interface so you can benefit from fast
   encoding and decoding in the terminal too!

## Usage

You can use `ubase64` in your project as a library, or install it as a command-line binary.

### Library

Add `ubase64` to your project's dependencies:

```toml
[dependencies]
ubase64 = { version = "0.0.1", features = [] }
```

**Please note**: the package's default features include dependencies only needed for a binary
install. If you don't need these, ensure you have an empty dependency array as shown above;
you can install with `cargo add --no-default-features ubase64` to do this automatically.

Example library usage:

```rust
use ubase64::encode_std;

let encoded = encode_std(b"Hello, world!");
assert_eq!(encoded, "SGVsbG8sIHdvcmxkIQ==");
```

### Binary

Install `ubase64` as a binary with Cargo:

```sh
cargo install ubase64
```

Example binary usage:

```sh
$ ubase64 encode "Hello, world!"
SGVsbG8sIHdvcmxkIQ==
$ echo -n "Hello, world!" > test.txt
$ ubase64 encode -f test.txt
SGVsbG8sIHdvcmxkIQ==
```

## Architecture

### References

- [RFC 4648](https://datatracker.ietf.org/doc/html/rfc4648): Base64 data encoding standard.
  Describese the most common implementations of base64.
- [feross/base64-js](https://github.com/feross/base64-js): Base64 in pure JavaScript. Fast and
  clean; this was highly influential in the design.
- [uhmarcel/rbase64](https://github.com/uhmarcel/rbase64): A solid Rust implementation with a
  relatively nice API, but not actively maintained.

### Considerations

- **Minimal dependencies**: To keep builds small and performance predictable, `ubase64`
  installed as a library includes only two dependencies (`thiserror` and `miette`, both for
  error handling).
- **Clean API**: The standard wrappers `encode_std` and `decode_std` are provided to make the
  most common use case as painless as possible, and the more complex configurations should
  still be relatively intuitive.
- **Easy CLI**: Some popular implementations include no CLI, and many try to match the
  interface of the classic [GNU `base64` utility](https://linux.die.net/man/1/base64). I found
  that interface confusing (why does it encode by default and require a flag to decode?) and
  it's always a process for me to figure out how to give it a string input, so I took a more
  opinionated approach.

### Implementation

#### Alphabets

Two alphabet options are provided, standard (`STD_ALPHABET`) and URL-safe (`URL_ALPHABET`).
These are represented as 64-byte const arrays.

For decoding, the `base64_reverse_alphabet` function is used to create a reverse lookup table
from the alphabet array at compile time. This is implemented as a 256-byte const array which
maps the byte value of each character in the alphabet to its index in the array. Only 64 of
the 256 values will ever be filled, but this is an insignificant amount of memory and should
incur less overhead than an equivalent map type.

#### Data types

Decoded values are arbitrary data and thus are represented as `&[u8]` (as input) or `Vec<u8>`
(as output).

Encoded values are strings represented as `&str` (input) or `String` (output).
