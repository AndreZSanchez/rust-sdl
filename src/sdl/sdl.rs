use std::mem;
use std::str;

// Setup linking for all targets.
#[cfg(target_os="macos")]
mod mac {
    #[cfg(mac_framework)]
    #[link(name="SDL", kind="framework")]
    extern {}

    #[cfg(not(mac_framework))]
    #[link(name="SDL")]
    extern {}
}

#[cfg(not(target_os="macos"))]
mod others {
    #[link(name="SDL")]
    extern {}
}

pub mod ll {
    #![allow(non_camel_case_types)]

    use libc::{c_int, c_uint, uint32_t};
    use libc::types::os::arch::c95::c_schar;

    pub type SDL_errorcode = c_uint;
    pub static SDL_ENOMEM: SDL_errorcode = 0;
    pub static SDL_EFREAD: SDL_errorcode = 1;
    pub static SDL_EFWRITE: SDL_errorcode = 2;
    pub static SDL_EFSEEK: SDL_errorcode = 3;
    pub static SDL_UNSUPPORTED: SDL_errorcode = 4;
    pub static SDL_LASTERROR: SDL_errorcode = 5;

    pub type SDL_InitFlag = uint32_t;
    pub static SDL_INIT_TIMER: SDL_InitFlag = 0x00000001;
    pub static SDL_INIT_AUDIO: SDL_InitFlag = 0x00000010;
    pub static SDL_INIT_VIDEO: SDL_InitFlag = 0x00000020;
    pub static SDL_INIT_CDROM: SDL_InitFlag = 0x00000100;
    pub static SDL_INIT_JOYSTICK: SDL_InitFlag = 0x00000200;
    pub static SDL_INIT_NOPARACHUTE: SDL_InitFlag = 0x00100000;
    pub static SDL_INIT_EVENTTHREAD: SDL_InitFlag = 0x01000000;
    pub static SDL_INIT_EVERYTHING: SDL_InitFlag = 0x0000FFFF;

    extern "C" {
        pub fn SDL_ClearError();
        pub fn SDL_Error(code: SDL_errorcode);
        pub fn SDL_SetError(fmt: *c_schar);
        pub fn SDL_GetError() -> *c_schar;
        pub fn SDL_Quit();
        pub fn SDL_QuitSubSystem(flags: SDL_InitFlag);
        pub fn SDL_Init(flags: uint32_t) -> c_int;
        pub fn SDL_InitSubSystem(flags: SDL_InitFlag) -> c_int;
        pub fn SDL_WasInit(flags: SDL_InitFlag) -> SDL_InitFlag;
        pub fn SDL_GetTicks() -> uint32_t;
    }
}

#[deriving(PartialEq)]
pub struct Rect {
    pub x: i16,
    pub y: i16,
    pub w: u16,
    pub h: u16
}

#[allow(non_snake_case_functions)]
pub fn Rect(x: i16, y: i16, w: u16, h: u16) -> Rect {
    Rect { x: x, y: y, w: w, h: h }
}

impl Rect {
    pub fn new(x: i16, y: i16, w: u16, h: u16) -> Rect {
        Rect {
            x: x,
            y: y,
            w: w,
            h: h
        }
    }
}

#[deriving(PartialEq, Eq)]
pub enum InitFlag {
     InitTimer = ll::SDL_INIT_TIMER as int,
     InitAudio = ll::SDL_INIT_AUDIO as int,
     InitVideo = ll::SDL_INIT_VIDEO as int,
     InitCDRom = ll::SDL_INIT_CDROM as int,
     InitJoystick = ll::SDL_INIT_JOYSTICK as int,
     InitNoParachute = ll::SDL_INIT_NOPARACHUTE as int,
     InitEventThread = ll::SDL_INIT_EVENTTHREAD as int,
     InitEverything = ll::SDL_INIT_EVERYTHING as int,
}

#[deriving(PartialEq, Eq)]
pub enum Error {
     NoMemError = ll::SDL_ENOMEM as int,
     ReadError = ll::SDL_EFREAD as int,
     WriteError = ll::SDL_EFWRITE as int,
     SeekError = ll::SDL_EFSEEK as int,
     UnsupportedError = ll::SDL_UNSUPPORTED as int
}

pub fn init(flags: &[InitFlag]) -> bool {
    unsafe {
        ll::SDL_Init(flags.iter().fold(0u32, |flags, &flag| {
            flags | flag as ll::SDL_InitFlag
        })) == 0
    }
}

pub fn init_subsystem(flags: &[InitFlag]) -> bool {
    unsafe {
        ll::SDL_InitSubSystem(flags.iter().fold(0u32, |flags, &flag| {
            flags | flag as ll::SDL_InitFlag
        })) == 0
    }
}

pub fn quit_subsystem(flags: &[InitFlag]) {
    let flags = flags.iter().fold(0u32, |flags, &flag| {
        flags | flag as ll::SDL_InitFlag
    });

    unsafe { ll::SDL_QuitSubSystem(flags); }
}

pub fn quit() {
    unsafe { ll::SDL_Quit(); }
}

pub fn was_inited(flags: &[InitFlag]) -> Vec<InitFlag> {
    let flags = flags.iter().fold(0u32, |flags, &flag| {
        flags | flag as ll::SDL_InitFlag
    });
    let bitflags = unsafe { ll::SDL_WasInit(flags) };

    let flags = [InitTimer,
        InitAudio,
        InitVideo,
        InitCDRom,
        InitJoystick,
        InitNoParachute,
        InitEventThread,
        InitEverything];

    flags.iter().filter_map(|&flag| {
        if bitflags & (flag as ll::SDL_InitFlag) != 0 { Some(flag) }
        else { None }
    }).collect()
}

pub fn get_error() -> String {
    unsafe {
        let cstr = ll::SDL_GetError();

        str::raw::from_c_str(mem::transmute_copy(&cstr))
    }
}

pub fn set_error(err: &str) {
    unsafe { ll::SDL_SetError(err.to_c_str().unwrap()); }
}

pub fn set_error_from_code(err: Error) {
    unsafe { ll::SDL_Error(err as ll::SDL_errorcode) }
}

pub fn clear_error() {
    unsafe { ll::SDL_ClearError(); }
}

pub fn get_ticks() -> uint {
    unsafe { ll::SDL_GetTicks() as uint }
}
