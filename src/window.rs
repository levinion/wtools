use windows::Win32::{
    Foundation::*,
    UI::{
        Input::KeyboardAndMouse::{SendInput, INPUT, INPUT_MOUSE},
        WindowsAndMessaging::*,
    },
};

pub struct Window {
    pub hwnd: HWND,
}

impl Window {
    pub fn from_foreground() -> Self {
        let hwnd = unsafe { GetForegroundWindow() };
        Self { hwnd }
    }

    pub fn from_top_level() -> Self {
        let hwnd = unsafe { GetTopWindow(None).unwrap() };
        Self { hwnd }
    }

    pub fn info(&self) -> WINDOWINFO {
        unsafe {
            let mut info = WINDOWINFO {
                cbSize: core::mem::size_of::<WINDOWINFO>() as u32,
                ..Default::default()
            };
            GetWindowInfo(self.hwnd, &mut info).unwrap();
            info
        }
    }

    pub fn name(&self) -> String {
        unsafe {
            let mut text: [u16; 512] = [0; 512];
            let len = GetWindowTextW(self.hwnd, &mut text);
            String::from_utf16_lossy(&text[..len as usize])
        }
    }

    pub fn minimize(&self) {
        unsafe {
            CloseWindow(self.hwnd).unwrap();
        }
    }

    pub fn close(&self) {
        unsafe {
            PostMessageW(self.hwnd, WM_SYSCOMMAND, WPARAM(SC_CLOSE as _), None).unwrap();
        }
    }

    pub fn maximize(&self) {
        unsafe {
            PostMessageW(self.hwnd, WM_SYSCOMMAND, WPARAM(SC_MAXIMIZE as _), None).unwrap();
        }
    }

    pub fn restore(&self) {
        unsafe {
            PostMessageW(self.hwnd, WM_SYSCOMMAND, WPARAM(SC_RESTORE as _), None).unwrap();
        }
    }

    pub fn fullscreen(&self) {
        unsafe {
            PostMessageW(self.hwnd, WM_SYSCOMMAND, WPARAM(SHOW_FULLSCREEN as _), None).unwrap();
        }
    }

    pub fn is_maximized(&self) -> bool {
        unsafe { IsZoomed(self.hwnd).into() }
    }

    pub fn is_minimized(&self) -> bool {
        unsafe { IsIconic(self.hwnd).into() }
    }

    pub fn maximize_or_restore(&self) {
        match self.is_maximized() {
            true => self.restore(),
            false => self.maximize(),
        }
    }

    // this function not work...
    pub fn focus(&self) {
        let event = [INPUT {
            r#type: INPUT_MOUSE,
            ..Default::default()
        }];

        unsafe {
            SendInput(&event, size_of::<INPUT>() as i32);
            let _ = SetWindowPos(
                self.hwnd,
                HWND_TOP,
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW,
            );
            SetForegroundWindow(self.hwnd).unwrap();
        }
    }
}
