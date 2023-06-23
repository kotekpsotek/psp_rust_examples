# **PSP-Rust-Examples**

## Short description
Draft contains examples how you can use Rust to make something for PlayStation Portable "PSP" (that small portable Sony gaming console from 2004 - 2014). 'psp' is a create which is using to make interactions with PSP.

## Longer description
Project uses "psp" crate which ofers port over original C language PSP API (API named with starting as 'sce'). When you'd to draw something using original 'sce' graphic library ('sceGu'/'sceGum' sides) you must in some way co-operate with C language like: adjust Vertex struct to C language memory struct layout or by usage Rust core::ffi crate substitutes of C language types like: c_void as C void. **Disclaimer:** <u>Co-operations with C occurs in all 'psp' library which is obvious due to C is original PSP API language</u>. On wide avaiable internet aren't as many examples describing working with 'sce' in Rust as in C language but in original C aren't also many. 'sceGu' piece of 'sce' API works in similar way as OpenGL. The simplier way to drawning graphic on psp screen is to use 'embedded-graphic' crate but durning usage I saw that drawed shapes isn't as sharp as in 'sce', I treat this as a well deal with simplier over torought in some kind. The other sources where you can find examples working with 'psp' API are:  [Original 'psp' crate Authors Github Repository](https://github.com/overdrivenpotato/rust-psp/tree/master/examples) (mainly in: Rust language), [IrideScence YouTube Tutorial series](https://www.youtube.com/@Iridescence/videos) (C, Zig and Rust), there are also forums for  PSP enthusiasts like: [one from forums bunch](https://psp-archive.github.io/).

## How works around repo:
1. You must download 'cargo-psp' rust binary to compile Rust to PSP executable: **cargo install cargo-psp**
2. Let go to 'src/main.rs' and uncomment interesting you example from 'unsafe {}' block surrounded in 'psp_main' entry function,
3. Open your terminal emulator (like windows PowerShell) and type command: **cargo psp**. This command will compile project to form of PSP executable targeted to some from 'target' folder subfolders,
4. Is done! You can now launch your program by send it to your PlayStation Portable or by emulating PSP environment on your computer

## How to launch compiled PSP program?
There are two known me manners: 
1. Emulating psp environment using emulator such as [PPSSPP (PlayStation Portable Simulator Suitable for Playing Portably)](https://www.ppsspp.org/),
2. Send resulting executable to your PSP across USB wire

### **Special aprecations**
For other persons like mentioned above [IrideScence](https://www.youtube.com/@Iridescence) and 'psp' API Rust port crate owner [overdrivenpotato](https://github.com/overdrivenpotato/) whose work helped me

## **License**
This project is shared under rights of MIT License ([source](./LICENSE))