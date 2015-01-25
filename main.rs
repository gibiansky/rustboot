#![no_std]
#![allow(improper_ctypes)]
#![feature(lang_items)]
#![feature(intrinsics)]

#[allow(unstable)]
extern crate core;

use core::marker::Copy;
use core::iter::*;
use core::num::{from_u8,FromPrimitive};
use core::option::*;
use core::option::Option::*;

extern "rust-intrinsic" {
    pub fn volatile_store<T>(src: *mut T, value: T);
    pub fn volatile_load<T>(src: *const T) -> T;
}

static VIDEO_MEMORY : u32 = 0xB8000;

pub enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Pink       = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    LightPink  = 13,
    Yellow     = 14,
    White      = 15,
}
static COLORS: [Color; 16] = [Color::Black, Color::Blue, Color::Green, Color::Cyan, Color::Red, Color::Pink, Color::Brown, Color::LightGray, Color::DarkGray, Color::LightBlue, Color::LightGreen, Color::LightCyan, Color::LightRed, Color::LightPink, Color::Yellow, Color::White];
impl Copy for Color {}
impl FromPrimitive for Color {
    fn from_i64(n: i64) -> Option<Self> {
        if n < 0 || n > 15 {
            None
        } else {
            Some(COLORS[n as usize])
        }
    }

    fn from_u64(n: u64) -> Option<Self> {
        if n > 15 {
            None
        } else {
            Some(COLORS[n as usize])
        }
    }
}

fn clear_screen(background: Color) {
    for x in range(0, 80) {
        for y in range(0, 25) {
            set_background(x, y, background);
            set_char(x, y, 0);
        }
    }
}

unsafe fn read_video_mem(x: u8, y: u8) -> u16 {
    let addr = VIDEO_MEMORY + 2 * (y as u32 * 80 + x as u32);
    return volatile_load(addr as *const u16);
}

unsafe fn write_video_mem(x: u8, y: u8, val: u16) {
    let addr = VIDEO_MEMORY + 2 * (y as u32 * 80 + x as u32);
    volatile_store(addr as *mut u16, val);
}

fn set_char(x: u8, y: u8, ch: u8) {
    unsafe {
        let old_val: u16 = read_video_mem(x, y);
        let new_val = (old_val & 0xFF00) | (ch as u16);
        write_video_mem(x, y, new_val);
    }
}

fn set_background(x: u8, y: u8, color: Color) {
    unsafe {
        let old_val: u16 = read_video_mem(x, y);
        let new_val = (old_val & 0x0FFF) | ((color as u16) << 12);
        write_video_mem(x, y, new_val);
    }
}

fn set_foreground(x: u8, y: u8, color: Color) {
    unsafe {
        let old_val: u16 = read_video_mem(x, y);
        let new_val = (old_val & 0xF0FF) | ((color as u16) << 8);
        write_video_mem(x, y, new_val);
    }
}

#[no_mangle]
pub fn main() {
    clear_screen(Color::Yellow);
    for x in range(0, 80) {
        for y in range(0, 25) {
            set_foreground(x, y, from_u8((x + y * 80) % 16).unwrap());
            set_char(x, y, '.' as u8);
        }
    }
}
