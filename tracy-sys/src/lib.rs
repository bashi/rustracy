use std::os::raw::c_char;
use std::os::raw::c_int;

#[repr(C)]
pub struct SourceLocation {
    pub name: *const c_char,
    pub function: *const c_char,
    pub file: *const c_char,
    pub line: u32,
    pub color: u32,
}

#[repr(C)]
pub struct ZoneContext {
    id: u32,
    active: c_int,
}

#[link(name = "tracy")]
extern "C" {
    fn ___tracy_emit_zone_begin(srcloc: *const SourceLocation, active: c_int) -> ZoneContext;
    fn ___tracy_emit_zone_end(ctx: ZoneContext);

    fn ___tracy_emit_frame_mark(name: *const c_char);
    fn ___tracy_emit_frame_mark_start(name: *const c_char);
    fn ___tracy_emit_frame_mark_end(name: *const c_char);
}

pub fn emit_zone_begin(loc: &SourceLocation) -> ZoneContext {
    unsafe { ___tracy_emit_zone_begin(loc, 1) }
}

pub fn emit_zone_end(ctx: ZoneContext) {
    unsafe { ___tracy_emit_zone_end(ctx) }
}

// Tentative
pub fn emit_frame_mark_with_null() {
    unsafe { ___tracy_emit_frame_mark(std::ptr::null()) }
}

pub fn emit_frame_mark(name: &str) {
    unsafe { ___tracy_emit_frame_mark(name.as_ptr() as *const _) }
}

pub fn emit_frame_mark_start(name: &str) {
    unsafe { ___tracy_emit_frame_mark_start(name.as_ptr() as *const _) }
}

pub fn emit_frame_mark_end(name: &str) {
    unsafe { ___tracy_emit_frame_mark_end(name.as_ptr() as *const _) }
}
