#![allow(non_camel_case_types)]

//! Raw bindings for [libuiohook](https://github.com/kwhat/libuiohook).

// DOCS(Unavailable): Document all items.
//
// fn docs: https://github.com/kwhat/libuiohook/tree/1.2/man
//
// DOCS(Unavailable): Port all examples

#[rustfmt::skip]
mod inner {

use core::{
    ffi::{c_char, c_uchar, c_uint, c_int, c_long},
    fmt, hash
};

/* Begin Error Codes */

// General error flags.
pub const UIOHOOK_SUCCESS                           : c_int = 0x00;
pub const UIOHOOK_FAILURE                           : c_int = 0x01;

// System level errors.
pub const UIOHOOK_ERROR_OUT_OF_MEMORY               : c_int = 0x02;

// Unix specific errors.
pub const UIOHOOK_ERROR_X_OPEN_DISPLAY              : c_int = 0x20;
pub const UIOHOOK_ERROR_X_RECORD_NOT_FOUND          : c_int = 0x21;
pub const UIOHOOK_ERROR_X_RECORD_ALLOC_RANGE        : c_int = 0x22;
pub const UIOHOOK_ERROR_X_RECORD_CREATE_CONTEXT     : c_int = 0x23;
pub const UIOHOOK_ERROR_X_RECORD_ENABLE_CONTEXT     : c_int = 0x24;
pub const UIOHOOK_ERROR_X_RECORD_GET_CONTEXT        : c_int = 0x25;

// Windows specific errors.
pub const UIOHOOK_ERROR_SET_WINDOWS_HOOK_EX         : c_int = 0x30;
pub const UIOHOOK_ERROR_GET_MODULE_HANDLE           : c_int = 0x31;

// Darwin specific errors.
pub const UIOHOOK_ERROR_AXAPI_DISABLED              : c_int = 0x40;
pub const UIOHOOK_ERROR_CREATE_EVENT_PORT           : c_int = 0x41;
pub const UIOHOOK_ERROR_CREATE_RUN_LOOP_SOURCE      : c_int = 0x42;
pub const UIOHOOK_ERROR_GET_RUNLOOP                 : c_int = 0x43;
pub const UIOHOOK_ERROR_CREATE_OBSERVER             : c_int = 0x44;

/* End Error Codes */
/* Begin Log Levels and Function Prototype */

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum log_level {
    LOG_LEVEL_DEBUG = 1,
    LOG_LEVEL_INFO,
    LOG_LEVEL_WARN,
    LOG_LEVEL_ERROR,
}

/// Logger callback function prototype.
pub type logger_t = extern "C" fn(c_uint, *const c_char, ...) -> bool;

/* End Log Levels and Function Prototype */
/* Begin Virtual Event Types and Data Structures */

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum event_type {
    EVENT_HOOK_ENABLED = 1,
    EVENT_HOOK_DISABLED,
    EVENT_KEY_TYPED,
    EVENT_KEY_PRESSED,
    EVENT_KEY_RELEASED,
    EVENT_MOUSE_CLICKED,
    EVENT_MOUSE_PRESSED,
    EVENT_MOUSE_RELEASED,
    EVENT_MOUSE_MOVED,
    EVENT_MOUSE_DRAGGED,
    EVENT_MOUSE_WHEEL,
}

// TODO(Unavailable): Implement `{Partial}Ord` in terms of `number`?

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct screen_data {
    pub number: u8,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
}

macro_rules! _keyboard_event_data {
    (@pub struct $name:ident) => {
        // TODO(Unavailable): Implement `{Partial}Ord` in terms of `keycode`?

        #[repr(C)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct $name {
            pub keycode: u16,
            pub rawcode: u16,
            pub keychar: u16,
        }
    };
}

_keyboard_event_data!(@pub struct keyboard_event_data    );
_keyboard_event_data!(@pub struct key_pressed_event_data );
_keyboard_event_data!(@pub struct key_released_event_data);
_keyboard_event_data!(@pub struct key_typed_event_data   );

