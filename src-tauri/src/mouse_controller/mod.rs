use winapi::um::winuser::{SendInput, INPUT, INPUT_MOUSE, MOUSEEVENTF_MOVE, MOUSEINPUT};

/// Sends a mouse input event to the system.
///
/// This function takes the horizontal and vertical movement deltas as `i32` values and sends a mouse input event to the system
/// using the `SendInput` function from the `winapi` crate.
///
/// The `init_mouse_input` function is used to create a `MOUSEINPUT` struct with the provided movement deltas, which is then
/// converted to an `INPUT` struct and passed to `SendInput`.
///
/// # Arguments
/// * `dx` - The horizontal movement delta.
/// * `dy` - The vertical movement delta.
pub fn send_mouse_input(dx: i32, dy: i32) {
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

/// Initializes a `MOUSEINPUT` struct with the provided horizontal and vertical movement deltas.
///
/// This function is used to create a `MOUSEINPUT` struct with the given `dx` and `dy` values, which can then be used to send a mouse input event to the system using the `send_mouse_input` function.
///
/// # Arguments
/// * `dx` - The horizontal movement delta.
/// * `dy` - The vertical movement delta.
///
/// # Returns
/// A `MOUSEINPUT` struct with the provided movement deltas.
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
