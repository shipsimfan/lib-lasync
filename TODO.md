# ToDo
 1. Add proper asynchronous I/O
   1. Change timer to use `timerfd_create`
   2. Change `LocalExecutor` to use `epoll`
 2. Change to single threaded
   1. Change `Arc` to `Rc` and `Mutex` to `RefCell`
   2. Change `mpsc::sync_channel` into a `VecDeque`
   3. Add `!Sync` and `!Send` implementations
   4. Add a thread local storage component to `LocalExecutor`
 3. Change from `Box<dyn Future>` to generics (`T: Future`)
 4. Add I/O options
   1. TCPListener
   2. TCPStream
   3. UDPSocket
   4. File