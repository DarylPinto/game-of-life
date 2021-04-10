## Building for Windows release
Prerequisits:
 - `rc.exe` from the [Windows SDK](https://developer.microsoft.com/en-us/windows/downloads/windows-10-sdk)
 - `windres.exe` and `ar.exe` from [minGW64](http://mingw-w64.org)

```
cargo build --release
```

See https://github.com/mxre/winres for details

## Building for Mac release
```
cargo install cargo-bundle
cargo bundle --release
```

See https://github.com/burtonageo/cargo-bundle for details
