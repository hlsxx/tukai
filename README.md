<div align="center">
  <h3 align="center">Tukai</h3>

  <p align="center">
    Terminal based touch typing application
  </p>
</div>

<div align="center">
  <img src="https://github.com/hlsxx/tukai/blob/master/blob/example.gif" alt="Tukai" style="width:100%; max-height:400px" />
</div>

</br>

A terminal-based touch typing application built in Rust using the [Ratatui](https://github.com/ratatui/ratatui) library. The app provides an interactive typing experience with switchable templates, designed to help users improve their typing speed and accuracy.

### Features
- **Terminal-Based**: Simple, lightweight, and runs in the terminal for easy accessibility on any platform
- **Rust-Based**: Built with Rust for performance and reliability
- **Switchable Templates**: Choose between different templates to customize the typing practice experience
- **Stats preview**: A built-in stats preview of already done tests
- **Multi-Language Support**: Supports multiple languages for typing practice

## Getting Started

To get a local copy up and running follow these simple example steps.

### Arch Linux

tukai is available in the Arch Linux [extra repository](https://archlinux.org/packages/extra/x86_64/tukai/).

```sh
pacman -S tukai
```

### Installation from the Cargo
```sh
cargo install tukai

```
### Installation

#### Step 1. Install Rust
If you don't have Rust installed, follow the instructions on the official [Rust website](https://www.rust-lang.org/tools/install) to install Rust.

#### Step 2: Clone the repository

Clone this repository to your local machine:

```sh
git clone https://github.com/hlsxx/tukai
cd tukai
```
#### Step 3: Run or build the application
```sh
cargo run
cargo build --release
```

<!-- USAGE EXAMPLES -->
## Usage / Shortcuts

|  Key |  Alternate Key |  Action |
| ------------ | ------------ | ------------ |
|  ctrl-c  | esc  |  Exit the application |
|   ctrl-r |   |  Restart and generate a new text |
|   ctrl-s |   | Switch between templates  |
|   ctrl-t |   | Switch transparent background  |
|   ctrl-h  | ←  | Switch to the typing screen   |
|   ctrl-l  | → | Switch to the stats screen   |
|   ctrl-p |   | Switch the language |

<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE` for more information.

