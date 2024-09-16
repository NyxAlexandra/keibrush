//! Renders a `red -> blue` gradient from the top-left corner to the
//! bottom-right.

use example_runner::winit::window::WindowAttributes;
use example_runner::{ExampleDescriptor, RunError};
use keibrush::{
    Color,
    ColorStop,
    FillStyle,
    LinearGradient,
    Point2,
    Rect,
    Scene,
    Size2,
    Zero,
};

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
            stops: vec![
                ColorStop { offset: 0.0, color: Color::RED },
                ColorStop { offset: 1.0, color: Color::BLUE },
            ],
            ..Default::default()
        },
        FillStyle::default(),
    );
}
