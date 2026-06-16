use core_graphics::event::{CGEvent, CGEventTapLocation, KeyCode};
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

use crate::tap_resolver::Action;

/// emits 英数/かな keys corresponding to actions.
pub fn emit(action: Action) {
    let keycode = match action {
        Action::Eisu => KeyCode::JIS_EISU,
        Action::Kana => KeyCode::JIS_KANA,
    };

    let Ok(source) = CGEventSource::new(CGEventSourceStateID::HIDSystemState) else {
        return;
    };

    if let Ok(down) = CGEvent::new_keyboard_event(source.clone(), keycode, true) {
        down.post(CGEventTapLocation::HID);
    }

    if let Ok(up) = CGEvent::new_keyboard_event(source, keycode, false) {
        up.post(CGEventTapLocation::HID);
    }
}
