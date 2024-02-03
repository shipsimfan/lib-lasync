# ToDo
 1. Switch to using `signalfd` and `epoll`
   1. Add `epoll` to `LocalEventManager`
     1. Add to struct
     2. Add creation and deletion code
     3. Add registration and unregistration code
     4. Add polling code
   2. Add a `signalfd` to `LocalEventManager`
     1. Add to struct
     2. Add creation code, let user choose signal number
     3. Add deletion code
     4. Add poll code
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