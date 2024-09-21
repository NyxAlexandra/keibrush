use parley::fontique::{Collection, CollectionOptions};
use parley::{FontContext, LayoutContext};
use vello::peniko;

/// State used to measure and render text.
pub struct TextContext {
    pub(super) layout_cx: LayoutContext<peniko::Brush>,
    pub(super) font_cx: FontContext,
}

/// Descriptor for creating a [`TextContext`].
#[derive(Clone)]
pub struct TextContextDescriptor {
    /// Whether to load fonts from the system or not (default: `true`).
    pub use_system_fonts: bool,
}

impl TextContext {
    /// Creates a new text context.
    pub fn new(desc: TextContextDescriptor) -> Self {
        let TextContextDescriptor { use_system_fonts } = desc;

        let layout_cx = LayoutContext::new();
        let font_cx = FontContext {
            collection: Collection::new(CollectionOptions {
                shared: false,
                system_fonts: use_system_fonts,
            }),
            ..Default::default()
        };

        Self { layout_cx, font_cx }
    }
}

impl Default for TextContext {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl Default for TextContextDescriptor {
    fn default() -> Self {
        Self { use_system_fonts: true }
    }
}
