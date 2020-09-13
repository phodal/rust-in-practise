# VM

cloned from:

 - [https://github.com/NishanthSpShetty/stack-langc](https://github.com/NishanthSpShetty/stack-langc)
 - [https://github.com/NishanthSpShetty/Stack-VM](https://github.com/NishanthSpShetty/Stack-VM)
 
 类型
 
 | 语言 | 虚拟机  | 架构  |
 |------|------|--------|
 | Java | JVM |  栈式虚拟机 |
 | Java | Dalvik | 寄存器式虚拟机 |
 | Ruby |  YARV | 栈式虚拟机 |
 | Ruby | mruby | 寄存器式虚拟机 |
 | Lua  | lua   |  寄存器式虚拟机 |
 | Python | CPython |  栈式虚拟机 |
 
 [栈式虚拟机和寄存器式虚拟机？](https://www.zhihu.com/question/35777031)

基于栈与基于寄存器的指令集，用在解释器里，笼统说有以下对比：

 - 从源码生成代码的难度：基于栈 < 基于寄存器，不过差别不是特别大
 - 表示同样程序逻辑的代码大小（code size）：基于栈 < 基于寄存器
 - 表示同样程序逻辑的指令条数（instruction count）：基于栈 > 基于寄存器
 - 简易实现中数据移动次数（data movement count）：基于栈 > 基于寄存器；不过值得一提的是实现时通过栈顶缓存（top-of-stack caching）可以大幅降低基于栈的解释器的数据移动开销，可以让这部分开销跟基于寄存器的在同等水平。请参考另一个回答：寄存器分配问题？ - RednaxelaFX 的回答
 - 采用同等优化程度的解释器速度：基于栈 < 基于寄存器
 - 交由同等优化程度的JIT编译器编译后生成的代码速度：基于栈 === 基于寄存器
 
因而，笼统说可以有以下结论：要追求

 - 尽量实现简单：选择基于栈
 - 传输代码的大小尽量小：选择基于栈
 - 纯解释执行的解释器的速度：选择基于寄存器
 - 带有JIT编译器的执行引擎的速度：随便，两者一样；对简易JIT编译器而言基于栈的指令集可能反而更便于生成更快的代码，而对比较优化的JIT编译器而言输入是基于栈还是基于寄存器都无所谓，经过parse之后就变得完全一样了。
