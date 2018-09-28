

//! High-level bindings to the OMPT 5.0 API.
//!
//! # Build Instructions
//! To build an OMPT tool, you will want to set your crate's library type to `cdylib`, like so:
//!
//! ```
//! [lib]
//! crate-type = ["cdylib"]
//! ```
//!
//! # Code Example
//!
//! ```rust
//! #[macro_use]
//! use std::os::raw::*;
//! use std::mem::{transmute};
//! use ompt_50_sys as ffi;
//! use ompt_50_rs as ompt;
//!
//! static OMPT_Tool: Option<OMPTTool> = None;
//!
//! unsafe {
//!     let OMPT_CALLBACKS: Vec<(ffi::Callbacks, ffi::CallbackFn)> = vec![
//!         (ffi::Callbacks::Master,        transmute(on_ompt_callback_master)),
//!         (ffi::Callbacks::ImplicitTask,  transmute(on_ompt_callback_implicit_task)),
//!     ];
//! }
//!
//! // Entry point for the tool. Must be visible as a C-style symbol in the DLL.
//! #[no_mangle]
//! pub extern "C" fn ompt_start_tool(omp_version: c_uint,
//!                                   runtime_version: *const c_char)
//!     -> *mut ffi::StartToolResult {
//!     let tool: ompt::OMPTTool = OMPTTool::new(Some(initialize_my_tool),
//!                                              None,
//!                                              None,
//!                                              Some(OMPT_CALLBACKS));
//!     // Set global tool variable so that we can register callbacks later.
//!     unsafe {
//!         OMPT_tool = Some(tool);
//!     }
//!
//!     // Create a StartToolResult to hand back to the runtime.
//!     let out: ffi::StartToolResult {
//!         initialize: Some(initialize_my_tool),
//!         finalize: None,
//!         tool_data: tool_data.unwrap_or(ffi::Data { value: 0 }),
//!     };
//!
//!     // HACK: This "boxing" logic. May eliminate or clean up later.
//!     let out_boxed = Box::new(out);
//!     let p = Box::into_raw(out_boxed);
//!     p
//! }
//!
//! // Required initializer function for the tool.
//! pub extern "C" fn initialize_my_tool(fn_lookup: ffi::FunctionLookupFn,
//!                                      tool_data: *mut ffi::Data) -> c_int {
//!     // Register callbacks with OpenMP runtime.
//!     OMPT_Tool.register_callbacks(fn_lookup, tool_data);
//!     1
//! }
//!
//! // Callback handlers
//! pub fn on_ompt_callback_master(
//!         endpoint: ompt::ompt_scope_endpoint_t,
//!         parallel_data: *mut ompt::ompt_data_t,
//!         task_data: *mut ompt::ompt_data_t,
//!         codeptr_ra: *const c_void) {
//!     // ...
//! }
//!
//! pub extern "C" fn on_ompt_callback_implicit_task(
//!         endpoint: ompt::ompt_scope_endpoint_t,
//!         parallel_data: *mut ompt::ompt_data_t,
//!         task_data: *mut ompt::ompt_data_t,
//!         team_size: c_uint,
//!         thread_num: c_uint) {
//!     // ...
//! }
//! ```


#[macro_use]
pub extern crate ompt_50_sys as sys;

use std::fmt;
use std::mem::{transmute};
use std::option::{Option};
use std::vec::*;
use std::ffi::CString;
use sys as ffi;


/// This structure contains the necessary context to construct/run an OMPT tool at runtime.
///
/// To create a valid tool you must bind at least the `initialize_fn` member of the struct.

#[derive(Clone)]
pub struct OMPTTool {
    callbacks: Vec<(ffi::Callbacks, ffi::CallbackFn)>,
    initialize_fn: ffi::InitializeFn,
    finalize_fn: ffi::FinalizeFn,
    tool_data: ffi::Data,
}

unsafe impl Send for OMPTTool {}
unsafe impl Sync for OMPTTool {}

impl OMPTTool {
    pub fn new(initializer: ffi::InitializeFn,
               finalizer: ffi::FinalizeFn,
               tool_data: Option<ffi::Data>,
               callbacks: Option<Vec<(ffi::Callbacks, ffi::CallbackFn)>>)
        -> OMPTTool {
        //unsafe {
        //    let out: Vec<(ffi::Callbacks, ffi::CallbackFn)> = vec![
        //        (ffi::Callbacks::Idle,             transmute(Idle)),
        //    ];
        //}
        OMPTTool {
            callbacks: callbacks.unwrap_or(Vec::new()),
            initialize_fn: initializer,
            finalize_fn: finalizer,
            tool_data: tool_data.unwrap_or(ffi::Data { value: 0 }),
        }
    }

    pub fn register_callbacks(&self,
                    fn_lookup: ffi::FunctionLookupFn,
                    tool_data: *mut ffi::Data) {
        // Obtain the function ptr to the lookup function.
        let lookup = fn_lookup.unwrap(); // Should always be Some(fn).
        // Have to do some wild function pointer type casting here.
        unsafe {
            // Allocate a C-style string, and unwrap the Option.
            let raw_cstr = CString::new("ompt_set_callback").unwrap();
            // Look up the Option<function pointer> and unsafely cast it.
            let ompt_set_callback_option: ffi::SetCallbackFn =
                transmute(lookup(raw_cstr.as_ptr()));
            // Unwrap the Option to get the real function.
            let ompt_set_callback = ompt_set_callback_option.expect("Error: Invalid pointer to ompt_set_callback.");
            // Assign all callbacks.
            for (callback_type, fnptr) in self.callbacks.clone() {
                if fnptr.is_some() {
                    let retcode = ompt_set_callback(callback_type, fnptr);
                    println!("{:?} {:?}", callback_type, retcode);
                }
            }
        }
    }

}

impl fmt::Debug for OMPTTool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OMPTTool {{ callbacks: {:?}, initialize_fn: {:?}, finalize_fn: {:?}, tool_data: {:?} }}",
               self.callbacks,
               self.initialize_fn,
               self.finalize_fn,
               "Tool data unavailable.")
    }
}
