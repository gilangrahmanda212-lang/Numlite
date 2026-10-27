```markdown
# NumLite Code Architecture

This document explains how NumLite works under the hood. If you want to modify the NumLite engine, add new functions, or are just curious about the source code, start here.

NumLite uses a **Tree-Walking Interpreter** approach. This means the code you type in the Editor is converted into Tokens, then into a Tree (Abstract Syntax Tree), and executed directly by the evaluator.

## 📂 Directory Structure (`src/`)

Here is the explanation of each file inside the `src/` folder:

| File | Main Function |
|---|---|
| `main.rs` | Application entry point. Detects whether to run in GUI mode or CLI mode based on command-line arguments. |
| `app.rs` | Contains all the GUI logic (Editor, Console Output, Figure Panel, and Pop-out windows). |
| `gui.rs` | Contains the `PlotData` struct used to pass plotting data from the engine to the GUI. |
| `lexer.rs` | Converts raw text (String) into small pieces called Tokens. It also handles comments (`%`) and strings (`'...'`). |
| `token.rs` | Contains an enum of all recognized Tokens (Numbers, Operators, `for` keyword, etc). |
| `parser.rs` | Reads Tokens and converts them into a tree structure (AST). It handles operator precedence and block parsing (`for`, `if`). |
| `ast.rs` | Defines the *Abstract Syntax Tree* data structures (`Expr` for expressions, `Stmt` for statements). |
| `eval.rs` | The "brain" of NumLite. Executes instructions from the AST, handles variables, loops, and calculates mathematical values. |
| `value.rs` | Defines the data types that can be processed (Scalar, Matrix, String) and how to format them for display. |

---

## ⚙️ Execution Flow (How Code Runs)

When a user types `x = [1, 2] + 3` and clicks **Run Script**, the flow is as follows:

1. **Lexer (`lexer.rs`)**
   The text is split into Tokens: `[Identifier("x"), Assign, LBracket, Number(1), ...]`.

2. **Parser (`parser.rs`)**
   Tokens are converted into an AST (`ast.rs`). The Parser ensures mathematical operators are calculated in the correct order.
   *Result: `Stmt::Assign { name: "x", value: Expr::Binary { ... } }`*

3. **Evaluator (`eval.rs`)**
   The AST is executed. The `eval()` function calculates the value of `[1, 2] + 3` into `Value::Matrix([4, 5])`.

4. **Environment (`Env`)**
   The variable `x` and its value are stored in memory (a HashMap) so it can be called again on the next line.

5. **Output (`app.rs`)**
   If there is `disp()` text, `ans`, or a `plot()` graph, the Evaluator sends them to `app.rs` to be displayed on the screen.

---

## 🛠️ How to Add New Math Functions

For example, if you want to add the `log10()` function.

1. Open **`src/eval.rs`**.
2. Find the `fn call_builtin(...)` function.
3. Add a new *match arm* inside it:

```rust
("log10", 1) => unary_fn("log10", &args[0], |x| x.log10()),
("cos", 1) => unary_fn("cos", &args[0], |x| x.cos()),
```

*Note: `unary_fn` is a helper function in `eval.rs` that automatically handles whether the input is a single Scalar or a Matrix.*

## 🛠️ How to Add New GUI Plot Types

If you want to add a new plot type (e.g., `stem()` or `bar()`):
1. Add a new variant to `Stmt` in **`src/ast.rs`** (e.g., `Stem { title, x, y }`).
2. Update `parse_stmt` in **`src/parser.rs`** to detect the `stem` function call.
3. Update `exec_stmt` in **`src/eval.rs`** to push the plot data into the `plots` vector.
4. Finally, update **`src/app.rs`** to draw the new plot type on the canvas.

## 🐛 Found a Bug?
If you find a bug (e.g., an error when typing a specific expression), please open a new *Issue* in this repository and include the code snippet or a screenshot that caused the error.

## 📜 License
This project is licensed under the MIT License.
```

