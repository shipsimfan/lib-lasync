# ToDo
 1. Switch from epoll to aio
   1. Remove epoll from `EventManager`
   2. Add index-based id to `Event`
   3. Add signal handler for waking tasks (use `sigevent.value.ptr`)
   4. Switch from `VecDeque` to `mpsc::channel` for signal saftey
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