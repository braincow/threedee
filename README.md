# Simple 3D software rasterizer, written in Rust

This code is based on the OneLoneCoders C# tutorials in Youtube:

* https://youtu.be/ih20l3pJoeU
* https://youtu.be/XgMWc6LumG4

## cross-compiling for windows

This project can be cross compiled to Windows from linux AND packaged into MSI installer that bundles the SDL dll and other required files with the executable.

### fixes

For error:

```text
  = note: /usr/lib/gcc/x86_64-w64-mingw32/9.2.1/../../../../x86_64-w64-mingw32/bin/ld: /home/bcow/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-pc-windows-gnu/lib/crt2.o:crtexe.c:(.rdata$.refptr.__onexitbegin[.refptr.__onexitbegin]+0x0): undefined reference to `__onexitbegin'
          /usr/lib/gcc/x86_64-w64-mingw32/9.2.1/../../../../x86_64-w64-mingw32/bin/ld: /home/bcow/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-pc-windows-gnu/lib/crt2.o:crtexe.c:(.rdata$.refptr.__onexitend[.refptr.__onexitend]+0x0): undefined reference to `__onexitend'
          collect2: error: ld returned 1 exit status
```

The fix is to copy over OS distributions library version and overwrite the one from rust toolchain:

```sh
cd ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-pc-windows-gnu/lib/
mv crt2.o crt2.o.bak
cp /usr/x86_64-w64-mingw32/sys-root/mingw/lib/crt2.o .
```

### references

* https://stackoverflow.com/questions/56602101/i-cant-get-cross-compiling-from-ubuntu-to-windows-working
* https://exceptionshub.com/cross-compile-a-rust-application-from-linux-to-windows-2.html
