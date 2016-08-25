use gfx;
use gfx_debug_draw::DebugRenderer;
use input::{GenericEvent, Key};
use input::Button::Keyboard;

use menu_item::MenuItem;

type ItemIndex = usize;

/// An in-game developer menu that responds to keyboard events,
/// and can be drawn using a DebugRenderer instance,
pub struct Menu<T> {
    items: Vec<MenuItem<T>>,
    selected_item: ItemIndex,
}

impl<T> Menu<T> {

    /// Create a new Menu instance
    pub fn new() -> Menu<T> {
        Menu {
            items: Vec::new(),
            selected_item: 0,
        }
    }

    /// Add a MenuItem to the menu
    pub fn add_item(&mut self, item: MenuItem<T>) {
        self.items.push(item);
    }

    /// Draw the menu using the current settings object
    pub fn draw<R, F>(&self, settings: &T, debug_renderer: &mut DebugRenderer<R, F>)
        where R: gfx::Resources,
              F: gfx::Factory<R> {

        let left_margin = 10;
        let top_margin = 10;
        let item_space = 20;

        for (index, item) in self.items.iter().enumerate() {
            let selected = index == self.selected_item;
            item.draw(settings, debug_renderer, [left_margin, top_margin + item_space * index as i32], selected);
        }
    }

    /// Respond to keyboard events
    pub fn event<E: GenericEvent>(&mut self, e: &E, settings: &mut T) {

        e.press(|button| {
            match button {

                Keyboard(Key::Up) => {
                    self.selected_item = self.selected_item.wrapping_sub(1) % self.items.len();
                }

                Keyboard(Key::Down) => {
                    self.selected_item = self.selected_item.wrapping_add(1) % self.items.len();
                }

                _ => {}
            }
        });

        let selected_item = &mut self.items[self.selected_item];
        selected_item.event(e, settings);
    }
}
