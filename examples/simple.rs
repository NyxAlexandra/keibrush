use example_runner::{RunError, WindowAttributes};
use keibrush::{
    Color, FillStyle, Font, FontFamily, Point2, Rect, Scene, Size2, StrokeStyle,
    TextStyle,
};

fn main() -> Result<(), RunError> {
    example_runner::run(
        WindowAttributes::default().with_title("keibrush - Simple Example"),
        render,
    )
}

fn render(scene: &mut Scene, size: Size2<f32>) {
    scene.fill(Rect::from_size(size), Color::GREEN, FillStyle::default());
    scene.fill(
        Rect::new(Point2::splat(100.0), Size2::new(200.0, 100.0)),
        Color::RED,
        FillStyle::default(),
    );
    scene.stroke(
        Rect::new(Point2::new(600.0, 400.0), Size2::splat(66.6)),
        Color::BLUE,
        StrokeStyle { width: 32.0, ..Default::default() },
    );
    scene.draw_text(
        "hello world!",
        Rect::new(Point2::splat(200.0), size),
        TextStyle {
            font: Font {
                family: FontFamily::Named("Noto Sanszzzz".into()),
                fallback: vec![FontFamily::Monospace],
                ..Default::default()
            },
            size: 32.0,
            ..Default::default()
        },
    );
}
