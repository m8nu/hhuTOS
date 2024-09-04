; ╔═════════════════════════════════════════════════════════════════════════╗
; ║ Module: thread                                                          ║
; ╟─────────────────────────────────────────────────────────────────────────╢
; ║ Descr.: Assembly function for starting a thread and switching between   ║
; ║         threads.                                                        ║
; ╟─────────────────────────────────────────────────────────────────────────╢
; ║ Author: Michael Schoettner, Univ. Duesseldorf, 15.5.2023                ║
; ╚═════════════════════════════════════════════════════════════════════════╝

; exported functions
[GLOBAL _Thread_start]
[GLOBAL _Thread_switch]

[SECTION .text]
[BITS 64]

;
; fn _thread_start (stack_ptr: u64); 
;                     (rdi           );
;
; Start thread
;
_Thread_start:
;
; Hier muss Code eingefuegt werden
; Register aus dem Stack laden

    mov rsp, rdi
    popf         ; rflags pop
    pop rbp
    pop rdi
    pop rsi
    pop rdx
    pop rcx
    pop rbx
    pop rax
    pop r15
    pop r14
    pop r13
    pop r12
    pop r11
    pop r10
    pop r9
    pop r8
    ret 




;
; fn _thread_switch (now_stack_ptr: *mut u64, then_stack: u64);
;                      (rdi,                     rsi            );
;    
; Switch threads
;
;    now_stack_ptr: This is a pointer to 'stack_ptr' in the thread struct of
;                   the current thread. Here we save RSP
;    then_stack:    This is the value of 'stack_ptr' of the thread which we
;                   switch to. This is the RSP saved before.
;
_Thread_switch:
;
; Hier muss Code eingefuegt werden
;
    ; Aktuelles Register in den Stack speichern
    push r8
    push r9
    push r10
    push r11
    push r12
    push r13
    push r14
    push r15
    push rax
    push rbx
    push rcx
    push rdx
    push rsi 
    push rdi
    push rbp
    pushf

    ; Aktueller Stack-Pointer in rdi speichern
    mov [rdi], rsp

    ; Neuer (then_stack) Register-Pointer laden
    mov rsp, rsi

    ; Register aus dem Stack laden
    popf         ; rflags pop
    pop rbp
    pop rdi
    pop rsi
    pop rdx
    pop rcx
    pop rbx
    pop rax
    pop r15
    pop r14
    pop r13
    pop r12
    pop r11
    pop r10
    pop r9
    pop r8
    ret 
