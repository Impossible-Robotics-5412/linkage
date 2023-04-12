use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use bus::Bus;
use common::messages::CockpitToLinkage;
use gilrs::Gilrs;

#[repr(u8)]
enum EventType {
    ButtonChanged = 0,
    AxisChanged = 1,
    Connected = 2,
    Disconnected = 3,
}

pub fn start_event_listener() -> Arc<Mutex<Bus<Option<CockpitToLinkage>>>> {
    let mut gilrs = Gilrs::new().unwrap();

    let bus = Arc::new(Mutex::new(
        Bus::new(std::mem::size_of::<CockpitToLinkage>()),
    ));

    thread::spawn({
        log::debug!("Started gamepad event listener");

        let bus = bus.clone();
        move || {
            loop {
                if let Some(event) = gilrs.next_event() {
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

                // HACK: This is needed until the gilrs crate supports blocking next_event calls.
                //        https://gitlab.com/gilrs-project/gilrs/-/merge_requests/86
                thread::sleep(Duration::from_micros(100));
            }
        }
    });

    bus
}

// HACK: This is needed because of the gilrs crate being neglectant.
fn gamepad_id_into_u8(gamepad_id: gilrs::GamepadId) -> u8 {
    unsafe { std::mem::transmute_copy::<gilrs::GamepadId, usize>(&gamepad_id) as u8 }
}
