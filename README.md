<div align="center">

#  BFX Programming Language

**A lightweight, Brainfuck-inspired programming language written in Rust.**

![Rust](https://shields.io)
![C#](https://shields.io)
![Windows](https://shields.io)

</div>

---

**BFX** extends the traditional tape-based Brainfuck model with additional arithmetic operations, memory manipulation, input/output commands, and execution control. It includes its own virtual machine, an interactive REPL, and a C#-based executable builder.

##  Features

*  **Custom Virtual Machine** – Executes BFX instructions using a tape-based memory system.
*  **Interactive REPL** – Write and execute BFX code directly in the terminal.
*  **File Execution** – Run `.bfx` source files effortlessly.
*  **EXE Builder** – Compile BFX programs into standalone Windows executables (`.exe`) using C#.
*  **Extended Instructions** – Additional arithmetic, memory manipulation, and I/O operations.
*  **Sleep Instruction** – Pause program execution based on the current cell's value.

---

##  Getting Started

### Installation (Windows)

Run the provided PowerShell script to install BFX on your system:

```powershell
.\install.ps1
```

### Running a BFX Program

```bash
bfx run hello.bfx
```

You can also execute a file directly:

```bash
bfx hello.bfx
```

### Interactive REPL

```bash
bfx
```
>  Type `exit` or `quit` to leave the interactive shell.

### Building an EXE

Compile your BFX source code into a standalone Windows `.exe` file:

```bash
bfx build hello.bfx
```
>  **Note:** The EXE builder requires the Windows C# compiler (`csc.exe`).

---

##  Language Reference

### Basic & Extended Commands

| Command | Description |
| :---: | :--- |
| `+` | Increment the current cell |
| `-` | Decrement the current cell |
| `*` | Multiply the current cell |
| `/` | Divide the current cell by a number |
| `>` | Move the pointer right |
| `<` | Move the pointer left |
| `0` | Set the current cell to zero |
| `M` | Move the current value to the next cell and clear the current cell |
| `C` | Copy the current value to the next cell |
| `X` | Swap the current cell with the next cell |
| `[` | Start a loop |
| `]` | End a loop |
| `.` | Output the current cell as an ASCII character |
| `:` | Output the current cell as a number |
| `,` | Read a character |
| `$` | Read a number |
| `T` | Sleep for the number of seconds stored in the current cell |

### Numeric Prefixes

BFX supports numeric prefixes for several instructions (whitespace is allowed between the number and its instruction):

| Syntax | Description |
| :---: | :--- |
| `5+` | Add 5 |
| `3-` | Subtract 3 |
| `4*` | Multiply by 4 |
| `2/` | Divide by 2 |
| `5>` | Move the pointer 5 cells right |
| `3<` | Move the pointer 3 cells left |

---

##  Example

```bfx
9+ 8* . 9+ 9+ 9+ 2+ . 7+ . . 3+ . > 4+ 8* .
> 9+ 9* 6+ . 9+ 9+ 6+ . 3+ . 6- . 8- . > 3+ 9* 6+ .
```

**Output:**
```text
Hello world!
```

---

##  Built With

* **Rust** – Core VM, parser, and CLI
* **C#** – Windows standalone executable (`.exe`) generation
* **PowerShell** – Automated installation script

##  Project Status & License

* **Status:** BFX is an experimental programming language project. Features and syntax may change as development continues.
* **License:** See the repository's `LICENSE` file for licensing information.