macro_rules! _mouse_event_data {
    (@pub struct $name:ident) => {
        // TODO(Unavailable): Implement `{Partial}Ord` in terms of `button`?

        #[repr(C)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct $name {
            pub button: u16,
            pub clicks: u16,
            pub x: i16,
            pub y: i16,
        }
    };
}

_mouse_event_data!(@pub struct mouse_event_data         );
_mouse_event_data!(@pub struct mouse_pressed_event_data );
_mouse_event_data!(@pub struct mouse_released_event_data);
_mouse_event_data!(@pub struct mouse_clicked_event_data );

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct mouse_wheel_event_data {
    pub clicks: u16,
    pub x: i16,
    pub y: i16,
    pub r#type: u8,
    pub amount: u16,
    pub rotation: i16,
    pub direction: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union input_event_data {
    pub keyboard: keyboard_event_data,
    pub mouse: mouse_event_data,
    pub wheel: mouse_wheel_event_data,
}

// TODO(Unavailable): Implement `{Partial}Ord` in terms of `time`?
//
// DOCS(Unavailable): Safety concerns around `r#type` and `data` mismatch.

#[repr(C)]
#[derive(Clone)]
pub struct uiohook_event {
    pub r#type: event_type,
    pub time: u64,
    pub mask: u16,
    pub reserved: u16,
    pub data: input_event_data,
}

impl fmt::Debug for uiohook_event {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use event_type as et;

        let mut strt = f.debug_struct("uiohook_event");

        strt.field("time", &self.time)
            .field("mask", &self.mask)
            .field("reserved", &self.reserved);

        let data: &dyn fmt::Debug = match self.r#type {
            et::EVENT_HOOK_ENABLED | et::EVENT_HOOK_DISABLED => &(),

            et::EVENT_KEY_TYPED
            | et::EVENT_KEY_PRESSED
            | et::EVENT_KEY_RELEASED => unsafe { &self.data.keyboard }

            et::EVENT_MOUSE_CLICKED
            | et::EVENT_MOUSE_PRESSED
            | et::EVENT_MOUSE_RELEASED
            | et::EVENT_MOUSE_MOVED
            | et::EVENT_MOUSE_DRAGGED => unsafe { &self.data.mouse }

            et::EVENT_MOUSE_WHEEL => unsafe { &self.data.wheel }
        };

        strt.field("data", data).finish()
    }
}

impl PartialEq for uiohook_event {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        use event_type as et;

        if self.r#type != other.r#type
            || self.time != other.time
            || self.mask != other.mask
            || self.reserved != other.reserved
        {
            return false;
        };

        match self.r#type {
            et::EVENT_HOOK_ENABLED | et::EVENT_HOOK_DISABLED => true,

            et::EVENT_KEY_TYPED
            | et::EVENT_KEY_PRESSED
            | et::EVENT_KEY_RELEASED => {
                unsafe { self.data.keyboard == other.data.keyboard }
            }

            et::EVENT_MOUSE_CLICKED
            | et::EVENT_MOUSE_PRESSED
            | et::EVENT_MOUSE_RELEASED
            | et::EVENT_MOUSE_MOVED
            | et::EVENT_MOUSE_DRAGGED => {
                unsafe { self.data.mouse == other.data.mouse }
            }

            et::EVENT_MOUSE_WHEEL => {
                unsafe { self.data.wheel == other.data.wheel }
            }
        }
    }
}

impl Eq for uiohook_event {}

impl hash::Hash for uiohook_event {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        use event_type as et;

        self.r#type.hash(state);
        self.time.hash(state);
        self.mask.hash(state);
        self.reserved.hash(state);

        match self.r#type {
            et::EVENT_HOOK_ENABLED | et::EVENT_HOOK_DISABLED => {},

            et::EVENT_KEY_TYPED
            | et::EVENT_KEY_PRESSED
            | et::EVENT_KEY_RELEASED => unsafe { self.data.keyboard }.hash(state),

            et::EVENT_MOUSE_CLICKED
            | et::EVENT_MOUSE_PRESSED
            | et::EVENT_MOUSE_RELEASED
            | et::EVENT_MOUSE_MOVED
            | et::EVENT_MOUSE_DRAGGED => unsafe { self.data.mouse }.hash(state),

