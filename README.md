# tui-math

Render LaTeX math beautifully in terminal UIs with [ratatui](https://github.com/ratatui/ratatui).

## Features

- LaTeX to Unicode terminal rendering
- MathML intermediate format support
- Native ratatui widget
- Unicode superscripts/subscripts when possible
- 2D rendering for fractions, roots, big operators
- Greek letters and mathematical symbols

## Installation

```toml
[dependencies]
tui-math = "0.1"
```

## Usage

### Simple rendering

```rust
use tui_math::render_latex;

let rendered = render_latex(r"\frac{x^2 + 1}{y}")?;
println!("{}", rendered);
// Output:
// x²+1
// ────
//   y
```

### As a ratatui widget

```rust
use tui_math::MathWidget;
use ratatui::widgets::Block;

let widget = MathWidget::new(r"\int_0^\infty e^{-x^2} dx")
    .block(Block::bordered().title("Math"));

frame.render_widget(widget, area);
```

### Stateful widget for caching

```rust
use tui_math::{MathWidgetState, StatefulMathWidget};

// Create state once
let mut state = MathWidgetState::new();
state.update(r"\sum_{i=1}^n i^2", true);

// Render multiple times without re-parsing
let widget = StatefulMathWidget::new();
widget.render(area, buf, &state);
```

## Examples

Run the interactive demo:

```sh
cargo run
```

Or the simple CLI example:

```sh
cargo run --example simple
```

## Output Examples

```
LaTeX: x^2 + y^2 = z^2
Rendered: x²+y²=z²

LaTeX: \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}
Rendered:
  -b±√b²-4ac
x=──────────
      2a

LaTeX: \sum_{i=1}^n i^2
Rendered:
 n
 ∑ i²
i=1
```

## Supported LaTeX

- Basic math: `+`, `-`, `*`, `/`, `=`, etc.
- Superscripts: `x^2`, `e^{i\pi}`
- Subscripts: `a_1`, `x_{ij}`
- Fractions: `\frac{a}{b}`
- Square roots: `\sqrt{x}`, `\sqrt[3]{x}`
- Greek letters: `\alpha`, `\beta`, `\Gamma`, etc.
- Big operators: `\sum`, `\prod`, `\int`, `\oint`
- Relations: `\leq`, `\geq`, `\neq`, `\equiv`, etc.
- Arrows: `\rightarrow`, `\Rightarrow`, `\leftrightarrow`
- Functions: `\sin`, `\cos`, `\log`, `\lim`, etc.
- Delimiters: `\langle`, `\rangle`, `\lceil`, `\rfloor`

## How it works

1. LaTeX → MathML via `latex2mathml`
2. MathML parsed with `roxmltree`
3. Rendered to 2D character grid (MathBox)
4. Unicode characters used where possible
5. Integrated with ratatui's Widget trait

## License

MIT
