use std::{error::Error, sync::Mutex};

use core_foundation::runloop::{CFRunLoop, kCFRunLoopCommonModes};
use core_graphics::event::{
    CGEvent, CGEventFlags, CGEventTap, CGEventTapLocation, CGEventTapOptions, CGEventTapPlacement,
    CGEventTapProxy, CGEventType, CallbackResult, EventField,
};

use crate::{
    key_emitter,
    tap_resolver::{Input, Key, TapResolver},
};

/// Let Command (kVK_Command)
const KEYCODE_LEFT_COMMAND: i64 = 0x37;
/// Right Command (kVK_RightCommand)
const KEYCODE_RIGHT_COMMAND: i64 = 0x36;

pub fn listen() -> Result<(), Box<dyn Error>> {
    let resolver = Mutex::new(TapResolver::new());

    let event = vec![
        CGEventType::FlagsChanged,
        CGEventType::KeyDown,
        CGEventType::LeftMouseDown,
        CGEventType::RightMouseDown,
        CGEventType::OtherMouseDown,
    ];

    let tap = CGEventTap::new(
        CGEventTapLocation::Session,
        CGEventTapPlacement::HeadInsertEventTap,
        CGEventTapOptions::ListenOnly,
        event,
        move |_: CGEventTapProxy, event_type: CGEventType, event: &CGEvent| -> CallbackResult {
            if let Some(input) = translate(event_type, event) {
                let action = resolver.lock().unwrap().resolve(input);
                if let Some(action) = action {
                    key_emitter::emit(action);
                }
            }
            CallbackResult::Keep
        },
    );
    let tap = match tap {
        Ok(v) => v,
        Err(_) => return Err("failed to create event tap".into()),
    };

    let loop_source = match tap.mach_port().create_runloop_source(0) {
        Ok(v) => v,
        Err(_) => return Err("failed to create runloop source".into()),
    };
    let current = CFRunLoop::get_current();
    unsafe {
        current.add_source(&loop_source, kCFRunLoopCommonModes);
    }

    println!("kanade: running. Tap left Command for 英数, right Command for かな. Ctrl-C to quit.");
    CFRunLoop::run_current();
    Ok(())
}

fn translate(event_type: CGEventType, event: &CGEvent) -> Option<Input> {
    match event_type {
        CGEventType::FlagsChanged => {
            let keycode = event.get_integer_value_field(EventField::KEYBOARD_EVENT_KEYCODE);
            let is_command_down = event.get_flags().contains(CGEventFlags::CGEventFlagCommand);

            match keycode {
                KEYCODE_LEFT_COMMAND => Some(if is_command_down {
                    Input::Down(Key::LeftCommand)
                } else {
                    Input::Up(Key::LeftCommand)
                }),
                KEYCODE_RIGHT_COMMAND => Some(if is_command_down {
                    Input::Down(Key::RightCommand)
                } else {
                    Input::Up(Key::RightCommand)
                }),
                _ => Some(Input::Other),
            }
        }
        CGEventType::KeyDown
        | CGEventType::LeftMouseDown
        | CGEventType::RightMouseDown
        | CGEventType::OtherMouseDown => Some(Input::Other),
        _ => None,
    }
}
