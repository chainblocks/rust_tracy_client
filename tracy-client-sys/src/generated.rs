type __uint8_t = ::std::os::raw::c_uchar;
type __uint16_t = ::std::os::raw::c_ushort;
type __uint32_t = ::std::os::raw::c_uint;
type __int64_t = ::std::os::raw::c_long;
type __uint64_t = ::std::os::raw::c_ulong;
extern "C" {
    pub fn ___tracy_set_thread_name(name: *const ::std::os::raw::c_char);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ___tracy_source_location_data {
    pub name: *const ::std::os::raw::c_char,
    pub function: *const ::std::os::raw::c_char,
    pub file: *const ::std::os::raw::c_char,
    pub line: u32,
    pub color: u32,
}
#[test]
fn bindgen_test_layout____tracy_source_location_data() {
    assert_eq!(
        ::std::mem::size_of::<___tracy_source_location_data>(),
        32usize,
        concat!("Size of: ", stringify!(___tracy_source_location_data))
    );
    assert_eq!(
        ::std::mem::align_of::<___tracy_source_location_data>(),
        8usize,
        concat!("Alignment of ", stringify!(___tracy_source_location_data))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<___tracy_source_location_data>())).name as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(___tracy_source_location_data),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<___tracy_source_location_data>())).function as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(___tracy_source_location_data),
            "::",
            stringify!(function)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<___tracy_source_location_data>())).file as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(___tracy_source_location_data),
            "::",
            stringify!(file)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<___tracy_source_location_data>())).line as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(___tracy_source_location_data),
            "::",
            stringify!(line)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<___tracy_source_location_data>())).color as *const _ as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(___tracy_source_location_data),
            "::",
            stringify!(color)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ___tracy_c_zone_context {
    pub id: u32,
    pub active: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout____tracy_c_zone_context() {
    assert_eq!(
        ::std::mem::size_of::<___tracy_c_zone_context>(),
        8usize,
        concat!("Size of: ", stringify!(___tracy_c_zone_context))
    );
    assert_eq!(
        ::std::mem::align_of::<___tracy_c_zone_context>(),
        4usize,
        concat!("Alignment of ", stringify!(___tracy_c_zone_context))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<___tracy_c_zone_context>())).id as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(___tracy_c_zone_context),
            "::",
            stringify!(id)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<___tracy_c_zone_context>())).active as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(___tracy_c_zone_context),
            "::",
            stringify!(active)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ___tracy_gpu_time_data {
    pub gpuTime: i64,
    pub queryId: u16,
    pub context: u8,
}
#[test]
fn bindgen_test_layout____tracy_gpu_time_data() {
    assert_eq!(
        ::std::mem::size_of::<___tracy_gpu_time_data>(),
        16usize,
        concat!("Size of: ", stringify!(___tracy_gpu_time_data))
    );
    assert_eq!(
        ::std::mem::align_of::<___tracy_gpu_time_data>(),
        8usize,
        concat!("Alignment of ", stringify!(___tracy_gpu_time_data))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<___tracy_gpu_time_data>())).gpuTime as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(___tracy_gpu_time_data),
            "::",
            stringify!(gpuTime)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<___tracy_gpu_time_data>())).queryId as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(___tracy_gpu_time_data),
            "::",
            stringify!(queryId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<___tracy_gpu_time_data>())).context as *const _ as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(___tracy_gpu_time_data),
            "::",
            stringify!(context)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ___tracy_gpu_zone_begin_data {
    pub srcloc: u64,
    pub queryId: u16,
    pub context: u8,
}
#[test]
fn bindgen_test_layout____tracy_gpu_zone_begin_data() {
    assert_eq!(
        ::std::mem::size_of::<___tracy_gpu_zone_begin_data>(),
        16usize,
        concat!("Size of: ", stringify!(___tracy_gpu_zone_begin_data))
    );
    assert_eq!(
        ::std::mem::align_of::<___tracy_gpu_zone_begin_data>(),
        8usize,
        concat!("Alignment of ", stringify!(___tracy_gpu_zone_begin_data))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<___tracy_gpu_zone_begin_data>())).srcloc as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(___tracy_gpu_zone_begin_data),
            "::",
            stringify!(srcloc)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<___tracy_gpu_zone_begin_data>())).queryId as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(___tracy_gpu_zone_begin_data),
            "::",
            stringify!(queryId)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<___tracy_gpu_zone_begin_data>())).context as *const _ as usize
        },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(___tracy_gpu_zone_begin_data),
            "::",
            stringify!(context)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ___tracy_gpu_zone_end_data {
    pub queryId: u16,
    pub context: u8,
}
#[test]
fn bindgen_test_layout____tracy_gpu_zone_end_data() {
    assert_eq!(
        ::std::mem::size_of::<___tracy_gpu_zone_end_data>(),
        4usize,
        concat!("Size of: ", stringify!(___tracy_gpu_zone_end_data))
    );
    assert_eq!(
        ::std::mem::align_of::<___tracy_gpu_zone_end_data>(),
        2usize,
        concat!("Alignment of ", stringify!(___tracy_gpu_zone_end_data))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<___tracy_gpu_zone_end_data>())).queryId as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(___tracy_gpu_zone_end_data),
            "::",
            stringify!(queryId)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<___tracy_gpu_zone_end_data>())).context as *const _ as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(___tracy_gpu_zone_end_data),
            "::",
            stringify!(context)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ___tracy_gpu_new_context_data {
    pub gpuTime: i64,
    pub period: f32,
    pub context: u8,
    pub flags: u8,
    pub type_: u8,
}
#[test]
fn bindgen_test_layout____tracy_gpu_new_context_data() {
    assert_eq!(
        ::std::mem::size_of::<___tracy_gpu_new_context_data>(),
        16usize,
        concat!("Size of: ", stringify!(___tracy_gpu_new_context_data))
    );
    assert_eq!(
        ::std::mem::align_of::<___tracy_gpu_new_context_data>(),
        8usize,
        concat!("Alignment of ", stringify!(___tracy_gpu_new_context_data))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<___tracy_gpu_new_context_data>())).gpuTime as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(___tracy_gpu_new_context_data),
            "::",
            stringify!(gpuTime)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<___tracy_gpu_new_context_data>())).period as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(___tracy_gpu_new_context_data),
            "::",
            stringify!(period)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<___tracy_gpu_new_context_data>())).context as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(___tracy_gpu_new_context_data),
            "::",
            stringify!(context)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<___tracy_gpu_new_context_data>())).flags as *const _ as usize
        },
        13usize,
        concat!(
            "Offset of field: ",
            stringify!(___tracy_gpu_new_context_data),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<___tracy_gpu_new_context_data>())).type_ as *const _ as usize
        },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(___tracy_gpu_new_context_data),
            "::",
            stringify!(type_)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ___tracy_gpu_context_name_data {
    pub context: u8,
    pub name: *const ::std::os::raw::c_char,
    pub len: u16,
}
#[test]
fn bindgen_test_layout____tracy_gpu_context_name_data() {
    assert_eq!(
        ::std::mem::size_of::<___tracy_gpu_context_name_data>(),
        24usize,
        concat!("Size of: ", stringify!(___tracy_gpu_context_name_data))
    );
    assert_eq!(
        ::std::mem::align_of::<___tracy_gpu_context_name_data>(),
        8usize,
        concat!("Alignment of ", stringify!(___tracy_gpu_context_name_data))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<___tracy_gpu_context_name_data>())).context as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(___tracy_gpu_context_name_data),
            "::",
            stringify!(context)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<___tracy_gpu_context_name_data>())).name as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(___tracy_gpu_context_name_data),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<___tracy_gpu_context_name_data>())).len as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(___tracy_gpu_context_name_data),
            "::",
            stringify!(len)
        )
    );
}
type TracyCZoneCtx = ___tracy_c_zone_context;
extern "C" {
    pub fn ___tracy_startup_profiler();
}
extern "C" {
    pub fn ___tracy_shutdown_profiler();
}
extern "C" {
    pub fn ___tracy_alloc_srcloc(
        line: u32,
        source: *const ::std::os::raw::c_char,
        sourceSz: usize,
        function: *const ::std::os::raw::c_char,
        functionSz: usize,
    ) -> u64;
}
extern "C" {
    pub fn ___tracy_alloc_srcloc_name(
        line: u32,
        source: *const ::std::os::raw::c_char,
        sourceSz: usize,
        function: *const ::std::os::raw::c_char,
        functionSz: usize,
        name: *const ::std::os::raw::c_char,
        nameSz: usize,
    ) -> u64;
}
extern "C" {
    pub fn ___tracy_emit_zone_begin(
        srcloc: *const ___tracy_source_location_data,
        active: ::std::os::raw::c_int,
    ) -> TracyCZoneCtx;
}
extern "C" {
    pub fn ___tracy_emit_zone_begin_callstack(
        srcloc: *const ___tracy_source_location_data,
        depth: ::std::os::raw::c_int,
        active: ::std::os::raw::c_int,
    ) -> TracyCZoneCtx;
}
extern "C" {
    pub fn ___tracy_emit_zone_begin_alloc(
        srcloc: u64,
        active: ::std::os::raw::c_int,
    ) -> TracyCZoneCtx;
}
extern "C" {
    pub fn ___tracy_emit_zone_begin_alloc_callstack(
        srcloc: u64,
        depth: ::std::os::raw::c_int,
        active: ::std::os::raw::c_int,
    ) -> TracyCZoneCtx;
}
extern "C" {
    pub fn ___tracy_emit_zone_end(ctx: TracyCZoneCtx);
}
extern "C" {
    pub fn ___tracy_emit_zone_text(
        ctx: TracyCZoneCtx,
        txt: *const ::std::os::raw::c_char,
        size: usize,
    );
}
extern "C" {
    pub fn ___tracy_emit_zone_name(
        ctx: TracyCZoneCtx,
        txt: *const ::std::os::raw::c_char,
        size: usize,
    );
}
extern "C" {
    pub fn ___tracy_emit_zone_color(ctx: TracyCZoneCtx, color: u32);
}
extern "C" {
    pub fn ___tracy_emit_zone_value(ctx: TracyCZoneCtx, value: u64);
}
extern "C" {
    pub fn ___tracy_emit_gpu_zone_begin_alloc(arg1: ___tracy_gpu_zone_begin_data);
}
extern "C" {
    pub fn ___tracy_emit_gpu_zone_end(data: ___tracy_gpu_zone_end_data);
}
extern "C" {
    pub fn ___tracy_emit_gpu_time(arg1: ___tracy_gpu_time_data);
}
extern "C" {
    pub fn ___tracy_emit_gpu_new_context(arg1: ___tracy_gpu_new_context_data);
}
extern "C" {
    pub fn ___tracy_emit_gpu_context_name(arg1: ___tracy_gpu_context_name_data);
}
extern "C" {
    pub fn ___tracy_emit_gpu_zone_begin_alloc_serial(arg1: ___tracy_gpu_zone_begin_data);
}
extern "C" {
    pub fn ___tracy_emit_gpu_zone_end_serial(data: ___tracy_gpu_zone_end_data);
}
extern "C" {
    pub fn ___tracy_emit_gpu_time_serial(arg1: ___tracy_gpu_time_data);
}
extern "C" {
    pub fn ___tracy_emit_gpu_new_context_serial(arg1: ___tracy_gpu_new_context_data);
}
extern "C" {
    pub fn ___tracy_emit_gpu_context_name_serial(arg1: ___tracy_gpu_context_name_data);
}
extern "C" {
    pub fn ___tracy_connected() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ___tracy_emit_memory_alloc(
        ptr: *const ::std::os::raw::c_void,
        size: usize,
        secure: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn ___tracy_emit_memory_alloc_callstack(
        ptr: *const ::std::os::raw::c_void,
        size: usize,
        depth: ::std::os::raw::c_int,
        secure: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn ___tracy_emit_memory_free(
        ptr: *const ::std::os::raw::c_void,
        secure: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn ___tracy_emit_memory_free_callstack(
        ptr: *const ::std::os::raw::c_void,
        depth: ::std::os::raw::c_int,
        secure: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn ___tracy_emit_memory_alloc_named(
        ptr: *const ::std::os::raw::c_void,
        size: usize,
        secure: ::std::os::raw::c_int,
        name: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn ___tracy_emit_memory_alloc_callstack_named(
        ptr: *const ::std::os::raw::c_void,
        size: usize,
        depth: ::std::os::raw::c_int,
        secure: ::std::os::raw::c_int,
        name: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn ___tracy_emit_memory_free_named(
        ptr: *const ::std::os::raw::c_void,
        secure: ::std::os::raw::c_int,
        name: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn ___tracy_emit_memory_free_callstack_named(
        ptr: *const ::std::os::raw::c_void,
        depth: ::std::os::raw::c_int,
        secure: ::std::os::raw::c_int,
        name: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn ___tracy_emit_message(
        txt: *const ::std::os::raw::c_char,
        size: usize,
        callstack: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn ___tracy_emit_messageL(
        txt: *const ::std::os::raw::c_char,
        callstack: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn ___tracy_emit_messageC(
        txt: *const ::std::os::raw::c_char,
        size: usize,
        color: u32,
        callstack: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn ___tracy_emit_messageLC(
        txt: *const ::std::os::raw::c_char,
        color: u32,
        callstack: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn ___tracy_emit_frame_mark(name: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn ___tracy_emit_frame_mark_start(name: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn ___tracy_emit_frame_mark_end(name: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn ___tracy_emit_frame_image(
        image: *const ::std::os::raw::c_void,
        w: u16,
        h: u16,
        offset: u8,
        flip: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn ___tracy_emit_plot(name: *const ::std::os::raw::c_char, val: f64);
}
extern "C" {
    pub fn ___tracy_emit_message_appinfo(txt: *const ::std::os::raw::c_char, size: usize);
}
extern "C" {
    pub fn ___tracy_fiber_enter(fiber: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn ___tracy_fiber_leave();
}
