# RIP-8

My implementation of a Chip-8 interpreter written in Rust with an egui interface for debugging and rendering.

To run it, you can use cargo run like: `cargo run roms/<rom>` or you can `cargo build --release` to get a full binary and run it like `./target/release/rip-8 roms/<rom>`.

It's only tested to work on Linux and Apple Silicon MacOS systems but I'm working on a Windows implementation if any changes are needed, ~~I have no Mac to test a mac version but feel free to open up an issue if you try using this (for whatever reason) and it doesn't work.~~ I now have an Apple silicon Mac that I've tested it on! A few changes were required.

Development from hereon out will continue on MacOS with Apple silicon. I will do my best to check Linux versions of this as well and I still have plans to get it working with Windows.

Thank you for taking a look at the first of hopefully many projects of mine in this space.

Oh it's also tailored to 4k screens change the DPI scaling const from 2 to 1.

## Working on:
- [ ] refactor the window and canvas rendering methods
    - [x] fix MacOS flickering bugs
    - [ ] fix MacOS framerate calculation
- [ ] get display waiting working for true accuracy
- [x] fix the fps inconsistency issues
- [ ] more advanced cli arguments for debug mode
- [ ] making it make noise
- [ ] dpi scaling but like fr