            et::EVENT_MOUSE_WHEEL => unsafe { self.data.wheel }.hash(state),
        };
    }
}

pub type dispatcher_t = extern "C" fn(*const uiohook_event);

/* End Virtual Event Types and Data Structures */
/* Begin Virtual Key Codes */

pub const VC_ESCAPE                                 : u16 = 0x0001;

// Begin Function Keys
pub const VC_F1                                     : u16 = 0x003B;
pub const VC_F2                                     : u16 = 0x003C;
pub const VC_F3                                     : u16 = 0x003D;
pub const VC_F4                                     : u16 = 0x003E;
pub const VC_F5                                     : u16 = 0x003F;
pub const VC_F6                                     : u16 = 0x0040;
pub const VC_F7                                     : u16 = 0x0041;
pub const VC_F8                                     : u16 = 0x0042;
pub const VC_F9                                     : u16 = 0x0043;
pub const VC_F10                                    : u16 = 0x0044;
pub const VC_F11                                    : u16 = 0x0057;
pub const VC_F12                                    : u16 = 0x0058;

pub const VC_F13                                    : u16 = 0x005B;
pub const VC_F14                                    : u16 = 0x005C;
pub const VC_F15                                    : u16 = 0x005D;
pub const VC_F16                                    : u16 = 0x0063;
pub const VC_F17                                    : u16 = 0x0064;
pub const VC_F18                                    : u16 = 0x0065;
pub const VC_F19                                    : u16 = 0x0066;
pub const VC_F20                                    : u16 = 0x0067;
pub const VC_F21                                    : u16 = 0x0068;
pub const VC_F22                                    : u16 = 0x0069;
pub const VC_F23                                    : u16 = 0x006A;
pub const VC_F24                                    : u16 = 0x006B;
// End Function Keys

// Begin Alphanumeric Zone
pub const VC_BACKQUOTE                              : u16 = 0x0029;

pub const VC_1                                      : u16 = 0x0002;
pub const VC_2                                      : u16 = 0x0003;
pub const VC_3                                      : u16 = 0x0004;
pub const VC_4                                      : u16 = 0x0005;
pub const VC_5                                      : u16 = 0x0006;
pub const VC_6                                      : u16 = 0x0007;
pub const VC_7                                      : u16 = 0x0008;
pub const VC_8                                      : u16 = 0x0009;
pub const VC_9                                      : u16 = 0x000A;
pub const VC_0                                      : u16 = 0x000B;

pub const VC_MINUS                                  : u16 = 0x000C; // '-'
pub const VC_EQUALS                                 : u16 = 0x000D; // '='
pub const VC_BACKSPACE                              : u16 = 0x000E;

pub const VC_TAB                                    : u16 = 0x000F;
pub const VC_CAPS_LOCK                              : u16 = 0x003A;

pub const VC_A                                      : u16 = 0x001E;
pub const VC_B                                      : u16 = 0x0030;
pub const VC_C                                      : u16 = 0x002E;
pub const VC_D                                      : u16 = 0x0020;
pub const VC_E                                      : u16 = 0x0012;
pub const VC_F                                      : u16 = 0x0021;
pub const VC_G                                      : u16 = 0x0022;
pub const VC_H                                      : u16 = 0x0023;
pub const VC_I                                      : u16 = 0x0017;
pub const VC_J                                      : u16 = 0x0024;
pub const VC_K                                      : u16 = 0x0025;
pub const VC_L                                      : u16 = 0x0026;
pub const VC_M                                      : u16 = 0x0032;
pub const VC_N                                      : u16 = 0x0031;
pub const VC_O                                      : u16 = 0x0018;
pub const VC_P                                      : u16 = 0x0019;
pub const VC_Q                                      : u16 = 0x0010;
pub const VC_R                                      : u16 = 0x0013;
pub const VC_S                                      : u16 = 0x001F;
pub const VC_T                                      : u16 = 0x0014;
pub const VC_U                                      : u16 = 0x0016;
pub const VC_V                                      : u16 = 0x002F;
pub const VC_W                                      : u16 = 0x0011;
pub const VC_X                                      : u16 = 0x002D;
pub const VC_Y                                      : u16 = 0x0015;
pub const VC_Z                                      : u16 = 0x002C;

