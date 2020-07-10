# Malloc with Rust

Refs Projects:

[Simple Heap Memory ALLocator](https://github.com/CCareaga/heap_allocator)

Document

 - https://medium.com/@andrestc/implementing-malloc-and-free-ba7e7704a473

Code:

 - https://github.com/lattera/glibc/blob/master/malloc/malloc.c
 - https://github.com/OpenSIPS/opensips/tree/master/mem

Origin:

 - http://gee.cs.oswego.edu/pub/misc/malloc.c
 - http://gee.cs.oswego.edu/dl/html/malloc.html

See in [docs](docs/)

### FreeRTOS Mem

Documens: [https://www.freertos.org/a00111.html](https://www.freertos.org/a00111.html)

#### Introduction

 - heap_1 – the very simplest, does not permit memory to be freed.
 - heap_2 – permits memory to be freed, but does not coalescence adjacent free blocks.
 - heap_3 – simply wraps the standard malloc() and free() for thread safety.
 - heap_4 – coalescences adjacent free blocks to avoid fragmentation. Includes absolute address placement option.
 - heap_5 – as per heap_4, with the ability to span the heap across multiple non-adjacent memory areas.

FreeRTOS:

https://github.com/FreeRTOS/FreeRTOS-Kernel/tree/210b1ffcc87bcff93871a37fbf0ad2033870ecaf/portable/MemMang


