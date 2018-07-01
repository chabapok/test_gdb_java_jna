package test;

import com.sun.jna.Native;

public class Main {


    public static void main(String[] args) throws Exception {
        IFoo foo = Native.loadLibrary("foo", IFoo.class);

        long pid = ProcessHandle.current().pid();
        System.out.println("Waiting for atach gdb to pid "+pid);
        System.in.read();
        System.out.println("wait ok");

        foo.foo((byte) 42);
    }

}
