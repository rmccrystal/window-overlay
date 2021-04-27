use imgui::{ImStr, StyleColor, StyleVar};
use crate::imgui::RenderState;
use imgui::*;
use winapi::um::winuser::GetAsyncKeyState;
use crate::imgui::util::get_keys_down;
use winutil::{InputEventListener, VK_ESCAPE};
use winutil::Event::KeyDown;

enum KeybindSelectState {
    Idle,
    Listening(InputEventListener),
}

pub fn keybind_select(ui: &::imgui::Ui, render_state: &mut RenderState, title: &ImStr, key: &mut i32) {
    use KeybindSelectState::*;
    let state = render_state.get(&format!("keybind:{:X}", key as *mut _ as usize), KeybindSelectState::Idle);

    let title = if let Listening { .. } = state {
        im_str!("Press key").to_owned()
    } else {
        im_str!("{}: {}", title, vkey_to_string(*key))
    };

    if ui.button(&title, [160.0, 20.0]) {
        *state = match *state {
            Idle => Listening(InputEventListener::new()),
            Listening { .. } => Idle
        }
    }
    if let Listening(lis) = state {
        let mut handled = false;
        for ev in lis {
            if let KeyDown(k) = ev {
                if k == VK_ESCAPE {
                    handled = true;
                    break;
                }
                *key = k;
                handled = true;
                break;
            }
        }

        if handled {
            *state = Idle;
        }
    }
}

/// Translate keycode into text, and also filter out keys not usable (arrows, escape)
fn vkey_to_string(key: i32) -> String {
    use winapi::um::winuser::*;
    let mut text = match key {
        VK_LBUTTON => "LBUTTON",
        VK_RBUTTON => "RBUTTON",
        VK_MBUTTON => "MBUTTON",
        VK_XBUTTON1 => "XBUTTON1",
        VK_XBUTTON2 => "XBUTTON2",
        VK_BACK => "BACK",
        VK_TAB => "TAB",
        VK_CLEAR => "CLEAR",
        VK_RETURN => "RETURN",
        VK_SHIFT => "SHIFT",
        VK_CONTROL => "CONTROL",
        VK_MENU => "MENU",
        VK_PAUSE => "PAUSE",
        VK_CAPITAL => "CAPITAL",
        VK_SPACE => "SPACE",
        VK_PRIOR => "PGUP",
        VK_NEXT => "PGDOWN",
        VK_END => "END",
        VK_HOME => "HOME",
        VK_PRINT => "PRINT",
        VK_EXECUTE => "EXECUTE",
        VK_SNAPSHOT => "SNAPSHOT",
        VK_INSERT => "INSERT",
        VK_DELETE => "DELETE",
        VK_HELP => "HELP",
        0x30 => "0",
        0x31 => "1",
        0x32 => "2",
        0x33 => "3",
        0x34 => "4",
        0x35 => "5",
        0x36 => "6",
        0x37 => "7",
        0x38 => "8",
        0x39 => "9",
        0x41 => "A",
        0x42 => "B",
        0x43 => "C",
        0x44 => "D",
        0x45 => "E",
        0x46 => "F",
        0x47 => "G",
        0x48 => "H",
        0x49 => "I",
        0x4A => "J",
        0x4B => "K",
        0x4C => "L",
        0x4D => "M",
        0x4E => "N",
        0x4F => "O",
        0x50 => "P",
        0x51 => "Q",
        0x52 => "R",
        0x53 => "S",
        0x54 => "T",
        0x55 => "U",
        0x56 => "V",
        0x57 => "W",
        0x58 => "X",
        0x59 => "Y",
        0x5A => "Z",
        VK_LWIN => "LWIN",
        VK_RWIN => "RWIN",
        VK_APPS => "APPS",
        VK_SLEEP => "SLEEP",
        VK_NUMPAD0 => "NUMPAD0",
        VK_NUMPAD1 => "NUMPAD1",
        VK_NUMPAD2 => "NUMPAD2",
        VK_NUMPAD3 => "NUMPAD3",
        VK_NUMPAD4 => "NUMPAD4",
        VK_NUMPAD5 => "NUMPAD5",
        VK_NUMPAD6 => "NUMPAD6",
        VK_NUMPAD7 => "NUMPAD7",
        VK_NUMPAD8 => "NUMPAD8",
        VK_NUMPAD9 => "NUMPAD9",
        VK_MULTIPLY => "MULTIPLY",
        VK_ADD => "ADD",
        VK_SEPARATOR => "SEPARATOR",
        VK_SUBTRACT => "SUBTRACT",
        VK_DECIMAL => "DECIMAL",
        VK_DIVIDE => "DIVIDE",
        VK_F1 => "F1",
        VK_F2 => "F2",
        VK_F3 => "F3",
        VK_F4 => "F4",
        VK_F5 => "F5",
        VK_F6 => "F6",
        VK_F7 => "F7",
        VK_F8 => "F8",
        VK_F9 => "F9",
        VK_F10 => "F10",
        VK_F11 => "F11",
        VK_F12 => "F12",
        VK_NUMLOCK => "NUMLOCK",
        VK_SCROLL => "SCROLL",
        _ => "",  // we can't have a ref making a number to a string so we have to do it somwhere else
    }.to_string();
    if text.is_empty() {
        text = key.to_string();
    }
    text
}