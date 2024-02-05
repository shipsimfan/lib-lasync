; The handler entry function
;
; This function is the actual function registered with `sigaction` to handle events. This function 
; the calls the actual signal handler in Rust code. This is written in assembly because the 
; function is part of structure that contains state information, like a closure. It extracts the 
; information by looking below itself in memory and setting the appropriate argument registers. 
; This is represented by the `SignalHandler` structure in Rust code.
;
; The following two values are passed as additional arguments to the signal handler:
;  - A `c_int` which is the file descriptor for the eventfd. this replaces the third argument of 
;    the signal handler which is normally a context structure we don't use.
;  - A `*const Sender<u64>` which is passed as the fourth argument to the signal handler

BITS 64

; This is just a placeholder for the assembler to understand what is happening. These bytes are 
; stripped manually from the output binary.
_variables:
    .event_fd: dd 0
    ._pad: dd 0
    .sender: dq 0

; Only this code is included in the output binary
asm_signal_handler:
    ; ESI is the third argument, as the second argument is actually on the heap
    mov edx, [rel _variables.event_fd]

    ; RDX is the fourth argument
    mov rcx, [rel _variables.sender]

    ; 0xFFFFFFFF0BADC0DE marks where the address for the actual function should be inserted
    mov rax, 0xFFFFFFFF0BADC0DE
    call rax
    ret