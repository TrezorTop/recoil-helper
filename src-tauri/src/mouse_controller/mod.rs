use winapi::um::winuser::{SendInput, INPUT, INPUT_MOUSE, MOUSEEVENTF_MOVE, MOUSEINPUT};

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
