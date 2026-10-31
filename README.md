# orin-cam

A proof of concept that pushes images from a Jetson Orin Nano to a web browser. A command-line tool sends a PNG to a server on the Orin, and the server pushes it over a WebSocket to every open viewer page. The browser side is written in Rust and compiled to WebAssembly.

```text
image-push  --HTTP POST /image-->  webserver  --WebSocket /ws-->  viewer (Rust WASM in the browser)
```

## Crates

| Crate | What it does |
|---|---|
| `crates/image-push` | Command-line tool that sends a PNG to the server |
| `crates/webserver` | Receives images, pushes them to viewers and serves the viewer page |
| `crates/viewer` | Runs in the browser, opens the WebSocket and shows each image |

## Server routes

The server listens on port `8080`. On the Orin, that's `http://192.168.0.215:8080`.

| Route | Method | Purpose |
|---|---|---|
| `/` | GET | Hello page |
| `/viewer/` | GET | Viewer page |
| `/ws` | GET | WebSocket that pushes images to viewers |
| `/image` | POST | Accepts a new PNG, up to 16 MB |

## Prerequisites

### On the development PC

Rust, installed with [rustup](https://rustup.rs), plus the WebAssembly target and `trunk`, which builds the viewer:

```bash
rustup target add wasm32-unknown-unknown
```

```bash
cargo install trunk --locked
```

### On the Orin

Rust, installed with rustup, and a C compiler. The Orin doesn't need `trunk` or the WebAssembly target, because the viewer is built on the PC.

```bash
sudo apt install -y build-essential
```

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Deployment

Run every step, in this order, each time you deploy.

### On the PC

**1. Check that everything compiles.**

```bash
cd /home/tony/dev/orin-cam && cargo clippy --workspace --all-targets
```

**2. Build the viewer.** This writes the page and WebAssembly files to `crates/viewer/dist`.

```bash
cd /home/tony/dev/orin-cam/crates/viewer && trunk build --release
```

### On the Orin

**3. Stop the running server.** Press `Ctrl+C` in the terminal where it's running. Skip this step if it isn't running.

### On the PC

**4. Copy the project to the Orin.** This sends only what changed and deletes nothing on the Orin.

```bash
rsync -av --exclude target --exclude .git --exclude .idea /home/tony/dev/orin-cam/ 192.168.0.215:orin-cam/
```

### On the Orin

**5. Build the server and the command-line tool.**

```bash
cd ~/orin-cam && cargo build --release -p webserver -p image-push
```

**6. Start the server.** Run it from `~/orin-cam`, because the path to the viewer files is relative. Leave it running. It stops if this SSH session ends.

```bash
./target/release/webserver
```

You should see `Server started at http://0.0.0.0:8080`.

### In the browser

**7. Open or refresh the viewer** at <http://192.168.0.215:8080/viewer/>.

Refresh is required if the page was already open, because the viewer doesn't reconnect after a server restart. The page shows "waiting for image" until the first push.

### Check the deployment

**8. Confirm the server is live.** From the PC:

```bash
curl -s -o /dev/null -w '%{http_code}\n' http://192.168.0.215:8080/image
```

`405` means the server is running and has the `/image` route, which only accepts POST. `404` means the old server is still running.

## Copy the assets to the Orin

This copies the contents of `assets/` into `~/Downloads` on the Orin. It overwrites files with the same name if they differ, and deletes nothing.

```bash
rsync -av /home/tony/dev/orin-cam/assets/ tony@192.168.0.215:Downloads/
```

## Switch images

Each command replaces the image in every open viewer, and that image stays until the next push. `assets.xcf` is a source file, not a PNG, so `image-push` refuses it.

### From the Orin

The server is using its own terminal, so run these in a second SSH session. They need steps 5 and 6 of the deployment, and the copy above.

```bash
~/orin-cam/target/release/image-push ~/Downloads/blue.png
```

```bash
~/orin-cam/target/release/image-push ~/Downloads/cyan.png
```

```bash
~/orin-cam/target/release/image-push ~/Downloads/green.png
```

```bash
~/orin-cam/target/release/image-push ~/Downloads/magenta.png
```

```bash
~/orin-cam/target/release/image-push ~/Downloads/red.png
```

```bash
~/orin-cam/target/release/image-push ~/Downloads/yellow.png
```

### From the PC

Run these from `/home/tony/dev/orin-cam`.

```bash
cargo run -p image-push -- assets/blue.png
```

```bash
cargo run -p image-push -- assets/cyan.png
```

```bash
cargo run -p image-push -- assets/green.png
```

```bash
cargo run -p image-push -- assets/magenta.png
```

```bash
cargo run -p image-push -- assets/red.png
```

```bash
cargo run -p image-push -- assets/yellow.png
```

### Use a different server

`image-push` sends to `http://192.168.0.215:8080` by default. To send somewhere else, add `--server`:

```bash
cargo run -p image-push -- --server http://localhost:8080 assets/blue.png
```

## Troubleshooting

| What you see | What it means | What to do |
|---|---|---|
| `Address already in use` when starting the server | A server is already running | Stop it with `Ctrl+C`, then start it again |
| `http status: 404` from `image-push` | The old server is still running | Redo deployment steps 3 to 6 |
| `http status: 413` from `image-push` | The PNG is over 16 MB | Use a smaller file |
| `Connection refused` from `image-push` | The server isn't running | Redo deployment step 6 |
| `[Not A PNG]` from `image-push` | The file doesn't start with a PNG signature | Use a PNG file |
| `/viewer/` returns 404 | The viewer wasn't built, or the server was started outside `~/orin-cam` | Redo deployment step 2, 4 or 6 |
| The page stays on "waiting for image" after a push | The page has a dead connection | Refresh the page |
| `[WebSocket Error]: ... Connection reset without closing handshake` in the server log | A client disconnected abruptly | Nothing, the server keeps running |

## Known limitations

- **No security:** anyone on the network can view the page or replace the image. There's no login, and traffic isn't encrypted.
- **Images aren't saved:** the latest image is only held in memory, so viewers are blank after a server restart until the next push.
- **No reconnect:** a viewer has to be refreshed after a server restart.
- **Server doesn't run in the background:** it stops when its SSH session ends and doesn't start on boot.
- **PNG check is basic:** both `image-push` and the server only check the first 8 bytes of the file.
- **Duplicated values:** `WS_ROUTE`, `IMAGE_ROUTE` and `PNG_SIGNATURE` are defined in more than one crate and must be kept in sync by hand. `VIEWER_ROUTE` must also match `public_url` in `crates/viewer/Trunk.toml`.
