# ToDo
 1. Switch to using `signalfd` and `epoll`
   1. Remove the mpsc channel
   2. Remove the signal handler
     1. Remove the function
     2. Remove the registration
   3. Add file descriptor to event
     1. Add to `Node::Used`
     2. Add `insert_fd()`
     3. Return `Option<fd>` from `remove()`
   4. Add `epoll` to `LocalEventManager`
     1. Add to struct
     2. Add creation and deletion code
     3. Add registration and unregistration code
     4. Add polling code
   5. Add `AtomicUsize` reference count for process signal blocking
   6. Add a `signalfd` to `LocalEventManager`
 2. Add I/O options
   1. TCP Listener
   2. TCP Stream
   3. UDP Socket
   4. File
 3. Add synchronization primitives
   1. Semaphore
   2. Mutex
   3. Conditional Variable
   4. Read/Write Lock