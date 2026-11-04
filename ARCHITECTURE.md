```markdown
# NumLite Code Architecture

This document explains how NumLite works under the hood. If you want to modify the NumLite engine, add new functions, or are just curious about the source code, start here.

NumLite uses a **Tree-Walking Interpreter** approach. This means the code you type in the Editor is converted into Tokens, then into a Tree (Abstract Syntax Tree), and executed directly by the evaluator.

## 📂 Directory Structure (`src/`)

Here is the explanation of each file inside the `src/` folder:

| File | Main Function |
|---|---|
| `main.rs` | Application entry point. Detects whether to run in GUI mode or CLI mode based on command-line arguments. |
| `app.rs` | Contains all the GUI logic (Editor, Console Drawer, Figure Panel, Magnifier Pop-out). |
| `gui.rs` | Contains the `PlotData` struct used to pass plotting data from the engine to the GUI. |
| `lexer.rs` | Converts raw text (String) into Tokens. It tracks line numbers and handles comments (`%`) and strings (`'...'`). |
| `token.rs` | Contains `TokenKind` (the type of token) and `Token` (the token combined with its line number). |
| `parser.rs` | Reads Tokens and converts them into an AST. It handles operator precedence, block parsing, and injects line numbers into AST nodes. |
| `ast.rs` | Defines the *Abstract Syntax Tree* (`Expr` and `Stmt` structs which wrap `ExprKind`/`StmtKind` and a `line` number). |
| `eval.rs` | The core engine. Executes instructions from the AST, handles variables, loops, and catches errors to display them with exact line numbers. |
| `builtins.rs` | (Separated from eval) Contains the `call_builtin` function where all mathematical, matrix, and string functions are implemented. |
| `math_utils.rs` | (Separated from eval) Contains heavy mathematical algorithms like FFT, Matrix Inversion, and Determinants. |
| `value.rs` | Defines the data types (`Scalar`, `Matrix`, `Complex`, `String`, `Struct`, `Cell`) and how to format them for display. |
| `file_dialog.rs` | Handles cross-platform file open/save dialogs. |
| `lib.rs` | Exposes all modules to be used by `main.rs`. |

---

## ⚙️ Execution Flow (How Code Runs)

When a user types `x = [1, 2] + 3` and clicks **Run Script**, the flow is as follows:

1. **Lexer (`lexer.rs`)**
   The text is split into Tokens: `[Identifier("x"), Assign, LBracket, Number(1), ...]`. At this stage, the `current_line` counter increments every time a newline (`\n`) is found, attaching line info to each Token.

2. **Parser (`parser.rs`)**
   Tokens are converted into an AST (`ast.rs`). The Parser ensures mathematical operators are calculated in the correct order and wraps the final node with the line number where it occurred.
   *Result: `Stmt { kind: StmtKind::Assign { name: "x", value: Expr { ... } }, line: 1 }`*

3. **Evaluator (`eval.rs`)**
   The AST is executed. The `eval()` function calculates the value of `[1, 2] + 3`. If an error occurs (e.g., undefined variable), it catches the error and appends the `line` number from the AST node.

4. **Environment (`Env`)**
   The variable `x` and its value are stored in memory (a HashMap) so it can be called again on the next line.

5. **Output (`app.rs`)**
   If there is `disp()` text, `ans`, or a `plot()` graph, the Evaluator sends them to `app.rs` to be displayed on the screen.

---

## 🛠️ How to Add New Math Functions

Since we have refactored the code, math functions are now located in `src/builtins.rs`.

For example, if you want to add the `log10()` function:
1. Open **`src/builtins.rs`**.
2. Find the `pub fn call_builtin(...)` function.
3. Add a new *match arm* inside it:

```rust
("log10", 1) => unary_fn("log10", &args[0], |x| x.log10()),
("cos", 1) => unary_fn("cos", &args[0], |x| x.cos()),
```

*Note: `unary_fn` is a helper function that automatically handles whether the input is a single Scalar or a Matrix.*

## 🛠️ How to Add New GUI Plot Types

If you want to add a new plot type (e.g., `bar()`):
1. Add a new variant to `PlotType` in **`src/ast.rs`**.
2. Update `parse_stmt` in **`src/parser.rs`** to detect the `bar` function call and return `StmtKind::Plot { ... }` with `PlotType::Bar`.
3. In **`src/app.rs`**, find the `draw_plot` function and add a `match` arm for `PlotType::Bar` to draw rectangles on the canvas.

## 🐛 Found a Bug?
If you find a bug, please open a new *Issue* in this repository and include the code snippet or a screenshot that caused the error. Our error handler will now tell you exactly which line the error occurred on!

## 📜 License
This project is licensed under the MIT License.
