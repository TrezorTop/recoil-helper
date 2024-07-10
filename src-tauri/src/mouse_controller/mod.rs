use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use winapi::um::winuser::{
    GetAsyncKeyState, SendInput, INPUT, INPUT_MOUSE, MOUSEEVENTF_MOVE, MOUSEINPUT, VK_LBUTTON,
    VK_RBUTTON,
};

use crate::app_state::PatternPart;

pub fn start_mouse_move_thread(closed: Arc<Mutex<bool>>, pattern: Arc<Mutex<Vec<PatternPart>>>) {
    thread::spawn(move || {
        let mut pattern_index = 0;
        let mut timer = Instant::now();
        let thread_sleep_duration = Duration::from_millis(16);

        loop {
            if *closed.lock().unwrap() {
                break;
            }

            let should_run = should_run();

            if should_run {
                let pattern_guard = pattern.lock().unwrap();

                if pattern_index >= pattern_guard.len() {
                    pattern_index = 0;
                }

                let current_pattern_part = &pattern_guard[pattern_index];

                send_mouse_input(current_pattern_part.x, current_pattern_part.y);

                if timer.elapsed() >= Duration::from_millis(current_pattern_part.delay)
                    && pattern_index < pattern_guard.len() - 1
                {
                    pattern_index = (pattern_index + 1) % pattern_guard.len();
                    timer = Instant::now();
                }
            } else {
                pattern_index = 0;
                timer = Instant::now();
            }

            thread::sleep(thread_sleep_duration);
        }
    });
}

fn send_mouse_input(dx: i32, dy: i32) {
    let mouse_input = init_mouse_input(dx, dy);

    // INPUT struct is used to send input events to the system.
    let mut input = INPUT {
        type_: INPUT_MOUSE,
        u: {
            // std::mem::transmute() is used to convert a MOUSEINPUT struct to a MOUSEINPUT union.
            // This is necessary because the INPUT struct is a C union that can represent different types of input events,
            // and Rust does not directly support unions in the same way C does.
            unsafe { std::mem::transmute(mouse_input) }
        },
    };

    unsafe {
        SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32);
    }
}

fn init_mouse_input(dx: i32, dy: i32) -> MOUSEINPUT {
    // This is a common pattern when working with FFI (Foreign Function Interface).
    // std::mem::zeroed() is used to initialize a MOUSEINPUT struct with all fields set to zero (every byte is 0).
    // Since MOUSEINPUT is a C-style struct with primitive integer fields,
    // using std::mem::zeroed() to initialize it to zero is safe and appropriate.
    let mut mouse_input: MOUSEINPUT = unsafe { std::mem::zeroed() };
    // horizontal movement
    mouse_input.dx = dx;
    // vertical movement
    mouse_input.dy = dy;
    // mouse wheel movement
    mouse_input.mouseData = 0;
    // mouse button state
    mouse_input.dwFlags = MOUSEEVENTF_MOVE;
    // time stamp for the event (let system decide)
    mouse_input.time = 0;
    // no extra info
    mouse_input.dwExtraInfo = 0;

    mouse_input
}

fn should_run() -> bool {
    (unsafe { GetAsyncKeyState(VK_LBUTTON) } & 0x8000u16 as i16 != 0)
        && (unsafe { GetAsyncKeyState(VK_RBUTTON) } & 0x8000u16 as i16 != 0)
}
