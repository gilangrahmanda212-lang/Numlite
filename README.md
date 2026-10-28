```markdown
# NumLite

**NumLite** is a lightweight, free, and open-source alternative to MATLAB, built entirely in the **Rust** programming language. It is designed for students, researchers, and anyone who needs a fast numerical computing and signal processing tool without having to install a massive 30 GB application.

With a binary size of under 10 MB, NumLite offers a modern, fast, and responsive GUI, and supports executing `.m` scripts just like MATLAB.

## ✨ Key Features

- **Matrix & Vector Operations:** Supports matrix literals `[1, 2; 3, 4]`, matrix multiplication, transpose, and element-wise operations (`.*`, `./`).
- **Signal Processing:** Comes with built-in functions such as `sin()`, `cos()`, `conv()` (convolution), `fft()` (Fast Fourier Transform), and `cumsum()`.
- **Control Flow:** Supports loops (`for`), conditionals (`if/else`), and the range operator (`0:0.1:5`).
- **Array Slicing & Indexing:** Extract subsets of arrays using standard MATLAB syntax (e.g., `x(1:5)`).
- **Interactive GUI & Plotting:** 
  - Built-in code Editor and Console Output.
  - Interactive Figure panel with Axes, Ticks, and Grids.
  - *Pop-Out Window* feature (click the magnifying glass icon) to enlarge graphs.
- **Dual-Mode:** Can be run as a *Desktop* application (GUI) or in the Terminal (CLI).

## 🚀 How to Run

### GUI Mode (Desktop Application)
Open a terminal, navigate to the project folder, and run:
```
cargo run
```

### Terminal Mode (CLI)
If you want to run NumLite on a server or a terminal without a GUI:

```bash
cargo run -- --cli
```

### Running `.m` Scripts
Inside the GUI Editor or the Terminal REPL, you can execute a script file:

```text
>> run('script.m')
```

## 📝 Usage Example

Here is an example of a simple signal processing simulation that you can copy-paste into the NumLite Editor:

```matlab
% Create a time vector
t = 0:0.1:5

% Create a sine wave signal
original_signal = sin(2 * 3.14159 * 2 * t)

% Add noise
noise = rand(1, length(t)) * 0.5 - 0.25
noisy_signal = original_signal + noise

% Filtering using convolution
filter = [0.2, 0.2, 0.2, 0.2, 0.2]
smooth_signal = conv(noisy_signal, filter)

% Display graphs
subplot(2,1,1)
plot(t, noisy_signal)
title('Original Signal + Noise')

subplot(2,1,2)
new_t = 1:length(smooth_signal)
plot(new_t, smooth_signal)
title('Filtered Signal (Convolution)')
```

## 🛠️ Built With

- **Language:** Rust
- **GUI Framework:** egui / eframe
- **Terminal CLI:** crossterm
- **Others:** rand

## 🗺️ Roadmap

- [x] Milestone 1: Matrices & Basic Arithmetic
- [x] Milestone 2: Convolution, FFT, & Math Functions
- [x] Milestone 3: Control Flow (For, If, Else)
- [x] Milestone 4: Range Operator, Slicing, & .m Script Execution
- [x] Milestone 5-7: Full GUI (Editor, Console, Interactive Graphs)
- [ ] Build Release & Binary Distribution
- [ ] Add advanced signal toolbox functions (Filter design, xcorr)

## 📚 Developer Documentation

Interested in looking under the hood of NumLite or contributing?
Read [ARCHITECTURE.md](ARCHITECTURE.md) to understand how the Lexer, Parser, and Evaluator work.

## 👨‍💻 Author

Created and developed by **Gilang Hukma Rahmanda & Bertha Adela**.

## 📜 License

This project is licensed under the MIT License.
