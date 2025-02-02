//! Event handling (mouse, keyboard, controller, touch screen, etc.)
//!
//! See [`Event`](enum.Event.html) for more information.

#[cfg(target_arch = "wasm32")]
compile_error!("This module should not be included when compiling to wasm");

use serde::{Serialize, Deserialize};
use piston_window::{Button, ButtonArgs, ButtonState, Event as PistonEvent, Input, Motion};
pub use piston_window::{ControllerAxisArgs as ControllerAxis, ControllerButton, Key, MouseButton, TouchArgs as Touch};

use crate::Point;

/// Possible events returned from [`Drawing::poll_event()`](../struct.Drawing.html#method.poll_event).
///
/// Events are used to make programs more interactive.
/// See that method's documentation for more information about how to use events.
///
/// This type is meant to provide a simplified model of `piston_window`'s `Event` type.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Event {
    /// Sent when a keyboard key is pressed
    KeyPressed(Key),
    /// Sent when a keyboard key is released
    KeyReleased(Key),
    /// Sent when a mouse button is pressed
    MouseButtonPressed(MouseButton),
    /// Sent when a mouse button is released
    MouseButtonReleased(MouseButton),
    /// Sent when a controller button is pressed
    ControllerButtonPressed(ControllerButton),
    /// Sent when a controller button is released
    ControllerButtonReleased(ControllerButton),
    /// Sent when a controller axis (usually a joystick) is changed
    ControllerAxisChange(ControllerAxis),

    /// Sent when the mouse is moving. Only sent when the mouse is over the window.
    /// `x` and `y` represent the new coordinates of where the mouse is currently.
    ///
    /// Coordinates are relative to the center of the window.
    MouseMove { x: f64, y: f64 },
    /// Sent when the mouse is scrolled. Only sent when the mouse is over the window.
    /// `x` and `y` are in scroll ticks.
    MouseScroll { x: f64, y: f64 },

    /// Sent when a user touches the screen
    Touch(Touch),

    /// Sent when the window gets resized
    WindowResized { width: u32, height: u32 },

    /// Sent when the window focus changes
    ///
    /// The boolean value is true if the window is in focus.
    WindowFocused(bool),
    /// Sent when the window gains or loses the cursor.
    ///
    /// The boolean value is true if the window gained the cursor.
    WindowCursor(bool),
    /// Sent when the window is closed
    WindowClosed,
}

/// Attempts to convert a piston Event to our event type
pub(crate) fn from_piston_event<F>(event: &PistonEvent, to_local_coords: F) -> Option<Event>
where
    F: FnOnce(Point) -> Point,
{
    use Event::*;

    let input_event = match *event {
        PistonEvent::Input(ref input_event) => input_event,
        _ => return None,
    };

    Some(match *input_event {
        Input::Button(ButtonArgs {
            state,
            button,
            ..
        }) => match state {
            ButtonState::Press => match button {
                Button::Keyboard(key) => KeyPressed(key),
                Button::Mouse(button) => MouseButtonPressed(button),
                Button::Controller(button) => ControllerButtonPressed(button),
                Button::Hat(_) => unimplemented!(), //TODO: No idea what to do with this
            },
            ButtonState::Release => match button {
                Button::Keyboard(key) => KeyReleased(key),
                Button::Mouse(button) => MouseButtonReleased(button),
                Button::Controller(button) => ControllerButtonReleased(button),
                Button::Hat(_) => unimplemented!(), //TODO: No idea what to do with this
            },
        },
        Input::Move(motion) => match motion {
            Motion::MouseCursor(x, y) => {
                let local = to_local_coords(Point { x, y });
                MouseMove { x: local.x, y: local.y }
            }
            // Ignored in favor of MouseCursor
            Motion::MouseRelative(..) => return None,
            Motion::MouseScroll(x, y) => MouseScroll { x, y },
            Motion::ControllerAxis(axis) => ControllerAxisChange(axis),
            Motion::Touch(touch) => Touch(touch),
        },
        // Ignored because this value doesn't produce text reliably for all keys
        // (especially when ctrl is pressed)
        Input::Text(_) => return None,
        // Not supported
        Input::FileDrag(_) => return None,
        Input::Resize(width, height) => WindowResized { width: width as u32, height: height as u32 },
        Input::Focus(focused) => WindowFocused(focused),
        Input::Cursor(cursor) => WindowCursor(cursor),
        Input::Close(_) => WindowClosed,
    })
}
