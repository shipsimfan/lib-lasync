use super::eventfd::EventFD;
use linux::{
    signal::{sigaction, sigaction_handler, sigaction_t, siginfo_t, SA_SIGINFO, SIG_DFL},
    sys::mman::{mprotect, PROT_EXEC, PROT_READ, PROT_WRITE},
    try_linux,
};
use std::{ffi::c_int, pin::Pin, ptr::null_mut, sync::mpsc::Sender};

/// A signal handler with a closure-like entry function
#[repr(C)]
#[repr(packed(8))]
pub(super) struct SignalHandler {
    /// The eventfd to signal
    event_fd: i32,

    /// The signal number, not used in the handler
    signal_number: i32,

    /// The sender to send [`EventID`]s on
    sender: *mut Sender<u64>,

    /// The body of the function
    function: [u8; FUNCTION_BASE.len()],
}

/// The bytes which make up the handler entry
const FUNCTION_BASE: [u8; 26] = *include_bytes!("handler.o");

/// The offset to which to write the real handler's address
const CALL_ADDRESS_OFFSET: usize = 15;

/// The actual signal handler, called by the assembled one
extern "C" fn signal_handler(
    _: c_int,
    siginfo: *mut siginfo_t,
    event_fd: c_int,
    sender: *mut Sender<u64>,
) {
    let siginfo = unsafe { &*siginfo };
    let event_id = unsafe { siginfo.fields.timer.si_value.ptr } as u64;

    unsafe { &*sender }.send(event_id).ok();

    EventFD::signal_raw(event_fd);
}

impl SignalHandler {
    /// Creates and registers a new [`SignalHandler`]
    pub(super) fn new(
        signal_number: c_int,
        event_fd: &EventFD,
        sender: Sender<u64>,
    ) -> linux::Result<Pin<Box<Self>>> {
        let sender = Box::into_raw(Box::new(sender));

        let mut handler = Box::pin(SignalHandler {
            event_fd: event_fd.fd(),
            signal_number,
            sender,
            function: FUNCTION_BASE,
        });

        // Write the real handler's address into the function
        handler.function[CALL_ADDRESS_OFFSET..CALL_ADDRESS_OFFSET + 8]
            .copy_from_slice(&(signal_handler as u64).to_le_bytes());

        // Add execute permissions to the page(s) that contain the function
        let function_start = &handler.function as *const _ as usize;
        let function_end = function_start + handler.function.len();
        let function_page_start = (function_start / 4096) * 4096;
        let protection_length = function_end - function_page_start;
        try_linux!(mprotect(
            function_page_start as _,
            protection_length,
            PROT_READ | PROT_WRITE | PROT_EXEC
        ))?;

        // Register the signal with sigaction
        let action = sigaction_t {
            handler: sigaction_handler {
                handler: function_start as isize,
            },
            flags: SA_SIGINFO,
            ..Default::default()
        };
        try_linux!(sigaction(signal_number, &action, null_mut()))?;

        Ok(handler)
    }
}

impl Drop for SignalHandler {
    fn drop(&mut self) {
        let action = sigaction_t {
            handler: sigaction_handler { handler: SIG_DFL },
            ..Default::default()
        };
        unsafe { sigaction(self.signal_number, &action, null_mut()) };

        drop(unsafe { Box::from_raw(self.sender) });
    }
}
