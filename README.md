# egui-overlay

Work in progress and proof of concept external [egui](https://crates.io/crates/egui) overlay for windows.

## Usage

Get the target window handle and supply it via the target argument.

```
./egui-overlay -t 0x27071A
```

## TODO

- Pass through input to underlying window
- Toggle overlay via hotkey
- Hide terminal window