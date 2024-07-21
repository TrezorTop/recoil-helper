use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use winapi::um::winuser::{
    GetAsyncKeyState, SendInput, INPUT, INPUT_MOUSE, MOUSEEVENTF_MOVE, MOUSEINPUT, VK_LBUTTON,
    VK_RBUTTON,
};

use crate::app_state::AppState;

/// The duration in milliseconds that the mouse controller thread should sleep between updates.
/// This value controls the frequency of mouse input updates.
pub const THREAD_SLEEP_DURATION_MS: u64 = 8;

/// Starts a new thread that controls the mouse based on the active pattern in the `AppState`.
///
/// This function creates a new thread that continuously checks if the left and right mouse buttons are pressed.
/// If the buttons are pressed, it retrieves the current pattern from the `AppState` and sends mouse input
/// based on the steps in the pattern. The thread sleeps for a short duration between updates to control
/// the frequency of mouse input.
///
/// The `app_state` parameter is an `Arc<Mutex<AppState>>` that is used to access the current pattern
/// and update the mouse input.
pub fn start_mouse_controller(app_state: Arc<Mutex<AppState>>) {
    // Create a new thread that runs the mouse controller loop.
    thread::spawn(move || {
        // Variables to track the pattern progress.
        let mut current_step_index = 0;
        let mut last_step_time = Instant::now();

        loop {
            if should_run() {
                // Lock the app state and retrieve the current pattern.
                let app_state = app_state.lock().expect("Failed to lock app state");
                let pattern = &app_state.active_pattern;

                // If the current step index is less than the length of the pattern, send mouse input based on the current step.
                if let Some(step) = pattern.get(current_step_index) {
                    send_mouse_input(step.dx, step.dy);

                    // If the current step is not the last step, and it's been long enough since the last step,
                    // increment the current step index and reset the last step time.
                    if current_step_index + 1 < pattern.len()
                        && last_step_time.elapsed() >= Duration::from_millis(step.duration)
                    {
                        current_step_index += 1;
                        last_step_time = Instant::now();
                    }
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

/// Sends mouse input to the operating system using the provided `dx` and `dy` values.
///
/// This function creates a `MOUSEINPUT` struct with the given `dx` and `dy` values, and then
/// uses the `SendInput` Windows API function to send the mouse input event to the operating
/// system. The `MOUSEINPUT` struct is initialized using the `init_mouse_input` function,
/// and the `INPUT` struct is used to wrap the `MOUSEINPUT` struct before passing it to
/// `SendInput`.
fn send_mouse_input(dx: i32, dy: i32) {
    // Create a MOUSEINPUT struct with the provided dx and dy values.
    let mouse_input = init_mouse_input(dx, dy);

    // Create an INPUT struct with necessary type and zeroed union.
    let mut input = INPUT {
        type_: INPUT_MOUSE,
        u: unsafe { std::mem::zeroed() },
    };
    
    // Set the MOUSEINPUT struct as the input union.
    unsafe {
        *input.u.mi_mut() = mouse_input;
    }

    // Send the input to the operating system.
    unsafe {
        SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32);
    }
}

/// Initializes a `MOUSEINPUT` struct with the provided `dx` and `dy` values, and sets the `dwFlags` field to `MOUSEEVENTF_MOVE`.
///
/// This function is used to create a `MOUSEINPUT` struct that can be used to send mouse input events to the operating system.
/// The `dx` and `dy` parameters represent the relative movement of the mouse cursor in the x and y directions, respectively.
/// The `dwFlags` field is set to `MOUSEEVENTF_MOVE` to indicate that the input is a mouse movement event.
fn init_mouse_input(dx: i32, dy: i32) -> MOUSEINPUT {
    // Create a MOUSEINPUT struct with zeroed values.
    let mut mouse_input: MOUSEINPUT = unsafe { std::mem::zeroed() };
    // Set the dx and dy values.
    mouse_input.dx = dx;
    mouse_input.dy = dy;
    // Set the dwFlags field to MOUSEEVENTF_MOVE to indicate that the input is a mouse movement event.
    mouse_input.dwFlags = MOUSEEVENTF_MOVE;

    // Return the initialized MOUSEINPUT struct.
    mouse_input
}

/// Checks if the left and right mouse buttons are both currently pressed.
///
/// This function checks the state of the left and right mouse buttons using the
/// `GetAsyncKeyState` Windows API function. It returns `true` if both buttons
/// are currently pressed, and `false` otherwise.
fn should_run() -> bool {
    (unsafe { GetAsyncKeyState(VK_LBUTTON) } & 0x8000u16 as i16 != 0)
        && (unsafe { GetAsyncKeyState(VK_RBUTTON) } & 0x8000u16 as i16 != 0)
}
