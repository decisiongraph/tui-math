//! # tui-math
//!
//! Render LaTeX math beautifully in terminal UIs with ratatui.
//!
//! ## Example
//!
//! ```rust,no_run
//! use tui_math::{MathWidget, render_latex};
//!
//! // Render LaTeX to Unicode string
//! let rendered = render_latex(r"\frac{x^2 + 1}{y}").unwrap();
//! println!("{}", rendered);
//!
//! // Or use as a ratatui widget
//! let widget = MathWidget::new(r"\int_0^\infty e^{-x^2} dx");
//! ```

mod canvas_widget;
mod mathbox;
mod renderer;
mod unicode_maps;
mod widget;

pub use canvas_widget::CanvasMathWidget;
pub use mathbox::MathBox;
pub use renderer::{MathRenderer, RenderError};
pub use widget::{MathWidget, MathWidgetState, StatefulMathWidget};

/// Render LaTeX math to a Unicode string for terminal display
pub fn render_latex(latex: &str) -> Result<String, RenderError> {
    let renderer = MathRenderer::new();
    renderer.render_latex(latex)
}

/// Render MathML to a Unicode string for terminal display
pub fn render_mathml(mathml: &str) -> Result<String, RenderError> {
    let renderer = MathRenderer::new();
    renderer.render_mathml(mathml)
}
