```markdown
# NumLite

**NumLite** is a lightweight, free, and open-source alternative to MATLAB, built entirely in the **Rust** programming language. It is designed for students, researchers, and anyone who needs a fast numerical computing and signal processing tool without having to install a massive 30 GB application.

With a binary size of under 10 MB, NumLite offers a modern, fast, and responsive GUI, and supports executing `.m` scripts just like MATLAB.

## ✨ Key Features

### Core Mathematics & Data Types
- **Matrix & Vector Operations:** Supports matrix literals `[1, 2; 3, 4]`, matrix multiplication, transpose, and element-wise operations (`.*`, `./`).
- **Advanced Linear Algebra:** Includes `inv()`, `det()`, `trace()`, `rank()`, and eigenvalue estimation (`eig()`).
- **Complex Numbers:** Full support for complex arithmetic (`complex`, `real`, `imag`, `conj`, `abs`, `angle`).
- **Advanced Data Types:** Full support for **`struct`** (including nested structs like `s.a.b`) and **`cell array`** `{1, 'text', [1,2]}`.
- **String Manipulation:** Convert and format text with `num2str()`, `str2num()`, `strcat()`, `fprintf()`, and `sprintf()`.

### Signal Processing & Statistics
- **Signal Processing:** Built-in functions such as `conv()` (convolution), `fft()` (Fast Fourier Transform), `ifft()`, `filter()` (FIR), and `xcorr()`.
- **Statistics:** Calculate `mean()`, `sum()`, `min()`, `max()`, and standard deviation (`std()`).
- **Generators:** Create sequences easily with `linspace()` and `cumsum()`.

### Control Flow & Scripting
- **Control Flow:** Supports loops (`for`, `while`), conditionals (`if/else`), `switch/case`, and logical operators (`&&`, `||`, `~`).
- **Custom Functions:** Define your own functions directly in the script (`function y = f(x)`).
- **Array Slicing & Indexing:** Extract subsets of arrays using standard MATLAB syntax (e.g., `x(1:5)` or `x(3:end)`).
- **Range Operator:** Generate sequences with `0:0.1:5`.
- **Line-Numbered Error Handling:** Advanced debugging with precise error messages indicating the exact line number where a syntax or runtime error occurred (e.g., `Error on line 7: Variable 'x' is undefined`).

### Interactive GUI & Visualization
- **Built-in Editor & Console:** A sleek Dark Mode editor with multi-tab support and a dedicated output console.
- **Floating Console Drawer:** The Console panel is resizable and can be pulled up/down without covering the Editor tabs. The header remains statically visible at the top.
- **2D Plotting:** Plot data with `plot(x, y)` or auto-generate X-axes with `plot(y)`.
- **Stem Plots:** Visualize discrete data with `stem()`.
- **Graph Customization:** Overlay multiple graphs with `hold on/off`, and add `title`, `xlabel`, and `ylabel` (with vertical text rotation support).
- **Plot Zoom (Magnifier):** Click the magnifying glass icon (🔍) in the Figure panel to open a larger pop-up window for detailed graph inspection.
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

### 🏗️ Phase 1: Foundation & Core Language (Completed ✅)
- [x] Milestone 1: Matrices, Basic Arithmetic, & Linear Algebra
- [x] Milestone 2: Convolution, FFT, & Math Functions
- [x] Milestone 3: Control Flow (For, If, Else, While, Switch)
- [x] Milestone 4: Range Operator, Slicing, & .m Script Execution
- [x] Milestone 5: Full GUI (Editor, Console, Multi-tab)
- [x] Milestone 6: Advanced Visualization (Stem, Hold On/Off, Axis Labels)
- [x] Milestone 7: String Manipulation (fprintf, sprintf, num2str, str2num)
- [x] Milestone 8: Advanced Data Types (Struct & Cell Array)
- [x] Milestone 9: Line-Numbered Error Handling & Code Refactoring
- [x] Milestone 10: CI/CD GitHub Actions & Cross-Platform Distribution

### 📊 Phase 2: Advanced Visualization & GUI (In Progress 🚧)
- [ ] Milestone 11: Plot Customization (Colors, `legend()`, `bar()`, `scatter()`)
- [ ] Milestone 12: GUI Enhancements (Syntax Highlighting, Keyboard Shortcuts)
- [ ] Milestone 13: Multiple Figures & Subplots (`figure(1)`, `subplot(2,1,1)`)

### ⚡ Phase 3: Performance & Signal Analysis
- [ ] Milestone 14: Core Optimization (Use `nalgebra` for matrix math, multithreading)
- [ ] Milestone 15: Discrete System Analysis (Pole-Zero, `freqz()`, IIR Filters)
- [ ] Milestone 16: Debugging Tools (`tic()`, `toc()`, step-by-step execution)

### 🌍 Phase 4: Ecosystem & Distribution
- [ ] Milestone 17: Workspace Management (`save`/`load` variables)
- [ ] Milestone 18: Native Installers (Windows `.msi`, Linux AppImage)
- [ ] Milestone 19: Documentation Website & Community Guidelines

## 📚 Developer Documentation
Interested in looking under the hood of NumLite or contributing?
Read [ARCHITECTURE.md](ARCHITECTURE.md) to understand how the Lexer, Parser, and Evaluator work.

## 👨‍💻 Author
Created and developed by **Gilang Hukma Rahmanda & Bertha Adela**.

## 📜 License
This project is licensed under the MIT License.
