# test_gdb_java_jna
Не работает команда step в другой crate. Она отрабатывает как next - без захода внутрь.


kjhkjhkjh

1. заходим в foo и делаем
    cargo build

Это соберет либу, которая будет лежать в `foo/target/debug`
Эта либа использует крейт bar, и вызывает функцию, которая находится в том крейте.

2. Устанавливаем переменную окружения LD_LIBRARY_PATH в каталог с либой. У меня получилось так:

LD_LIBRARY_PATH=/home/chabapok/IdeaProjects/testgdb/foo/target/debug

3. Запускаем. Программа на java загрузит нативную либу и будет ждать ввода с консоли, чтобы вызвать нативный метод
в этой либе. Перед заходом в ожидаемние она печатает в консоль свой pid.

В это время я с консоли запускаю gdb или lldb:
```
chabapok@chabapok-xubuntu:~/IdeaProjects/testgdb$ gdb -p 17208
GNU gdb (GDB) 7.99.90.20170419-git
Copyright (C) 2017 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "x86_64-pc-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word".
Attaching to process 17208
[New LWP 17209]
[New LWP 17221]
[New LWP 17222]
[New LWP 17223]
[New LWP 17224]
[New LWP 17225]
[New LWP 17226]
[New LWP 17227]
[New LWP 17228]
[New LWP 17229]
[New LWP 17230]
[New LWP 17231]
[New LWP 17235]
[New LWP 17241]
[New LWP 17243]
[New LWP 17244]
[New LWP 17245]
[New LWP 17246]
[New LWP 17247]
[New LWP 17248]
[New LWP 17249]
[New LWP 17262]
[New LWP 17264]
[New LWP 17265]
[New LWP 17274]

warning: Could not load shared library symbols for /tmp/jna-1421167249/jna11799513901868596759.tmp.
Do you need "set solib-search-path" or "set sysroot"?
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
0x00007fcc7e79166b in pthread_join () from /lib/x86_64-linux-gnu/libpthread.so.0
warning: Missing auto-load script at offset 0 in section .debug_gdb_scripts
of file /home/chabapok/IdeaProjects/testgdb/foo/target/debug/libfoo.so.
Use `info auto-load python-scripts [REGEXP]' to list them.
(gdb) b foo
Breakpoint 1 at 0x7fcc252afe10: file src/lib.rs, line 9.
(gdb) c
Continuing.



(тут я нажимаю any key в программе и мы попадаем на точку оставнова)



[Switching to Thread 0x7fcc7eda6700 (LWP 17209)]

Thread 2 "java" hit Breakpoint 1, foo (u=42) at src/lib.rs:9
9	    println!("Foo from rust!");
(gdb) n
10	    println!("Foo from rust 2");
(gdb)
12	    let t = bar::bar(u as u8);
(gdb) s
13	    let t2 = localbar(t as i8);
(gdb)
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ вот тут на 12 строке оно не зашло в bar::bar. Какого хрена?
дальше я в программе проверяю, что оно заходит по команде s если это вызов локальных модулей.

c lldb происходит та же хрень: команда s (`step`) не заходит внутрь, если это вызов внешнего крейта.

Почему оно может так работать? Если мы дебажим просто программу, которая использует другой крейт, то такой
проблемы не возникает.
```

Впринципе, проблема решается командой si - шаги по ассемблерным инструкциям, просто так делать неудобно.