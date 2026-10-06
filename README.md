# better-greet

`better-greet` is an ultra-lightweight, high-performance Wayland greeter written in **Rust** using **egui**, designed specifically for **greetd**.

It serves as a minimal, resource-friendly, and fully customizable display manager frontend for Wayland compositors such as Hyprland or Sway.

## Features

- **Blazing Fast:** Built with native Rust and `egui` for instant rendering and near-zero latency.
- **Ultra-Lightweight:** Consumes minimal RAM (~10-15 MB) during runtime.
- **IPC Integration:** Directly communicates with `greetd` via Unix sockets.
- **Modern UI:** Minimalist left-side login panel, live clock, and customized input fields.
- **Zero Legacy Dependencies:** No Qt or GTK runtime overhead.

## Requirements

- `greetd` daemon installed and running
- A Wayland compositor (e.g., Hyprland)

## Building & Usage

```bash
# Clone the repository
git clone [https://github.com/WastleQ/better-greet.git](https://github.com/WastleQ/better-greet.git)
cd better-greet

# Build release binary
cargo build --release
