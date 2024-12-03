use windows::{
    core::PCWSTR,
    Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_OK},
};

pub trait IntoPCWSTR {
    fn into_pcwstr(self) -> PCWSTR;
}

impl IntoPCWSTR for &str {
    fn into_pcwstr(self) -> PCWSTR {
        let mut encoded = self.encode_utf16().chain([0u16]).collect::<Vec<u16>>();

        PCWSTR(encoded.as_mut_ptr())
    }
}

impl IntoPCWSTR for String {
    fn into_pcwstr(self) -> PCWSTR {
        let mut encoded = self.encode_utf16().chain([0u16]).collect::<Vec<u16>>();

        PCWSTR(encoded.as_mut_ptr())
    }
}

pub fn message_box(lptext: &str, lpcaption: &str) {
    unsafe {
        MessageBoxW(None, lptext.into_pcwstr(), lpcaption.into_pcwstr(), MB_OK);
    }
}
