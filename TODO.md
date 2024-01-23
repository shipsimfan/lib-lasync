# ToDo
 1. Change to single threaded
   1. Change `Arc` to `Rc` and `Mutex` to `RefCell`
   2. Change `mpsc::sync_channel` into a `VecDeque`
   3. Add `!Sync` and `!Send` implementations
 2. Change from `Box<dyn Future>` to generics (`T: Future`)
 3. Add proper asynchronous I/O
   1. `ListenSocket`
   2. `TCPStream`