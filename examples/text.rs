use example_runner::{ExampleDescriptor, RunError, WindowAttributes};
use keibrush::{FontFamily, Rect, Scene, Size2, Span, TextStyle, Vec2};

fn main() -> Result<(), RunError> {
    example_runner::run(
        ExampleDescriptor {
            window_attributes: WindowAttributes::default()
                .with_title("keibrush - Text Example"),
            ..Default::default()
        },
        render,
    )
}

fn render(scene: &mut Scene, size: Size2<f32>) {
    let bounds = Rect::from_size(size).with_insets(Vec2::splat(24.0));
    let half_height = bounds.size.map_h(|h| h / 2.0);
    let upper_half = bounds.with_size(half_height);
    let lower_half = upper_half.map_origin(|origin| origin + Vec2::from_y(half_height.h));

    scene.draw_text(
        "hello world! this is an example of plain text (`Source::Plain`). The provided `TextStyle` applies to all text.",
        upper_half,
        TextStyle { size: 36.0, ..Default::default() },
    );
    scene.draw_text(
        [
            Span::new("hello world!").with_italic(),
            Span::new(" this is an example of rich text ("),
            Span::new("`Source::Rich`").with_font_family(FontFamily::Monospace),
            Span::new("). The provided "),
            Span::new("`TextStyle`").with_font_family(FontFamily::Monospace),
            Span::new(" applies to all text, but can be overriden by each span."),
        ],
        lower_half,
        TextStyle { size: 36.0, ..Default::default() },
    );
}