pub const VC_OPEN_BRACKET                           : u16 = 0x001A; // '['
pub const VC_CLOSE_BRACKET                          : u16 = 0x001B; // ']'
pub const VC_BACK_SLASH                             : u16 = 0x002B; // '\'

pub const VC_SEMICOLON                              : u16 = 0x0027; // ';'
pub const VC_QUOTE                                  : u16 = 0x0028;
pub const VC_ENTER                                  : u16 = 0x001C;

pub const VC_COMMA                                  : u16 = 0x0033; // ','
pub const VC_PERIOD                                 : u16 = 0x0034; // '.'
pub const VC_SLASH                                  : u16 = 0x0035; // '/'

pub const VC_SPACE                                  : u16 = 0x0039;
// End Alphanumeric Zone

pub const VC_PRINTSCREEN                            : u16 = 0x0E37;
pub const VC_SCROLL_LOCK                            : u16 = 0x0046;
pub const VC_PAUSE                                  : u16 = 0x0E45;

pub const VC_LESSER_GREATER                         : u16 = 0x0E46; // '<', '>', '|' on qwertz layout

// Begin Edit Key Zone
pub const VC_INSERT                                 : u16 = 0x0E52;
pub const VC_DELETE                                 : u16 = 0x0E53;
pub const VC_HOME                                   : u16 = 0x0E47;
pub const VC_END                                    : u16 = 0x0E4F;
pub const VC_PAGE_UP                                : u16 = 0x0E49;
pub const VC_PAGE_DOWN                              : u16 = 0x0E51;
// End Edit Key Zone

// Begin Cursor Key Zone
pub const VC_UP                                     : u16 = 0xE048;
pub const VC_LEFT                                   : u16 = 0xE04B;
pub const VC_CLEAR                                  : u16 = 0xE04C;
pub const VC_RIGHT                                  : u16 = 0xE04D;
pub const VC_DOWN                                   : u16 = 0xE050;
// End Cursor Key Zone

// Begin Numeric Zone
pub const VC_NUM_LOCK                               : u16 = 0x0045;
pub const VC_KP_DIVIDE                              : u16 = 0x0E35;
pub const VC_KP_MULTIPLY                            : u16 = 0x0037;
pub const VC_KP_SUBTRACT                            : u16 = 0x004A;
pub const VC_KP_EQUALS                              : u16 = 0x0E0D;
pub const VC_KP_ADD                                 : u16 = 0x004E;
pub const VC_KP_ENTER                               : u16 = 0x0E1C;
pub const VC_KP_SEPARATOR                           : u16 = 0x0053;

pub const VC_KP_1                                   : u16 = 0x004F;
pub const VC_KP_2                                   : u16 = 0x0050;
pub const VC_KP_3                                   : u16 = 0x0051;
pub const VC_KP_4                                   : u16 = 0x004B;
pub const VC_KP_5                                   : u16 = 0x004C;
pub const VC_KP_6                                   : u16 = 0x004D;
pub const VC_KP_7                                   : u16 = 0x0047;
pub const VC_KP_8                                   : u16 = 0x0048;
pub const VC_KP_9                                   : u16 = 0x0049;
pub const VC_KP_0                                   : u16 = 0x0052;

pub const VC_KP_END                                 : u16 = 0xEE00 | VC_KP_1;
pub const VC_KP_DOWN                                : u16 = 0xEE00 | VC_KP_2;
pub const VC_KP_PAGE_DOWN                           : u16 = 0xEE00 | VC_KP_3;
pub const VC_KP_LEFT                                : u16 = 0xEE00 | VC_KP_4;
pub const VC_KP_CLEAR                               : u16 = 0xEE00 | VC_KP_5;
pub const VC_KP_RIGHT                               : u16 = 0xEE00 | VC_KP_6;
pub const VC_KP_HOME                                : u16 = 0xEE00 | VC_KP_7;
pub const VC_KP_UP                                  : u16 = 0xEE00 | VC_KP_8;
pub const VC_KP_PAGE_UP                             : u16 = 0xEE00 | VC_KP_9;
pub const VC_KP_INSERT                              : u16 = 0xEE00 | VC_KP_0;
pub const VC_KP_DELETE                              : u16 = 0xEE00 | VC_KP_SEPARATOR;
// End Numeric Zone

