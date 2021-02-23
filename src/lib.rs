#[cfg(windows)]
extern crate winapi;

use std::io::Error;
use std::iter::once;
use std::mem;
use std::ptr::null_mut;
use winapi::ctypes::*;
use winapi::shared::minwindef::*;
use winapi::shared::windef::HWND;
use winapi::um::commctrl::{
    TaskDialogIndirect, TASKDIALOGCONFIG, TASKDIALOG_BUTTON, TASKDIALOG_COMMON_BUTTON_FLAGS,
    TASKDIALOG_FLAGS,
};
use winapi::um::libloaderapi::GetModuleHandleA;

pub use winapi::um::commctrl::{
    TDCBF_CANCEL_BUTTON, TDCBF_CLOSE_BUTTON, TDCBF_NO_BUTTON, TDCBF_OK_BUTTON, TDCBF_RETRY_BUTTON,
    TDCBF_YES_BUTTON, TDF_ALLOW_DIALOG_CANCELLATION, TDF_CALLBACK_TIMER, TDF_CAN_BE_MINIMIZED,
    TDF_ENABLE_HYPERLINKS, TDF_EXPANDED_BY_DEFAULT, TDF_EXPAND_FOOTER_AREA,
    TDF_NO_DEFAULT_RADIO_BUTTON, TDF_NO_SET_FOREGROUND, TDF_POSITION_RELATIVE_TO_WINDOW,
    TDF_RTL_LAYOUT, TDF_SHOW_MARQUEE_PROGRESS_BAR, TDF_SHOW_PROGRESS_BAR, TDF_SIZE_TO_CONTENT,
    TDF_USE_COMMAND_LINKS, TDF_USE_COMMAND_LINKS_NO_ICON, TDF_USE_HICON_FOOTER, TDF_USE_HICON_MAIN,
    TDF_VERIFICATION_FLAG_CHECKED,
};

pub struct TaskDialogConfig {
    pub parent: HWND,
    pub instance: HMODULE,
    pub flags: TASKDIALOG_FLAGS,
    pub common_buttons: TASKDIALOG_COMMON_BUTTON_FLAGS,
    pub window_title: String,
    pub main_instruction: String,
    pub content: String,
    pub verification_text: String,
    pub expanded_information: String,
    pub expanded_control_text: String,
    pub collapsed_control_text: String,
    pub footer: String,
    pub buttons: Vec<TaskDialogButton>,
    pub default_button: c_int,
    pub radio_buttons: Vec<TaskDialogButton>,
    pub default_radio_buttons: c_int,
}

pub struct TaskDialogButton {
    pub id: c_int,
    pub text: String,
}

pub struct TaskDialogResult {
    pub button_id: i32,
    pub radio_button_id: i32,
    pub checked: bool,
}

/** Create dialog */
#[cfg(windows)]
pub fn show_task_dialog(conf: &TaskDialogConfig) -> Result<TaskDialogResult, Error> {
    let mut result = TaskDialogResult {
        button_id: 0,
        radio_button_id: 0,
        checked: false,
    };
    let ret = unsafe {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;
        /** Convert string to wide string */
        fn to_os_string(text: &String) -> Vec<u16> {
            OsStr::new(text).encode_wide().chain(once(0)).collect()
        }

        // Call GetModuleHandleA on conf.instance is null
        let instance = if conf.instance == null_mut() {
            GetModuleHandleA(std::ptr::null())
        } else {
            conf.instance
        };

        // Some text
        let window_title: Vec<u16> = to_os_string(&conf.window_title);
        let main_instruction: Vec<u16> = to_os_string(&conf.main_instruction);
        let content: Vec<u16> = to_os_string(&conf.content);
        let verification_text: Vec<u16> = to_os_string(&conf.verification_text);
        let expanded_information: Vec<u16> = to_os_string(&conf.expanded_information);
        let expanded_control_text: Vec<u16> = to_os_string(&conf.expanded_control_text);
        let collapsed_control_text: Vec<u16> = to_os_string(&conf.collapsed_control_text);
        let footer: Vec<u16> = to_os_string(&conf.footer);

        // Buttons
        let mut buttons: Vec<TASKDIALOG_BUTTON> = Vec::new();
        let mut btn_text: Vec<Vec<u16>> = Vec::new();
        for v in conf.buttons.iter() {
            btn_text.push(to_os_string(&v.text));
        }
        for i in 0..conf.buttons.len() {
            buttons.push(TASKDIALOG_BUTTON {
                nButtonID: conf.buttons[i].id,
                pszButtonText: btn_text[i].as_ptr(),
            });
        }

        // Radio Buttons
        let mut radio_buttons: Vec<TASKDIALOG_BUTTON> = Vec::new();
        let mut radio_btn_text: Vec<Vec<u16>> = Vec::new();
        for v in conf.radio_buttons.iter() {
            radio_btn_text.push(to_os_string(&v.text));
        }
        for i in 0..conf.radio_buttons.len() {
            radio_buttons.push(TASKDIALOG_BUTTON {
                nButtonID: conf.radio_buttons[i].id,
                pszButtonText: radio_btn_text[i].as_ptr(),
            });
        }

        let config = TASKDIALOGCONFIG {
            cbSize: mem::size_of::<TASKDIALOGCONFIG>() as UINT,
            hwndParent: conf.parent,
            hInstance: instance,
            dwFlags: conf.flags,
            dwCommonButtons: conf.common_buttons,
            pszWindowTitle: window_title.as_ptr(),
            pszMainInstruction: main_instruction.as_ptr(),
            pszContent: content.as_ptr(),
            pszVerificationText: verification_text.as_ptr(),
            pszExpandedInformation: expanded_information.as_ptr(),
            pszExpandedControlText: expanded_control_text.as_ptr(),
            pszCollapsedControlText: collapsed_control_text.as_ptr(),
            pszFooter: footer.as_ptr(),
            cButtons: buttons.len() as UINT,
            pButtons: buttons.as_slice().as_ptr(),
            nDefaultButton: conf.default_button,
            cRadioButtons: radio_buttons.len() as UINT,
            pRadioButtons: radio_buttons.as_slice().as_ptr(),
            nDefaultRadioButton: conf.default_radio_buttons,
            u1: std::mem::zeroed(),
            u2: std::mem::zeroed(),
            pfCallback: None,
            lpCallbackData: 0,
            cxWidth: 0,
        };

        // Result
        let mut btn1: c_int = 0;
        let mut btn2: c_int = 0;
        let mut verify: BOOL = FALSE;
        let dialog_result = TaskDialogIndirect(&config, &mut btn1, &mut btn2, &mut verify);
        result.button_id = btn1;
        result.radio_button_id = btn2;
        result.checked = verify != 0;
        dialog_result
    };
    if ret != 0 {
        Err(Error::last_os_error())
    } else {
        Ok(result)
    }
}

#[cfg(not(windows))]
pub fn ShowTaskDialog(conf: &DialogConfig) -> Result<TaskDialogResult, Error> {
    Err("Only support on Windows")
}