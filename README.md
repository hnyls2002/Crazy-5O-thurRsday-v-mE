# Crazy-5O-thurRsday-v-mE
A toy OS

### How to test it

```
>> cd os
>> make run
```

Then you can see the initial output of the OS.

```
 __  ___  _______   ______      _______..______    __  
|  |/  / |   ____| /      |    /       ||   _  \  |  | 
|  '  /  |  |__   |  ,----'   |   (----`|  |_)  | |  | 
|    <   |   __|  |  |         \   \    |   _  <  |  | 
|  .  \  |  |     |  `----..----)   |   |  |_)  | |  | 
|__|\__\ |__|      \______||_______/    |______/  |__| 
[KFC-OS] Entering into kernel_main function!
[KFC-OS] MEMORY END ADDRESS is 0x84000000
[KFC-OS] heap test start!
[KFC-OS] heap test passed!
[KFC-OS] -----------------------kernel space-----------------------
[KFC-OS] .text                          [0x80000000, 0x8000E000)
[KFC-OS] .rodata                                [0x8000E000, 0x80013000)
[KFC-OS] .data                          [0x80013000, 0x8183E000)
[KFC-OS] .bss                           [0x8184E000, 0x81B4F000)
[KFC-OS] frame pool                     [0x81B4F000, 0x84000000)
[KFC-OS] trampoline                     [0x80001000, 0x80002000)
[KFC-OS] -----------------------kernel space-----------------------
[KFC-OS] remap_test start!
[KFC-OS] remap_test passed!
[KFC-OS] ====================The Supported Apps====================
[KFC-OS] exit
[KFC-OS] fantastic_text
[KFC-OS] forkexec
[KFC-OS] forktest
[KFC-OS] forktest2
[KFC-OS] forktest_simple
[KFC-OS] forktree
[KFC-OS] hello
[KFC-OS] initproc
[KFC-OS] load_fault
[KFC-OS] matrix
[KFC-OS] power_3
[KFC-OS] power_5
[KFC-OS] power_7
[KFC-OS] read_test
[KFC-OS] shell
[KFC-OS] sleep
[KFC-OS] sleep_simple
[KFC-OS] stack_overflow
[KFC-OS] store_fault
[KFC-OS] usertests
[KFC-OS] usertests-simple
[KFC-OS] yield
[KFC-OS] ==========================================================
zck@zck-A7S:~$ 
```

The support apps are listed in the initial output. You can run them by typing the app name in the shell.

```
zck@zck-A7S:~$ hello
[DEBUG] waiting for pid...
Hello world from user mode!
[KFC-OS] In process "hello", pid = 3, exit with code 0
```

The shell can be termiated by `Ctrl + C`.
```
zck@zck-A7S:~$ ^C
[KFC-OS] In process "shell", pid = 2, exit with code 0
init process : no child process left, exiting...
[KFC-OS] In process "initproc", pid = 1, exit with code 0
[KFC-OS] No process to schedule...
[KFC-OS] Shutdown...
[KFC-OS] Normal shutdown...
```