// Begin Modifier and Control Keys
pub const VC_SHIFT_L                                : u16 = 0x002A;
pub const VC_SHIFT_R                                : u16 = 0x0036;
pub const VC_CONTROL_L                              : u16 = 0x001D;
pub const VC_CONTROL_R                              : u16 = 0x0E1D;
pub const VC_ALT_L                                  : u16 = 0x0038; // Option or Alt Key
pub const VC_ALT_R                                  : u16 = 0x0E38; // Option or Alt Key
pub const VC_META_L                                 : u16 = 0x0E5B; // Windows or Command Key
pub const VC_META_R                                 : u16 = 0x0E5C; // Windows or Command Key
pub const VC_CONTEXT_MENU                           : u16 = 0x0E5D;
// End Modifier and Control Keys

// Begin Media Control Keys
pub const VC_POWER                                  : u16 = 0xE05E;
pub const VC_SLEEP                                  : u16 = 0xE05F;
pub const VC_WAKE                                   : u16 = 0xE063;

pub const VC_MEDIA_PLAY                             : u16 = 0xE022;
pub const VC_MEDIA_STOP                             : u16 = 0xE024;
pub const VC_MEDIA_PREVIOUS                         : u16 = 0xE010;
pub const VC_MEDIA_NEXT                             : u16 = 0xE019;
pub const VC_MEDIA_SELECT                           : u16 = 0xE06D;
pub const VC_MEDIA_EJECT                            : u16 = 0xE02C;

pub const VC_VOLUME_MUTE                            : u16 = 0xE020;
pub const VC_VOLUME_UP                              : u16 = 0xE030;
pub const VC_VOLUME_DOWN                            : u16 = 0xE02E;

pub const VC_APP_MAIL                               : u16 = 0xE06C;
pub const VC_APP_CALCULATOR                         : u16 = 0xE021;
pub const VC_APP_MUSIC                              : u16 = 0xE03C;
pub const VC_APP_PICTURES                           : u16 = 0xE064;

pub const VC_BROWSER_SEARCH                         : u16 = 0xE065;
pub const VC_BROWSER_HOME                           : u16 = 0xE032;
pub const VC_BROWSER_BACK                           : u16 = 0xE06A;
pub const VC_BROWSER_FORWARD                        : u16 = 0xE069;
pub const VC_BROWSER_STOP                           : u16 = 0xE068;
pub const VC_BROWSER_REFRESH                        : u16 = 0xE067;
pub const VC_BROWSER_FAVORITES                      : u16 = 0xE066;
// End Media Control Keys

// Begin Japanese Language Keys
pub const VC_KATAKANA                               : u16 = 0x0070;
pub const VC_UNDERSCORE                             : u16 = 0x0073;
pub const VC_FURIGANA                               : u16 = 0x0077;
pub const VC_KANJI                                  : u16 = 0x0079;
pub const VC_HIRAGANA                               : u16 = 0x007B;
pub const VC_YEN                                    : u16 = 0x007D;
pub const VC_KP_COMMA                               : u16 = 0x007E;
// End Japanese Language Keys
    
// Begin Sun keyboards
pub const VC_SUN_HELP                               : u16 = 0xFF75;

pub const VC_SUN_STOP                               : u16 = 0xFF78;
pub const VC_SUN_PROPS                              : u16 = 0xFF76;
pub const VC_SUN_FRONT                              : u16 = 0xFF77;
pub const VC_SUN_OPEN                               : u16 = 0xFF74;
pub const VC_SUN_FIND                               : u16 = 0xFF7E;
pub const VC_SUN_AGAIN                              : u16 = 0xFF79;
pub const VC_SUN_UNDO                               : u16 = 0xFF7A;
pub const VC_SUN_COPY                               : u16 = 0xFF7C;
pub const VC_SUN_INSERT                             : u16 = 0xFF7D;
pub const VC_SUN_CUT                                : u16 = 0xFF7B;
// End Sun keyboards

pub const VC_UNDEFINED                              : u16 = 0x0000; // KeyCode Unknown

