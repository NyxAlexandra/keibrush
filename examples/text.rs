use example_runner::{ExampleDescriptor, RunError, WindowAttributes};
use keibrush::{Rect, Scene, Size2, TextStyle, Vec2};

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

const LOREM_IPSUM: &str =
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vestibulum in mi a metus \
     egestas iaculis quis et ante. Nullam sodales tellus at hendrerit elementum. Nunc \
     vulputate mattis elit ac semper. Fusce sollicitudin scelerisque est a venenatis. \
     Nam consectetur est id mauris finibus maximus. Maecenas id cursus urna. Nullam \
     porta risus quis euismod dignissim. Fusce placerat ultrices convallis. ";

fn render(scene: &mut Scene, size: Size2<f32>) {
    let half_size = size.map_w(|w| w / 2.0);
    let left_half = Rect::from_size(half_size);
    let right_half = left_half.map_origin(|origin| origin + Vec2::from_x(half_size.w));

    scene.draw_text(
        LOREM_IPSUM,
        left_half,
        TextStyle { size: 36.0, ..Default::default() },
    );
    scene.draw_text(
        LOREM_IPSUM,
        right_half,
        TextStyle { size: 36.0, ..Default::default() },
    );
}
