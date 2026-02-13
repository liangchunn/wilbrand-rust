# wilbrand-rust

Rust (+WebAssembly) port of [Wilbrand](https://github.com/giantpune/mailboxbomb), a program used to build the mailbox bomb exploit for the Wii system menu.

#### Why does this exist?

I noticed that the [web version of Wilbrand](https://wilbrand.donut.eu.org/) runs on PHP, and executes the binary to generate the exploit.

I thought it would be fun to port the C++ code to Rust, then compile it into WebAssembly, which makes it possible to generate the exploit payload from your browser; no servers needed.

### Running the CLI

Requirements:

- [rustup](https://rustup.rs/)

```sh
# show cli usage help
cargo run -- --help

# example
# cargo run <MAC_ADDRESS> <DATE> <SYS_VERSION> <OUT_DIR>
cargo run aa-bb-cc-dd-ee-ff 02-05-2022 4.3e /path/to/sdcard
```

### Development

Requirements:

- [rustup](https://rustup.rs/)
- [volta](https://volta.sh/)
- [wasm-pack](https://drager.github.io/wasm-pack/)
- [just](https://github.com/casey/just) (optional)

> volta and wasm-pack are required to build the UI

```sh
# running the CLI
cargo run

# running the UI
just dev

# or alternatively
cd wasm && wasm-pack build
cd ui && npm install && npm run dev
```

### Project Structure

- `cli`: cli runner (WIP)
- `lib`: core library to construct the cdb payload
- `wasm`: WebAssembly bindings
- `ui`: UI code for the website
- `data`: `loader.bin` and `envelope.bin`
- `docs`: documentation for misc stuff, like `envelope.bin`

### TODOs

- [x] Test on 4.3E, bit identical binary produced, runs on my RVL-001 and boots HackMii installer
- [x] Implement `clap` args parser in `cli`
- [x] Binary diff all supported versions with original Wilbrand impl
- [ ] Check padding of `loader.bin` when being included in the exploit

### Disclaimer

I am not responsible for your console bricking, or any form of
software/hardware damage that may be caused by the software
provided. Use at your own risk!

### Credits

- giantpune - [mailboxbomb](https://github.com/giantpune/mailboxbomb) as the reference implementation for porting to Rust, as well the original person who discovered the exploit
- HackMii team - savezelda's loader.bin is used
- emilydaemon - [web version of Wilbrand](https://wilbrand.donut.eu.org/)
- leahanderson1 - for [extracting the image](https://github.com/giantpune/mailboxbomb/issues/1#issuecomment-3104792286) from `envelope.bin`