pub const CHAR_UNDEFINED                            : u16 = 0xFFFF; // CharCode Unknown

/* End Virtual Key Codes */
/* Begin Virtual Modifier Masks */

pub const MASK_SHIFT_L                              : u16 = 1 << 0;
pub const MASK_CTRL_L                               : u16 = 1 << 1;
pub const MASK_META_L                               : u16 = 1 << 2;
pub const MASK_ALT_L                                : u16 = 1 << 3;

pub const MASK_SHIFT_R                              : u16 = 1 << 4;
pub const MASK_CTRL_R                               : u16 = 1 << 5;
pub const MASK_META_R                               : u16 = 1 << 6;
pub const MASK_ALT_R                                : u16 = 1 << 7;

pub const MASK_SHIFT                                : u16 = MASK_SHIFT_L | MASK_SHIFT_R;
pub const MASK_CTRL                                 : u16 = MASK_CTRL_L  | MASK_CTRL_R;
pub const MASK_META                                 : u16 = MASK_META_L  | MASK_META_R;
pub const MASK_ALT                                  : u16 = MASK_ALT_L   | MASK_ALT_R;

pub const MASK_BUTTON1                              : u16 = 1 << 8;
pub const MASK_BUTTON2                              : u16 = 1 << 9;
pub const MASK_BUTTON3                              : u16 = 1 << 10;
pub const MASK_BUTTON4                              : u16 = 1 << 11;
pub const MASK_BUTTON5                              : u16 = 1 << 12;

pub const MASK_NUM_LOCK                             : u16 = 1 << 13;
pub const MASK_CAPS_LOCK                            : u16 = 1 << 14;
pub const MASK_SCROLL_LOCK                          : u16 = 1 << 15;

/* End Virtual Modifier Masks */
/* Begin Virtual Mouse Buttons */

pub const MOUSE_NOBUTTON                            : u16 = 0; // Any Button
pub const MOUSE_BUTTON1                             : u16 = 1; // Left Button
pub const MOUSE_BUTTON2                             : u16 = 2; // Right Button
pub const MOUSE_BUTTON3                             : u16 = 3; // Middle Button
pub const MOUSE_BUTTON4                             : u16 = 4; // Extra Mouse Button
pub const MOUSE_BUTTON5                             : u16 = 5; // Extra Mouse Button

pub const WHEEL_UNIT_SCROLL                         :  u8 = 1;
pub const WHEEL_BLOCK_SCROLL                        :  u8 = 2;

pub const WHEEL_VERTICAL_DIRECTION                  :  u8 = 3;
pub const WHEEL_HORIZONTAL_DIRECTION                :  u8 = 4;

/* End Virtual Mouse Buttons */

#[link(name = "uiohook")]
extern "C" {
    /// Set the logger callback functions.
    pub fn hook_set_logger_proc(logger_proc: logger_t);

    /// Send a virtual event back to the system.
    pub fn hook_post_event(event: *const uiohook_event);

    /// Set the event callback function.
    pub fn hook_set_dispatch_proc(dispatch_proc: dispatcher_t);

    /// Insert the event hook.
    pub fn hook_run() -> c_int;

    /// Withdraw the event hook.
    pub fn hook_stop() -> c_int;

    /// Retrieves an array of screen data for each available monitor.
    pub fn hook_create_screen_info(count: *mut c_uchar) -> *const screen_data;

    /// Retrieves the keyboard auto repeat rate.
    pub fn hook_get_auto_repeat_rate() -> c_long;

    /// Retrieves the keyboard auto repeat delay.
    pub fn hook_get_auto_repeat_delay() -> c_long;

    /// Retrieves the mouse acceleration multiplier.
    pub fn hook_get_pointer_acceleration_multiplier() -> c_long;

    /// Retrieves the mouse acceleration threshold.
    pub fn hook_get_pointer_acceleration_threshold() -> c_long;

    /// Retrieves the mouse sensitivity.
    pub fn hook_get_pointer_sensitivity() -> c_long;

    /// Retrieves the double/triple click interval.
    pub fn hook_get_multi_click_time() -> c_long;
}

}

pub use inner::*;
