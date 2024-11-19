<div align="center">
  <h3 align="center">Tukai</h3>

  <p align="center">
    ⌨ Touch typing terminal based application
  </p>
</div>

## About The Tukai

<div align="center">
  <img src="https://github.com/hlsxx/tukai/blob/master/blob/example.gif" alt="Tukai" style="width:100%; max-height:400px" />
</div>

</br>

A terminal-based touch typing application built in Rust using the [Ratatui](https://github.com/ratatui/ratatui) library. The app provides an interactive typing experience with switchable templates, designed to help users improve their typing speed and accuracy.

### Features
- **Terminal-Based**: Simple, lightweight, and runs in the terminal for easy accessibility on any platform
- **Rust-Based**: Built with Rust for performance and reliability
- **Touch Typing Practice**: Practice typing skills with real-time feedback on accuracy and speed
- **Switchable Templates**: Choose between different templates to customize the typing practice experience
- **Stats preview**: A built-in stats preview of already done tests



## Getting Started

To get a local copy up and running follow these simple example steps.

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

- **ESC**:  Exit the application
- **CTRL + R**:  Restart and generate a new text
- **CTRL + S**: Switch between the templates
- **CTRL + J (left arrow)**: Switch to the Typing screen
- **CTRL + L (right arrow)**: Switch to the Stats screen

<!-- ROADMAP -->
## Roadmap

- [x] Highlighted typed text
- [x] Highlighted mistakes
- [x] Calculate WPM and accuracy after the run
- [x] Display remaining time
- [x] Template switch
- [ ]  Selectable type test times
- [x]  Stats overview
- [ ]  Activity overview
- [ ]  Custom texts

<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE` for more information.

