# Project introduction

`unifetch` is a tool used to show your computer hardware configuration information. It slows than `fastfetch` and shows fewer information. If you need a tool to show off how advanced your devices are, `fastfetch` should be what you need.

`unifetch` is for study only. It is completely free for everyone.

# Project structure

`unifetch` is written in `Rust`. `src` directory contains all source files. the functionalities of each mod are below:

- `cli`: Receive and analyse the arguments.
- `system`: Defined how various hardware components organize their information.
- `system_info`: Defined how to get various hardware components' information.

# Project usage

1. Download `Release` version of the project, then add the directory contains `unifetch.exe` into your system environmental variables.
2. Execute `unifetch` in your shell.
3. You can add following parameters to configure `unifetch`'s behaviour.

| Parameter | Alias |          Possible Value          |             Description             |
|:---------:|:-----:|:--------------------------------:|:-----------------------------------:|
| `--style` | `-s`  | `default`, `minimal`, `detailed` | Set the style of information output |
