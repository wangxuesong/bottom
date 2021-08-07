//! Functions regarding "global" app events and how they're handled.

use crossterm::event::{KeyEvent, MouseEvent};

use super::AppState;

#[derive(Debug)]
pub enum InputEvent {
    KeyEvent(KeyEvent),
    MouseEvent(MouseEvent),
}

#[derive(Debug)]
pub enum GlobalEventResult {
    Handled,
    NotHandled,
    Exit,
}

/// Global event handler - that is, these events can be triggered regardless of the selected widget.
pub fn global_event_handler(state: &mut AppState, event: InputEvent) -> GlobalEventResult {
    match event {
        InputEvent::KeyEvent(event) => GlobalEventResult::NotHandled,
        InputEvent::MouseEvent(event) => GlobalEventResult::NotHandled,
    }
}

/// Move to the nearest widget to the left of the current one if possible.
fn move_to_left_widget(state: &mut AppState) {}

/// Move to the nearest widget to the right of the current one if possible.
fn move_to_right_widget(state: &mut AppState) {}

/// Move to the nearest widget above the the current one if possible.
fn move_to_up_widget(state: &mut AppState) {}

/// Move to the nearest widget below the current one if possible.
fn move_to_down_widget(state: &mut AppState) {}
