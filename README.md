# Rust brainfuck

Tiny bad brainfuck interpreter written in rust

### How to use
Clone the repo and run/build using cargo, or get one of the releases and rename the executable to 'rustbrainfuck' (only if you are on windows)

###### *(Im too lazy to do cross compilation, so if youre on linux, macos (whatever) just build it from source using cargo)*

Once you've got an executable of it you can run it on any file, for example:

```shell
rustbrainfuck code.bf
```

Or just 

```shell
cargo run -- code.bf
```

## Notes
- The length of the tape (available data cells) is 30,000
- `+` and `-` use wrapping. Which means if you have 255 in a cell and you try to increment it again it will wrap over to 0. Same goes backwards, if you try to decrement 0 it will wrap to 255
- The cell pointer however doesnt wrap. If you try to set the pointer to something higher than 30,000 or lower than 0 the interpreter will crash
- This interpreter is not that good because i wrote it in a very short amount of time