use layout::Layout;
use stack::Stack;
use Viewport;
use x::{Connection, WindowId};


#[derive(Clone)]
pub struct TiledLayout {
    name: String,
    padding: u32,
}

impl TiledLayout {
    pub fn new<S: Into<String>>(name: S, padding: u32) -> Box<Layout> {
        Box::new(TiledLayout {
            name: name.into(),
            padding,
        })
    }
}

impl Layout for TiledLayout {
    fn name(&self) -> &str {
        &self.name
    }

    fn layout(&self, connection: &Connection, viewport: &Viewport, stack: &Stack<WindowId>) {
        if stack.len() == 0 {
            return;
        }

        let tile_height = ((viewport.height - self.padding) / stack.len() as u32) - self.padding;

        for (i, window_id) in stack.iter().enumerate() {
            connection.disable_window_tracking(window_id);
            connection.map_window(window_id);
            connection.configure_window(
                window_id,
                viewport.x + self.padding,
                viewport.y + self.padding + (i as u32 * (tile_height + self.padding)),
                viewport.width - (self.padding * 2),
                tile_height,
            );
            connection.enable_window_tracking(window_id);
        }
    }
}
