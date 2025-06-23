use log::error;
use winapi::um::winuser::{
    GetAsyncKeyState, SendInput, INPUT, INPUT_MOUSE, MOUSEEVENTF_MOVE, MOUSEINPUT, VK_LBUTTON,
    VK_RBUTTON,
};

use crate::mouse_controller::error::MouseInputError;

/// Sends mouse input to the operating system using the provided `dx` and `dy` values.
///
/// # Arguments
/// * `dx` - Horizontal movement in pixels
/// * `dy` - Vertical movement in pixels
///
/// # Returns
/// * `Ok(())` if the input was sent successfully
/// * `Err(MouseInputError)` if the input could not be sent
pub fn send_mouse_input(dx: i32, dy: i32) -> Result<(), MouseInputError> {
    // Create a MOUSEINPUT struct for the mouse movement
    let mouse_input = MOUSEINPUT {
        dx,
        dy,
        mouseData: 0,
        dwFlags: MOUSEEVENTF_MOVE,
        time: 0,
        dwExtraInfo: 0,
    };

    // Create an INPUT struct with the MOUSEINPUT data
    let mut input = INPUT {
        type_: INPUT_MOUSE,
        u: unsafe { std::mem::zeroed() },
    };

    // Set the MOUSEINPUT struct as the input union
    unsafe {
        *input.u.mi_mut() = mouse_input;
    }

    // Send the input to the operating system
    let input_size = std::mem::size_of::<INPUT>() as i32;
    let result = unsafe { SendInput(1, &mut input, input_size) };

    // Check if the input was sent successfully
    if result != 1 {
        let error_msg = format!("SendInput failed with result: {}", result);
        error!("{}", error_msg);
        return Err(MouseInputError::SendInputFailed);
    }

    Ok(())
}

/// Checks if the mouse controller should be running.
///
/// This function checks if both the left and right mouse buttons are currently pressed.
/// It uses the Windows API function `GetAsyncKeyState` which returns the state of the
/// specified virtual key. The high-order bit (0x8000) is set if the key is currently pressed.
///
/// # Safety
/// This function uses unsafe code to call the Windows API. It's safe to use as long as
/// the Windows API is available and functioning correctly.
///
/// # Returns
/// `true` if both left and right mouse buttons are pressed, `false` otherwise
pub fn should_run() -> bool {
    // Constant for the high-order bit that indicates if a key is pressed
    const KEY_PRESSED_MASK: i16 = 0x8000u16 as i16;

    // Check if the left mouse button is pressed
    let left_pressed = unsafe { 
        GetAsyncKeyState(VK_LBUTTON) & KEY_PRESSED_MASK != 0 
    };

    // Only check the right button if the left is pressed (short-circuit optimization)
    if !left_pressed {
        return false;
    }

    // Check if the right mouse button is pressed
    let right_pressed: bool = unsafe { 
        GetAsyncKeyState(VK_RBUTTON) & KEY_PRESSED_MASK != 0 
    };

    right_pressed
}