use uring::io_uring_cqe;

/// A handler called when an event signals completion
#[derive(Clone, Copy)]
pub struct EventHandler {
    /// The shared value associated with the event
    value: usize,

    /// The handler itself
    handler: fn(cqe: &mut io_uring_cqe, value: &mut usize),
}

impl EventHandler {
    /// Creates a new [`EventHandler`] for `handler` with `initial value`
    pub const fn new(
        initial_value: usize,
        handler: fn(cqe: &mut io_uring_cqe, value: &mut usize),
    ) -> Self {
        EventHandler {
            value: initial_value,
            handler,
        }
    }

    /// Gets the value associated with the event
    pub fn value(&self) -> usize {
        self.value
    }

    /// Sets the value associated with the event
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
    }

    /// Runs the event handler
    pub(crate) fn run(&mut self, cqe: &mut io_uring_cqe) {
        (self.handler)(cqe, &mut self.value)
    }
}
