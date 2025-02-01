use std::{collections::HashSet, thread, time::Duration};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState, GetKeyNameTextA, MapVirtualKeyA,
};

fn get_key_name(vk_code: u32) -> String {
    let scan_code = unsafe {
        MapVirtualKeyA(
            vk_code as u32,
            windows::Win32::UI::Input::KeyboardAndMouse::MAP_VIRTUAL_KEY_TYPE(0),
        )
    };
    if scan_code == 0 {
        return format!("Unknown({})", vk_code);
    }

    let mut buffer = [0u8; 32];
    let result = unsafe { GetKeyNameTextA((scan_code as i32) << 16, &mut buffer) };

    if result > 0 {
        String::from_utf8_lossy(&buffer[..result as usize]).to_string()
    } else {
        format!("Unknown({})", vk_code)
    }
}

fn main() {
    println!("Listening for key presses...");

    let mut pressed_keys = HashSet::new();

    loop {
        for key in 0..256 {
            let state = unsafe { GetAsyncKeyState(key) };
            let is_pressed = (state as u16) & 0x8000 != 0;

            if is_pressed && !pressed_keys.contains(&key) {
                let key_name = get_key_name(key as u32);
                println!("Key pressed: {}", key_name);
                pressed_keys.insert(key);
            } else if !is_pressed {
                pressed_keys.remove(&key);
            }
        }
        thread::sleep(Duration::from_millis(10));
    }
}
