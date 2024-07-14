use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use winapi::um::winuser::{
    GetAsyncKeyState, SendInput, INPUT, INPUT_MOUSE, MOUSEEVENTF_MOVE, MOUSEINPUT, VK_LBUTTON,
    VK_RBUTTON,
};

use crate::app_state::AppState;

/// The duration in milliseconds that the mouse controller thread should sleep between iterations.
/// This constant is used to control the frequency at which the mouse input is updated.
pub const THREAD_SLEEP_DURATION_MS: u64 = 16;

/// Starts the mouse controller thread that updates the mouse position based on the active pattern in the `AppState`.
///
/// The mouse controller thread runs in a loop, checking if the left and right mouse buttons are pressed.
/// If so, it retrieves the active pattern from the `AppState` and sends mouse input for each step in the pattern.
/// The thread sleeps for a short duration between iterations to control the frequency of mouse updates.
///
/// If the left and right mouse buttons are not pressed, the thread resets the current step index and the last step time.
pub fn start_mouse_controller(app_state: Arc<Mutex<AppState>>) {
    // Create a new thread that runs the mouse controller loop.
    thread::spawn(move || {
        // Variables to track the pattern progress.
        let mut current_step_index = 0;
        let mut last_step_time = Instant::now();

        loop {
            if should_run() {
                // Lock the app state and retrieve the current pattern.
                let app_state = app_state.lock().unwrap();
                let pattern = &app_state.active_pattern;

                // Retrieve the current step and send mouse input for the current step.
                let step = &pattern[current_step_index];

                // Send the mouse input for the current step.
                send_mouse_input(step.dx, step.dy);

                // Moving to the next step in the pattern if the current step is not the last
                // and the duration of the current step has elapsed.
                if current_step_index + 1 < pattern.len()
                    && last_step_time.elapsed() >= Duration::from_millis(step.duration)
                {
                    // Increment the current step index and reset the last step time.
                    current_step_index += 1;
                    last_step_time = Instant::now();
                }
            } else {
                current_step_index = 0;
                last_step_time = Instant::now();
            }

            // Sleep for a short duration to control the frequency of mouse updates.
            thread::sleep(Duration::from_millis(THREAD_SLEEP_DURATION_MS));
        }
    });
}

/// Sends mouse input to the operating system.
///
/// This function creates a `MOUSEINPUT` struct with the provided `dx` and `dy` values, and then
/// sends the input to the operating system using the `SendInput` function.
///
/// # Arguments
/// * `dx` - The horizontal mouse movement in pixels.
/// * `dy` - The vertical mouse movement in pixels.
fn send_mouse_input(dx: i32, dy: i32) {
    let mouse_input = init_mouse_input(dx, dy);

    let input = INPUT {
        type_: INPUT_MOUSE,
        u: unsafe { std::mem::zeroed() },
    };

    let mut input_union = input;
    unsafe {
        *input_union.u.mi_mut() = mouse_input;
    }

    unsafe {
        SendInput(1, &mut input_union, std::mem::size_of::<INPUT>() as i32);
    }
}

/// Initializes a `MOUSEINPUT` struct with the provided horizontal and vertical movement values.
///
/// This function creates a `MOUSEINPUT` struct with the specified `dx` and `dy` values,
/// and sets the `dwFlags` field to `MOUSEEVENTF_MOVE` to indicate that the input represents mouse movement.
///
/// # Arguments
/// * `dx` - The horizontal mouse movement in pixels.
/// * `dy` - The vertical mouse movement in pixels.
///
/// # Returns
/// A `MOUSEINPUT` struct with the specified movement values and the `MOUSEEVENTF_MOVE` flag set.
fn init_mouse_input(dx: i32, dy: i32) -> MOUSEINPUT {
    let mut mouse_input: MOUSEINPUT = unsafe { std::mem::zeroed() };
    mouse_input.dx = dx;
    mouse_input.dy = dy;
    mouse_input.dwFlags = MOUSEEVENTF_MOVE;
    mouse_input
}

fn should_run() -> bool {
    (unsafe { GetAsyncKeyState(VK_LBUTTON) } & 0x8000u16 as i16 != 0)
        && (unsafe { GetAsyncKeyState(VK_RBUTTON) } & 0x8000u16 as i16 != 0)
}
