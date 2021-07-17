# Winwrap DLL injection

An example of DLL injection with Winwrap.

- mydll: A test DLL.
- dll_injector: DLL injector.

# Usage

1. Build

```
cargo build --release
```

2. Execute a target process (e.g., notepad.exe)
   
3. Check the pid of the target process.

4. Execute

```
target\release\dll_injector.exe [pid] C:\path\to\the\mydll.dll
```
