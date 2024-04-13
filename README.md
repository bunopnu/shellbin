# ShellBin

Ship your shell script as executable binary!

## Output

This tool doesn't need additional tools like compilers or assemblers to generate a binary.

### Windows

The generated binary invokes the `system` function to execute the specified script. **Keep in mind that some anti-malware software may flag binaries due to this behavior.**

### Linux

The generated binary uses the `execve` syscall to execute the specified shell script.

## Example

> Note that you can cross-compile your script, allowing you to compile a Bash script for Linux while using Windows, for example.

### Windows

```
$ shellbin -s "echo testing for windows! && ping bun.rip" -o output.exe -t windows
Wrote 9728 bytes to: output.exe

$ .\output.exe
testing for windows!

Pinging bun.rip [104.21.28.78] with 32 bytes of data:
Reply from 104.21.28.78: bytes=32 time=64ms TTL=48
Reply from 104.21.28.78: bytes=32 time=66ms TTL=48
Reply from 104.21.28.78: bytes=32 time=70ms TTL=48
Reply from 104.21.28.78: bytes=32 time=63ms TTL=48

Ping statistics for 104.21.28.78:
    Packets: Sent = 4, Received = 4, Lost = 0 (0% loss),
Approximate round trip times in milli-seconds:
    Minimum = 63ms, Maximum = 70ms, Average = 65ms
```

### Linux

```
$ shellbin -s "echo testing for linux! && dir" -o output -t linux
Wrote 288 bytes to: output

$ ./output
testing for linux!
Cargo.lock  Cargo.toml  LICENSE  README.md  output  src  target  testing
```

## License

ShellBin is licensed under the MIT license.
