use gfx;
use gfx_debug_draw::DebugRenderer;
use input::{GenericEvent, PressEvent, ReleaseEvent, UpdateEvent, Key};
use input::Button::Keyboard;

/// A single menu item instance
pub enum MenuItem<T> {
    ActionItem(ActionMenuItem<T>),
    SliderItem(SliderMenuItem<T>),
}

impl<T> MenuItem<T> {
    pub fn draw<R, F>(&self, settings: &T, debug_renderer: &mut DebugRenderer<R, F>, position: [i32; 2], selected: bool)
        where R: gfx::Resources,
              F: gfx::Factory<R> {
        match self {
            &MenuItem::ActionItem(ref item) => item.draw(settings, debug_renderer, position, selected),
            &MenuItem::SliderItem(ref item) => item.draw(settings, debug_renderer, position, selected),
        }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E, settings: &mut T) {
        match self {
            &mut MenuItem::ActionItem(ref mut item) => item.event(e, settings),
            &mut MenuItem::SliderItem(ref mut item) => item.event(e, settings),
        }
    }

    pub fn action_item(text: &str, action: Box<Fn(&mut T) -> ()>) -> MenuItem<T> {
        MenuItem::ActionItem(ActionMenuItem {
            text: text.to_string(),
            action: action,
        })
    }

    pub fn slider_item(label: &str, range: [f32; 2], step_size: f32, value_getter: Box<Fn(&T) -> f32>, value_setter: Box<Fn(&mut T, f32) -> ()>) -> MenuItem<T> {
        MenuItem::SliderItem(SliderMenuItem {
            label: label.to_string(),
            range: range,
            step_size: step_size,
            get_value: value_getter,
            set_value: value_setter,
            state: SliderMenuState::Default,
        })
    }
}

/// Executes a single action callback when spacebar or left/right arrow keys are hit
pub struct ActionMenuItem<T> {
    text: String,
    action: Box<Fn(&mut T) -> ()>,
}

impl<T> ActionMenuItem<T> {
    pub fn draw<R, F>(&self, _settings: &T, debug_renderer: &mut DebugRenderer<R, F>, position: [i32; 2], selected: bool)
        where R: gfx::Resources,
              F: gfx::Factory<R> {

        let color = if selected {
            [1.0, 0.5, 0.5, 1.0]
        } else {
            [0.5, 0.5, 0.5, 1.0]
        };

        debug_renderer.draw_text_on_screen(
            &self.text[..],
            position,
            color,
        );
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E, settings: &mut T) {
        e.press(|button| {
            match button {
                Keyboard(Key::Space) | Keyboard(Key::Left) | Keyboard(Key::Right) => { (*self.action)(settings); },
                _ => {}
            }
        });
    }
}

enum SliderMenuState {
    Default,
    Increasing,
    Decreasing,
}

/// Increments and decrements some value with left and right arrow keys
pub struct SliderMenuItem<T> {
    label: String,
    range: [f32; 2],
    get_value: Box<Fn(&T) -> f32>,
    set_value: Box<Fn(&mut T, f32) -> ()>,
    step_size: f32,
    state: SliderMenuState,
}

impl<T> SliderMenuItem<T> {

    pub fn draw<R, F>(&self, settings: &T, debug_renderer: &mut DebugRenderer<R, F>, position: [i32; 2], selected: bool)
        where R: gfx::Resources,
              F: gfx::Factory<R> {

        let color = if selected {
            [1.0, 0.5, 0.5, 1.0]
        } else {
            [0.5, 0.5, 0.5, 1.0]
        };

        let value = (*self.get_value)(settings);

        let text = format!("{} {}", self.label, value);

        debug_renderer.draw_text_on_screen(
            &text[..],
            position,
            color,
        );
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E, settings: &mut T) {

        e.update(|_| {
            match self.state {
                SliderMenuState::Increasing => {
                    let current_value = (*self.get_value)(settings);
                    let new_value = self.range[1].min(current_value + self.step_size);
                    (*self.set_value)(settings, new_value);
                },
                SliderMenuState::Decreasing => {
                    let current_value = (*self.get_value)(settings);
                    let new_value = self.range[0].max(current_value - self.step_size);
                    (*self.set_value)(settings, new_value);
                },
                _ => {}
            }
        });


        e.press(|button| {
            match button {
                Keyboard(Key::Right) => {
                    self.state = SliderMenuState::Increasing
                },
                Keyboard(Key::Left) => {
                    self.state = SliderMenuState::Decreasing
                },
                _ => {}
            }
        });

        e.release(|button| {
            match button {
                Keyboard(Key::Right) | Keyboard(Key::Left) => {
                    self.state = SliderMenuState::Default
                },
                _ => {}
            }
        });
    }

}
