# nod
---

```
Build the same package twice and diffoscope

Usage: nod [OPTIONS] <NAME>

Arguments:
  <NAME>  Package to build

Options:
  -D, --duration <DURATION>            How much to wait between rebuilds (milliseconds) [default: 1000]
  -p, --path <PATH>                    Path of nixpkgs repo [default: .]
  -b, --build-command <BUILD_COMMAND>  The build command to use [default: nix-build @path -A @name --keep-failed]
  -d, --diff-command <DIFF_COMMAND>    The build command to use [default: diffoscope @1 @2 -o out.html]
  -h, --help                           Print help
  -V, --version                        Print version
```
