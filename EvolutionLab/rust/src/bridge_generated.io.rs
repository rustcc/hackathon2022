use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_create(port_: i64, shape: *mut wire_Shape, boundary: i32) {
    wire_create_impl(port_, shape, boundary)
}

#[no_mangle]
pub extern "C" fn wire_decode_rle(port_: i64, rle: *mut wire_uint_8_list) {
    wire_decode_rle_impl(port_, rle)
}

#[no_mangle]
pub extern "C" fn wire_encode_rle(
    port_: i64,
    header: *mut wire_Header,
    cells: *mut wire_list_position,
) {
    wire_encode_rle_impl(port_, header, cells)
}

#[no_mangle]
pub extern "C" fn wire_default_pattern(port_: i64) {
    wire_default_pattern_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_evolve__method__Life(port_: i64, that: *mut wire_Life, step: *mut u32) {
    wire_evolve__method__Life_impl(port_, that, step)
}

#[no_mangle]
pub extern "C" fn wire_clean_cells__method__Life(port_: i64, that: *mut wire_Life) {
    wire_clean_cells__method__Life_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_rand__method__Life(port_: i64, that: *mut wire_Life, distr: f64) {
    wire_rand__method__Life_impl(port_, that, distr)
}

#[no_mangle]
pub extern "C" fn wire_get_cells__method__Life(port_: i64, that: *mut wire_Life) {
    wire_get_cells__method__Life_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_set_cells__method__Life(
    port_: i64,
    that: *mut wire_Life,
    cells: *mut wire_list_position,
) {
    wire_set_cells__method__Life_impl(port_, that, cells)
}

#[no_mangle]
pub extern "C" fn wire_set_boundary__method__Life(port_: i64, that: *mut wire_Life, boundary: i32) {
    wire_set_boundary__method__Life_impl(port_, that, boundary)
}

#[no_mangle]
pub extern "C" fn wire_set_shape__method__Life(
    port_: i64,
    that: *mut wire_Life,
    shape: *mut wire_Shape,
    clean: *mut bool,
) {
    wire_set_shape__method__Life_impl(port_, that, shape, clean)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_MutexArrayLife() -> wire_MutexArrayLife {
    wire_MutexArrayLife::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_bool_0(value: bool) -> *mut bool {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_header_0() -> *mut wire_Header {
    support::new_leak_box_ptr(wire_Header::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_life_0() -> *mut wire_Life {
    support::new_leak_box_ptr(wire_Life::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_shape_0() -> *mut wire_Shape {
    support::new_leak_box_ptr(wire_Shape::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_u32_0(value: u32) -> *mut u32 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_list_position_0(len: i32) -> *mut wire_list_position {
    let wrap = wire_list_position {
        ptr: support::new_leak_vec_ptr(<wire_Position>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: related functions

#[no_mangle]
pub extern "C" fn drop_opaque_MutexArrayLife(ptr: *const c_void) {
    unsafe {
        Arc::<Mutex<ArrayLife>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_MutexArrayLife(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<Mutex<ArrayLife>>::increment_strong_count(ptr as _);
        ptr
    }
}

// Section: impl Wire2Api

impl Wire2Api<RustOpaque<Mutex<ArrayLife>>> for wire_MutexArrayLife {
    fn wire2api(self) -> RustOpaque<Mutex<ArrayLife>> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}

impl Wire2Api<Header> for *mut wire_Header {
    fn wire2api(self) -> Header {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Header>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Life> for *mut wire_Life {
    fn wire2api(self) -> Life {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Life>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Shape> for *mut wire_Shape {
    fn wire2api(self) -> Shape {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Shape>::wire2api(*wrap).into()
    }
}

impl Wire2Api<Header> for wire_Header {
    fn wire2api(self) -> Header {
        Header {
            name: self.name.wire2api(),
            owner: self.owner.wire2api(),
            comment: self.comment.wire2api(),
            rule: self.rule.wire2api(),
            x: self.x.wire2api(),
            y: self.y.wire2api(),
        }
    }
}

impl Wire2Api<Life> for wire_Life {
    fn wire2api(self) -> Life {
        Life(self.field0.wire2api())
    }
}
impl Wire2Api<Vec<Position>> for *mut wire_list_position {
    fn wire2api(self) -> Vec<Position> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}

impl Wire2Api<Position> for wire_Position {
    fn wire2api(self) -> Position {
        Position {
            x: self.x.wire2api(),
            y: self.y.wire2api(),
        }
    }
}
impl Wire2Api<Shape> for wire_Shape {
    fn wire2api(self) -> Shape {
        Shape {
            x: self.x.wire2api(),
            y: self.y.wire2api(),
        }
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}

// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_MutexArrayLife {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Header {
    name: *mut wire_uint_8_list,
    owner: *mut wire_uint_8_list,
    comment: *mut wire_uint_8_list,
    rule: *mut wire_uint_8_list,
    x: usize,
    y: usize,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Life {
    field0: wire_MutexArrayLife,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_position {
    ptr: *mut wire_Position,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Position {
    x: usize,
    y: usize,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Shape {
    x: usize,
    y: usize,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

impl NewWithNullPtr for wire_MutexArrayLife {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}

impl NewWithNullPtr for wire_Header {
    fn new_with_null_ptr() -> Self {
        Self {
            name: core::ptr::null_mut(),
            owner: core::ptr::null_mut(),
            comment: core::ptr::null_mut(),
            rule: core::ptr::null_mut(),
            x: Default::default(),
            y: Default::default(),
        }
    }
}

impl NewWithNullPtr for wire_Life {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: wire_MutexArrayLife::new_with_null_ptr(),
        }
    }
}

impl NewWithNullPtr for wire_Position {
    fn new_with_null_ptr() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}

impl NewWithNullPtr for wire_Shape {
    fn new_with_null_ptr() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturnStruct(val: support::WireSyncReturnStruct) {
    unsafe {
        let _ = support::vec_from_leak_ptr(val.ptr, val.len);
    }
}
