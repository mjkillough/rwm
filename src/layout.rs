use groups::{GroupIter, GroupWindow};
use window::Window;


pub trait Layout {
    fn layout(&self, width: i32, height: i32, focused: Option<GroupWindow>, stack: GroupIter);
}


pub struct TiledLayout;

impl Layout for TiledLayout {
    fn layout(&self, width: i32, height: i32, _: Option<GroupWindow>, stack: GroupIter) {
        if stack.len() == 0 {
            return;
        }

        let tile_height = height / stack.len() as i32;

        for (i, window) in stack.enumerate() {
            window.without_focus_tracking(|window| {
                                              window.map();
                                              window.configure(0,
                                                               i as i32 * tile_height,
                                                               width,
                                                               tile_height);
                                          });
        }
    }
}


pub struct StackLayout;

impl Layout for StackLayout {
    fn layout(&self, width: i32, height: i32, focused: Option<GroupWindow>, stack: GroupIter) {
        if stack.len() == 0 {
            return;
        }


        for window in stack {
            window.without_focus_tracking(|window| window.unmap());
        }
        focused.map(|window| {
                        window.without_focus_tracking(|window| {
                                                          window.map();
                                                          window.configure(0, 0, width, height);
                                                      })
                    });
    }
}
