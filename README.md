# dev_menu [![Build Status](https://travis-ci.org/PistonDevelopers/dev_menu.png?branch=master)](https://travis-ci.org/PistonDevelopers/dev_menu)
In-game developer menu for Piston and [gfx-rs](https://github.com/gfx-rs/gfx-rs)

[Documentation](http://www.piston.rs/docs/dev_menu/dev_menu/)

## Usage

A `Menu` instance can be intitialized to operate on some particular type, for example a "Settings" struct:

```Rust
pub struct Settings {
	setting_a: bool,
	setting_b: f32,
	// ... etc
}

...

let mut menu = dev_menu::Menu<Settings>::new();

```

Menu items display in a vertical list, and the selection can be changed with the up and down arrow keys.

Items can be added to the menu with `add_item`. An `ActionItem`, when selected, executes a given closure when the spacebar or left or right arrow keys are hit. For example, it can be used to toggle a boolean setting as follows:

```Rust
menu.add_item(dev_menu::MenuItem::action_item(
	"Toggle Setting A", // Label for the item
	Box::new(|ref mut settings| { settings.setting_a = !settings.setting_a; }) // Closure to execute
));
```

A `SliderItem`, when selected, can be used to increment or decrement a particular value while the right or left arrow keys are held down, using a pair of accessor closures to get or set the value within the settings object. It will also display the current value to the right of the label. For example:

```Rust
menu.add_item(dev_menu::MenuItem::slider_item(
	"Setting B = ", // Label for the item. Value will show to the right
	[-5.0, 5.0], // Valid range for the value
	0.01, // Value to increment / decrement by on each update, while key is held down
	Box::new(|ref settings| { settings.setting_b }), // Closure for retrieving value
	Box::new(|ref mut settings, value| { settings.setting_b = value }), // Closure for setting value
));
```


To update and render the menu, using the Piston event loop:

```Rust
for e in window.events() {

	// Send event to menu, with the settings object that should be accessed and/or modified
	menu.event(&e, &mut settings);

	...

	e.render(|args| {

		...

		// Draw the menu with gfx_debug_draw::DebugRenderer
		menu.draw(&settings, &mut debug_renderer);

		...
	}
}
```
