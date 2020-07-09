# RUDP

> 在计算机网络中，可靠用户数据报协议是贝尔实验室为Plan 9操作系统设计的传输层协议。它旨在提供一种解决方案，其中UDP过于原始，因为需要保证顺序的数据包传递，但是TCP增加了太多的复杂性/开销。为了使RUDP获得更高的服务质量，RUDP实现了与TCP类似的功能，并且开销较小。

 - [Go RUDP](https://github.com/u35s/rudp)
 - [Rust Laminar](https://github.com/amethyst/laminar) is an application-level transport protocol which provides configurable reliability and ordering guarantees built on top of UDP. It focuses on fast-paced fps-games and provides a lightweight, message-based interface.
 - [Rust kcp](https://github.com/Matrix-Zhang/kcp)  is just Rust translation for KCP
 - [C KCP](https://github.com/skywind3000/kcp)  是一个快速可靠协议，能以比 TCP浪费10%-20%的带宽的代价，换取平均延迟降低 30%-40%，且最大延迟降低三倍的传输效果。纯算法实现，并不负责底层协议（如UDP）的收发，需要使用者自己定义下层数据包的发送方式，以 callback的方式提供给 KCP。


