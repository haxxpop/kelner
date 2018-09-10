// Copyright (c) 2018, Suphanat Chunhapanya
// This file is part of Kelner.
//
// Kelner is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version 2
// of the License, or (at your option) any later version.
//
// Kelner is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Kelner.  If not, see <https://www.gnu.org/licenses/>.
#![feature(panic_handler, doc_cfg)]
#![no_std]
#![cfg_attr(all(not(test), not(rustdoc)), no_main)]

#[cfg(not(test))]
mod alloc;
mod util;

#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[global_allocator]
static ALLOCATOR: alloc::Allocator = alloc::Allocator;

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    hello();
    loop {}
}

#[cfg(not(test))]
fn hello() {
    let hello_str: &[u8] = b"Hello World!";
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in hello_str.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
}

#[cfg(not(test))]
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
