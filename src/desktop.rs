use crate::window::Window;

pub fn get_current() -> u32 {
    winvd::get_current_desktop().unwrap().get_index().unwrap() + 1
}

pub fn get_count() -> u32 {
    winvd::get_desktop_count().unwrap()
}

pub fn switch_to(index: u32) {
    winvd::switch_desktop(index - 1).unwrap();
    let window = Window::from_top_level();
    window.focus();
}

pub fn switch_left() {
    let current = get_current();
    if current > 1 {
        switch_to(current - 1);
    }
}

pub fn create_desktop() {
    winvd::create_desktop().unwrap();
}

pub fn switch_right() {
    let current = get_current();
    if get_current() < get_count() {
        switch_to(current + 1)
    }
}
