# microbit-v2-project-template

To use this [Github template
repository](https://docs.github.com/en/repositories/creating-and-managing-repositories/creating-a-repository-from-a-template):

1. Follow the instructions linked above to make a Github repo
   for your project.
   
2. If needed, do the following to set up your tools:


```rust
  - rustup target add thumbv7em-none-eabihf
```
```rust
  - rustup component add llvm-tools
```
```rust
  - cargo install cargo-binutils
```
```rust
  - cargo install probe-rs --features cli
```


3. Build binary -

```rust
  - cargo build
```

4. Flash binary on micro:bit v2 -

```rust
  - cargo embed
```
