# rust-sandbox
I'm currently learning Rust through
[exercism](https://exercism.org/tracks/rust). This is a project that serves as a
code sandbox while I do that. The disadvantage of an online environment like
Exercism is that it doesn't give you the ability to debug your submissions or
execute arbitrary code. Also, you can't inspect the output very much.

## Options
[in progress] The sandbox expects you to provide a selection, which runs one
of the various different things the project can do.

To run the sandbox:

```sh
cargo run
```

Options available (run with no params to see this list):
- `reverse`

  Reverses a provided string. For example:

  ```sh
  cargo run reverse 'Hello World.'
  .dlroW olleH
  ```

## Rust size performance considerations
In the `size-comparison` folder I was comparing output binary sizes
between C++ and Rust. The primary difference appears to be that the C++
standard library is dynamically linked by default, whereas Rust statically
compiles its standard library into the binary. Using `-C prefer-dynamic` leads
to a pretty comparable binary size.

To see this in action:
```sh
cd size-comparison
./build-and-compare
```
