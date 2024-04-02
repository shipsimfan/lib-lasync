use uring::io_uring_cqe;

/// A handler called when an event signals completion
#[derive(Clone, Copy)]
pub enum EventHandler {
    /// No data is associated with the event
    Unit(fn(cqe: &mut io_uring_cqe)),

    /// A boolean is associated with the event
    Boolean(bool, fn(cqe: &mut io_uring_cqe, value: &mut bool)),

    /// An integer is associated with the event
    Integer(usize, fn(cqe: &mut io_uring_cqe, value: &mut usize)),
}

impl EventHandler {
    /// Creates a new [`EventHandler`] with no associated data
    pub fn unit(handler: fn(&mut io_uring_cqe)) -> Self {
        EventHandler::Unit(handler)
    }

    /// Creates a new [`EventHandler`] with an associated boolean initialized to `false`
    pub fn boolean(handler: fn(&mut io_uring_cqe, &mut bool)) -> Self {
        EventHandler::Boolean(false, handler)
    }

    /// Creates a new [`EventHandler`] with an associated boolean initialized to `value`
    pub fn boolean_with_value(handler: fn(&mut io_uring_cqe, &mut bool), value: bool) -> Self {
        EventHandler::Boolean(value, handler)
    }

    /// Creates a new [`EventHandler`] with an associated integer initilized to `0`
    pub fn integer(handler: fn(&mut io_uring_cqe, &mut usize)) -> Self {
        EventHandler::Integer(0, handler)
    }

    /// Creates a new [`EventHandler`] with an associated integer initilized to `value`
    pub fn integer_with_value(handler: fn(&mut io_uring_cqe, &mut usize), value: usize) -> Self {
        EventHandler::Integer(value, handler)
    }

    /// Gets the boolean value associated with the event if there is one
    pub fn as_boolean_opt(&self) -> Option<bool> {
        match self {
            EventHandler::Boolean(value, _) => Some(*value),
            _ => None,
        }
    }

    /// Gets the boolean value associated with the event, panicking if there isn't one.
    pub fn as_boolean(&self) -> bool {
        self.as_boolean_opt()
            .expect("Attempted to get a boolean from a non-boolean event")
    }

    /// Gets the integer value associated with the event if there is one
    pub fn as_integer_opt(&self) -> Option<usize> {
        match self {
            EventHandler::Integer(value, _) => Some(*value),
            _ => None,
        }
    }

    /// Gets the integer value associated with the event, panicking if there isn't one.
    pub fn as_integer(&self) -> usize {
        self.as_integer_opt()
            .expect("Attempted to get an integer from a non-integer event")
    }

    /// Sets the boolean associated with the event, panicking if the event doesn't contain a
    /// boolean.
    pub fn set_boolean(&mut self, new_value: bool) {
        match self {
            EventHandler::Boolean(value, _) => *value = new_value,
            _ => panic!("Attempted to set a boolean on a non-boolean event"),
        }
    }

    /// Sets the integer associated with the event, panicking if the event doesn't contain an
    /// integer.
    pub fn set_integer(&mut self, new_value: usize) {
        match self {
            EventHandler::Integer(value, _) => *value = new_value,
            _ => panic!("Attempted to set an integer on a non-integer event"),
        }
    }

    /// Runs the event handler
    pub(crate) fn run(&mut self, cqe: &mut io_uring_cqe) {
        match self {
            EventHandler::Unit(handler) => (handler)(cqe),
            EventHandler::Boolean(value, handler) => (handler)(cqe, value),
            EventHandler::Integer(value, handler) => (handler)(cqe, value),
        }
    }
}

impl From<fn(&mut io_uring_cqe)> for EventHandler {
    fn from(handler: fn(&mut io_uring_cqe)) -> Self {
        EventHandler::Unit(handler)
    }
}

impl From<fn(&mut io_uring_cqe, &mut bool)> for EventHandler {
    fn from(handler: fn(&mut io_uring_cqe, &mut bool)) -> Self {
        EventHandler::Boolean(false, handler)
    }
}

impl From<(fn(&mut io_uring_cqe, &mut bool), bool)> for EventHandler {
    fn from(value: (fn(&mut io_uring_cqe, &mut bool), bool)) -> Self {
        let (handler, initial_value) = value;
        EventHandler::Boolean(initial_value, handler)
    }
}

impl From<fn(&mut io_uring_cqe, &mut usize)> for EventHandler {
    fn from(handler: fn(&mut io_uring_cqe, &mut usize)) -> Self {
        EventHandler::Integer(0, handler)
    }
}

impl From<(fn(&mut io_uring_cqe, &mut usize), usize)> for EventHandler {
    fn from(value: (fn(&mut io_uring_cqe, &mut usize), usize)) -> Self {
        let (handler, initial_value) = value;
        EventHandler::Integer(initial_value, handler)
    }
}
