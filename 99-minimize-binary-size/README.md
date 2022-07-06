# minimize-binary-size

```
cargo build --release
```

## 2022-07-05

```sh
# without [profile.release] directives
> du -sh target/release/minimize-binary-size
440K	target/release/minimize-binary-size
# with [profile.release] directives
> du -sh target/release/minimize-binary-size
244K	target/release/minimize-binary-size
```
