//! Renders a `red -> blue` gradient from the top-left corner to the
//! bottom-right.

use example_runner::winit::window::WindowAttributes;
use example_runner::{ExampleDescriptor, RunError};
use keibrush::element::{Color, ColorStop, FillStyle, LinearGradient};
use keibrush::math::{Point2, Rect, Size2, Zero};
use keibrush::Scene;

fn main() -> Result<(), RunError> {
    example_runner::run(
        ExampleDescriptor {
            window_attributes: WindowAttributes::default()
                .with_title("keibrush - Gradients Example"),
            ..Default::default()
        },
        render,
    )
}

fn render(scene: &mut Scene, size: Size2<f32>) {
    let start = Point2::ZERO;
    let end = start + size;

    scene.fill(
        Rect::from_size(size),
        LinearGradient {
            start,
            end,
            stops: vec![ColorStop::new(0.0, Color::RED), ColorStop::new(1.0, Color::BLUE)],
            ..Default::default()
        },
        FillStyle::default(),
    );
}
