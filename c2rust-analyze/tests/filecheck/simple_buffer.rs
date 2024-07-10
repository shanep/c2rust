#![feature(rustc_private)]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]

extern crate libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}
pub type uint8_t = libc::c_uchar;
pub type size_t = libc::c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer {
    pub data: *mut uint8_t,
    pub len: size_t,
    pub cap: size_t,
}

#[no_mangle]
pub unsafe extern "C" fn buffer_new(mut cap: size_t) -> *mut buffer {
    let mut buf: *mut buffer = malloc(::std::mem::size_of::<buffer>() as libc::c_ulong)
        as *mut buffer;
    if cap == 0 as libc::c_int as libc::c_ulong {
        let ref mut fresh0 = (*buf).data;
        *fresh0 = 0 as *mut uint8_t;
    } else {
        let ref mut fresh1 = (*buf).data;
        *fresh1 = malloc(cap) as *mut uint8_t;
    }
    (*buf).len = 0 as libc::c_int as size_t;
    (*buf).cap = cap;
    return buf;
}

#[no_mangle]
pub unsafe extern "C" fn buffer_delete(mut buf: *mut buffer) {
    if !((*buf).data).is_null() {
        free((*buf).data as *mut libc::c_void);
    }
    free(buf as *mut libc::c_void);
}

#[no_mangle]
pub unsafe extern "C" fn buffer_realloc(mut buf: *mut buffer, mut new_cap: size_t) {
    if new_cap == (*buf).cap {
        return;
    }
    if (*buf).cap == 0 as libc::c_int as libc::c_ulong {
        let ref mut fresh2 = (*buf).data;
        *fresh2 = malloc(new_cap) as *mut uint8_t;
    } else if new_cap == 0 as libc::c_int as libc::c_ulong {
        free((*buf).data as *mut libc::c_void);
        let ref mut fresh3 = (*buf).data;
        *fresh3 = 0 as *mut uint8_t;
    } else {
        let ref mut fresh4 = (*buf).data;
        *fresh4 = realloc((*buf).data as *mut libc::c_void, new_cap) as *mut uint8_t;
    }
    (*buf).cap = new_cap;
    if (*buf).len > new_cap {
        (*buf).len = new_cap;
    }
}

#[no_mangle]
pub unsafe extern "C" fn buffer_push(mut buf: *mut buffer, mut byte: uint8_t) {
    if (*buf).len == (*buf).cap {
        if (*buf).cap == 0 as libc::c_int as libc::c_ulong {
            buffer_realloc(buf, 4 as libc::c_int as size_t);
        } else {
            buffer_realloc(
                buf,
                ((*buf).cap).wrapping_mul(2 as libc::c_int as libc::c_ulong),
            );
        }
    }
    *((*buf).data).offset((*buf).len as isize) = byte;
    let ref mut fresh5 = (*buf).len;
    *fresh5 = (*fresh5).wrapping_add(1);
}

#[no_mangle]
pub unsafe extern "C" fn test_buffer() {
    let mut buf: *mut buffer = buffer_new(3 as libc::c_int as size_t);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        buffer_push(buf, i as uint8_t);
        i += 1;
    }
    buffer_delete(buf);
}

unsafe fn main_0() -> libc::c_int {
    test_buffer();
    return 0;
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
