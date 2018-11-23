# conmux

conmux is a Console multiplexer for Windows using the new Window 10 ConPTY APIs.

- Written in Rust
- Zoomable Panes
- Tabs
- tmux-like default bindings

While conmux supplies tmux-like functionality, much like ConPTY itself is not a Unix PTY, *conmux is not tmux*. There is no session support, and conmux does not use a compatible configuration file.

Requirements
- Windows 10 1809 or Higher
## Todo
The goal so far is to support buffered multiplexing. An event loop has been implemented to handle multiple PTYs at once, but still only runs one PTY at a time.

## Usage

`ConPty::new` spawns a pseudoconsole and two pipe ends that can read and write from the console buffer. These pipes are synchronous but are thread safe to read and write from, and are backed by `std::fs::File` instances. 

```rust
// Spawn the pseudoconsole
let mut pty = ConPty::new((120, 30), "pwsh", Some(&PathBuf::from("C:\\"))).unwrap();

// start the shell
pty.start_shell().unwrap();

```

See `main.rs` for more details.

## Thanks to
  * Alacritty