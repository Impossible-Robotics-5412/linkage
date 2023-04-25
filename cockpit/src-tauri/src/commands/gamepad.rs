use std::sync::atomic::{AtomicBool, Ordering};
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use bus::Bus;
use gilrs::Gilrs;
use messaging::CockpitToLinkage;
use tauri::{Manager, Runtime};

const EVENT_GAMEPAD_EVENT: &str = "gamepad_event";

pub struct GamepadState {
    pub gamepad_event_bus: Arc<Mutex<Bus<Option<CockpitToLinkage>>>>,
    listening: Arc<AtomicBool>,
}

impl GamepadState {
    pub fn new() -> Self {
        Self {
            gamepad_event_bus: Arc::new(Mutex::new(Bus::new(
                std::mem::size_of::<CockpitToLinkage>(),
            ))),
            listening: Arc::new(AtomicBool::new(false)),
        }
    }
}

#[repr(u8)]
enum EventType {
    ButtonChanged = 0,
    AxisChanged = 1,
    Connected = 2,
    Disconnected = 3,
}

#[tauri::command]
pub fn start_gamepad_event_listener<R: Runtime>(
    app: tauri::AppHandle<R>,
    state: tauri::State<'_, GamepadState>,
) {
    if state.listening.load(Ordering::Relaxed) {
        return;
    }
    state.listening.store(true, Ordering::Relaxed);

    let mut gilrs = Gilrs::new().unwrap();

    thread::spawn({
        let mut rx = state.gamepad_event_bus.lock().unwrap().add_rx();

        move || loop {
            if let Ok(Some(message)) = rx.recv() {
                let CockpitToLinkage::GamepadInputEvent { .. } = message;
                app.emit_all(EVENT_GAMEPAD_EVENT, message).unwrap();
            }
        }
    });

    thread::spawn({
        log::debug!("Started gamepad event listener");

        // FIXME: We currently only tell Linkage-lib we have connected controllers by sending an event, but no initial information.
        //        This means Linkage-lib will only know if a gamepad is connected as soon as we send some kind of event.

        let bus = state.gamepad_event_bus.clone();
        move || loop {
            if let Some(event) = gilrs.next_event_blocking(Some(Duration::from_millis(500))) {
                match event.event {
                    gilrs::EventType::ButtonChanged(button, value, _) => {
                        let message = CockpitToLinkage::GamepadInputEvent {
                            gamepad_id: gamepad_id_into_u8(event.id),
                            event_type: EventType::ButtonChanged as u8,
                            control: button as u8,
                            value: (value.clamp(0.0, 1.0) * 255.0) as u8,
                        };

                        bus.lock().unwrap().broadcast(Some(message));
                    }
                    gilrs::EventType::AxisChanged(axis, value, _) => {
                        let message = CockpitToLinkage::GamepadInputEvent {
                            gamepad_id: gamepad_id_into_u8(event.id),
                            event_type: EventType::AxisChanged as u8,
                            control: axis as u8,
                            value: (127.0 + (value.clamp(-1.0, 1.0)) * 255.0) as u8,
                        };

                        bus.lock().unwrap().broadcast(Some(message));
                    }
                    gilrs::EventType::Connected => {
                        let message = CockpitToLinkage::GamepadInputEvent {
                            gamepad_id: gamepad_id_into_u8(event.id),
                            event_type: EventType::Connected as u8,
                            control: 0,
                            value: 0,
                        };

                        bus.lock().unwrap().broadcast(Some(message));
                    }
                    gilrs::EventType::Disconnected => {
                        let message = CockpitToLinkage::GamepadInputEvent {
                            gamepad_id: gamepad_id_into_u8(event.id),
                            event_type: EventType::Disconnected as u8,
                            control: 0,
                            value: 0,
                        };

                        bus.lock().unwrap().broadcast(Some(message));
                    }
                    _ => {}
                }
            }
        }
    });
}

// HACK: This is needed because of the gilrs crate being neglectant.
fn gamepad_id_into_u8(gamepad_id: gilrs::GamepadId) -> u8 {
    unsafe { std::mem::transmute_copy::<gilrs::GamepadId, usize>(&gamepad_id) as u8 }
}
