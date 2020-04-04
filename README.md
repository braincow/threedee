### references

https://exceptionshub.com/cross-compile-a-rust-application-from-linux-to-windows-2.html

### fixes

For error:

```
  = note: /usr/lib/gcc/x86_64-w64-mingw32/9.2.1/../../../../x86_64-w64-mingw32/bin/ld: /home/bcow/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-pc-windows-gnu/lib/crt2.o:crtexe.c:(.rdata$.refptr.__onexitbegin[.refptr.__onexitbegin]+0x0): undefined reference to `__onexitbegin'
          /usr/lib/gcc/x86_64-w64-mingw32/9.2.1/../../../../x86_64-w64-mingw32/bin/ld: /home/bcow/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-pc-windows-gnu/lib/crt2.o:crtexe.c:(.rdata$.refptr.__onexitend[.refptr.__onexitend]+0x0): undefined reference to `__onexitend'
          collect2: error: ld returned 1 exit status
```

The fix is to copy over OS distributions library version and overwrite the one from rust toolchain:

https://stackoverflow.com/questions/56602101/i-cant-get-cross-compiling-from-ubuntu-to-windows-working

```sh
cd ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-pc-windows-gnu/lib/
mv crt2.o crt2.o.bak
cp /usr/x86_64-w64-mingw32/sys-root/mingw/lib/crt2.o .
```
