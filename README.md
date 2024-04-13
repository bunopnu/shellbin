# ShellBin

Ship your shell script as executable binary.

This tool does not require any other tools such as a compiler or an assembler to generate a binary. The generated binary simply invokes the `system` function, which executes the specified shell script. It allows executing shell commands from within a program. It's important to note that the length of the shell script provided to this tool is limited to **8148** characters and this tool **only supports Microsoft Windows**.

## Options

- `-s, --script <SCRIPT>`: Sets the shell script to assembly
- `-f, --file <FILE>`: Sets the input file containing the shell script
- `-o, --output <OUTPUT>`: Sets the output file path (default: "output.exe")

## Example

```
$ shellbin -s "echo testing out! && ping bun.rip" -o out.exe
Wrote 9728 bytes to: out.exe

$.\out.exe
testing out!

Pinging bun.rip [172.67.144.227] with 32 bytes of data:
Reply from 172.67.144.227: bytes=32 time=65ms TTL=48
Reply from 172.67.144.227: bytes=32 time=63ms TTL=48
Reply from 172.67.144.227: bytes=32 time=63ms TTL=48
Reply from 172.67.144.227: bytes=32 time=98ms TTL=48

Ping statistics for 172.67.144.227:
    Packets: Sent = 4, Received = 4, Lost = 0 (0% loss),
Approximate round trip times in milli-seconds:
    Minimum = 63ms, Maximum = 98ms, Average = 72ms
```

## License

ShellBin is licensed under the MIT license.
