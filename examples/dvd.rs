use example_runner::winit::window::WindowAttributes;
use example_runner::{ExampleDescriptor, RunError};
use keibrush::element::{Color, FillStyle};
use keibrush::math::{Point2, Rect, Size2, Vec2};

fn main() -> Result<(), RunError> {
    let mut extent = Rect::new(Point2::splat(100.0), Size2::new(200.0, 100.0));
    let mut velocity = Vec2::new(13.0, 11.0);

    example_runner::run(
        ExampleDescriptor {
            window_attributes: WindowAttributes::default().with_title("keibrush - DvD Example"),
            always_redraw: true,
            ..Default::default()
        },
        move |scene, size| {
            let bounds = Rect::from_size(size);

            extent.origin += velocity;

            if (extent.left() <= -bounds.left() && velocity.x < 0.0)
            // collide right
            || (extent.right() >= bounds.right() && velocity.x > 0.0)
            {
                velocity.x = -velocity.x;
            }

            // collide top
            if (extent.top() <= bounds.top() && velocity.y < 0.0)
            // collide bottom
            || (extent.bottom() >= bounds.bottom() && velocity.y > 0.0)
            {
                velocity.y = -velocity.y;
            }

            extent.origin.x = extent.origin.x.clamp(bounds.left(), bounds.right());
            extent.origin.y = extent.origin.y.clamp(bounds.top(), bounds.bottom());

            scene.fill(extent, Color::BLUE, FillStyle::default());
        },
    )
}
