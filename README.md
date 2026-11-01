
```markdown
# NumLite

**NumLite** is a lightweight, free, and open-source alternative to MATLAB, built entirely in the **Rust** programming language. It is designed for students, researchers, and anyone who needs a fast numerical computing and signal processing tool without having to install a massive 30 GB application.

With a binary size of under 10 MB, NumLite offers a modern, fast, and responsive GUI, and supports executing `.m` scripts just like MATLAB.

## ✨ Key Features

### Core Mathematics & Data Types
- **Matrix & Vector Operations:** Supports matrix literals `[1, 2; 3, 4]`, matrix multiplication, transpose, and element-wise operations (`.*`, `./`).
- **Advanced Linear Algebra:** Includes `inv()`, `det()`, `trace()`, `rank()`, and eigenvalue estimation (`eig()`).
- **Complex Numbers:** Full support for complex arithmetic (`complex`, `real`, `imag`, `conj`, `abs`, `angle`).
- **String Manipulation:** Convert and format text with `num2str()`, `str2num()`, `strcat()`, `fprintf()`, and `sprintf()`.

### Signal Processing & Statistics
- **Signal Processing:** Built-in functions such as `conv()` (convolution), `fft()` (Fast Fourier Transform), `ifft()`, `filter()` (FIR), and `xcorr()`.
- **Statistics:** Calculate `mean()`, `sum()`, `min()`, `max()`, and standard deviation (`std()`).
- **Generators:** Create sequences easily with `linspace()` and `cumsum()`.

### Control Flow & Scripting
- **Control Flow:** Supports loops (`for`, `while`), conditionals (`if/else`), `switch/case`, and logical operators (`&&`, `||`, `~`).
- **Custom Functions:** Define your own functions directly in the script (`function y = f(x)`).
- **Array Slicing & Indexing:** Extract subsets of arrays using standard MATLAB syntax (e.g., `x(1:5)` or `x(end)`).
- **Range Operator:** Generate sequences with `0:0.1:5`.

### Interactive GUI & Visualization
- **Built-in Editor & Console:** A sleek Dark Mode editor with multi-tab support and a dedicated output console.
- **2D Plotting:** Plot data with `plot(x, y)` or auto-generate X-axes with `plot(y)`.
- **Stem Plots:** Visualize discrete data with `stem()`.
- **Graph Customization:** Overlay multiple graphs with `hold on/off`, and add `title`, `xlabel`, and `ylabel` (with vertical text rotation support).
- **Pop-Out Window:** Click the magnifying glass icon in the Figure panel to enlarge graphs for better inspection.
- **Dual-Mode:** Run as a *Desktop* application (GUI) or in the Terminal (CLI).

## 🚀 How to Run

### Pre-built Binaries (Recommended)
Download the latest `.exe` (for Windows) or Linux binary from the **[Releases page](https://github.com/gilangrahmanda212-lang/Numlite/releases)**.
- **Windows:** Double-click `numlite.exe`.
- **Linux:** Open terminal, run `chmod +x numlite`, then `./numlite`.

### Build from Source
Make sure you have [Rust installed](https://rustup.rs/). 
```bash
git clone https://github.com/gilangrahmanda212-lang/Numlite.git
cd Numlite

# Run GUI Mode
cargo run

# Run Terminal Mode (CLI)
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
t = 0:0.1:5;

% Create a sine wave signal
original_signal = sin(2 * 3.14159 * 2 * t);

% Add noise
noise = rand(1, length(t)) * 0.5 - 0.25;
noisy_signal = original_signal + noise;

% Filtering using convolution
filter = [0.2, 0.2, 0.2, 0.2, 0.2];
smooth_signal = conv(noisy_signal, filter);

% Display graphs
title('Signal Processing Simulation')
xlabel('Time (s)')
ylabel('Amplitude')

plot(t, noisy_signal)

hold on
stem(t, smooth_signal(1:length(t)))
hold off
```

## 🛠️ Built With
- **Language:** Rust 🦀
- **GUI Framework:** `eframe` / `egui`
- **Terminal CLI:** `crossterm`
- **File Dialog:** `rfd`
- **Randomness:** `rand`
- **Windows Resources:** `winres`

## 🗺️ Roadmap
- [x] Milestone 1: Matrices & Basic Arithmetic
- [x] Milestone 2: Convolution, FFT, & Math Functions
- [x] Milestone 3: Control Flow (For, If, Else, While, Switch)
- [x] Milestone 4: Range Operator, Slicing, & .m Script Execution
- [x] Milestone 5-7: Full GUI (Editor, Console, Interactive Graphs)
- [x] Milestone 8: Advanced Visualization (Stem, Hold On/Off, Labels, Vertical Text)
- [x] Milestone 9: String Manipulation (fprintf, sprintf, num2str, str2num)
- [x] Milestone 10: GitHub Actions CI/CD & Binary Distribution
- [ ] Milestone 11: Struct and Cell Array Data Types
- [ ] Milestone 12: Legend and Bar Charts

## 📚 Developer Documentation
Interested in looking under the hood of NumLite or contributing?
Read [ARCHITECTURE.md](ARCHITECTURE.md) to understand how the Lexer, Parser, and Evaluator work.

## 👨‍💻 Author
Created and developed by **Gilang Hukma Rahmanda & Bertha Adela**.

## 📜 License
This project is licensed under the MIT License.
```
