use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::process::Command;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Add(i64),
    Mul(i64),
    Div(i64),
    MovePtr(isize),
    Zero,
    MoveVal,
    CopyVal,
    SwapVal,
    LoopStart(usize),
    LoopEnd(usize),
    PrintChar,
    PrintNum,
    ReadChar,
    ReadNum,
    Sleep, // T: 현재 테이프 값만큼 대기(초)
}

pub struct Compiler;

impl Compiler {
    pub fn compile(source: &str) -> Vec<Op> {
        let chars: Vec<char> = source.chars().collect();
        let mut ops = Vec::new();
        let mut i = 0;
        let len = chars.len();

        while i < len {
            let ch = chars[i];

            if ch.is_whitespace() {
                i += 1;
                continue;
            }

            if ch.is_ascii_digit() {
                let num = ch.to_digit(10).unwrap() as i64;
                let mut next_idx = i + 1;
                while next_idx < len && chars[next_idx].is_whitespace() {
                    next_idx += 1;
                }

                if next_idx < len {
                    let next_ch = chars[next_idx];
                    let mut matched = true;

                    match next_ch {
                        '+' => ops.push(Op::Add(num)),
                        '-' => ops.push(Op::Add(-num)),
                        '*' => ops.push(Op::Mul(num)),
                        '/' => ops.push(Op::Div(num)),
                        '>' => ops.push(Op::MovePtr(num as isize)),
                        '<' => ops.push(Op::MovePtr(-(num as isize))),
                        _ => matched = false,
                    }

                    if matched {
                        i = next_idx + 1;
                        continue;
                    }
                }
            }

            match ch {
                '+' => ops.push(Op::Add(1)),
                '-' => ops.push(Op::Add(-1)),
                '*' => ops.push(Op::Mul(1)),
                '>' => ops.push(Op::MovePtr(1)),
                '<' => ops.push(Op::MovePtr(-1)),
                '0' => ops.push(Op::Zero),
                'M' => ops.push(Op::MoveVal),
                'C' => ops.push(Op::CopyVal),
                'X' => ops.push(Op::SwapVal),
                '[' => ops.push(Op::LoopStart(0)),
                ']' => ops.push(Op::LoopEnd(0)),
                '.' => ops.push(Op::PrintChar),
                ':' => ops.push(Op::PrintNum),
                ',' => ops.push(Op::ReadChar),
                '$' => ops.push(Op::ReadNum),
                'T' => ops.push(Op::Sleep), // T 키워드 추가
                _ => {}
            }

            i += 1;
        }

        let mut loop_stack = Vec::new();
        for pc in 0..ops.len() {
            match ops[pc] {
                Op::LoopStart(_) => loop_stack.push(pc),
                Op::LoopEnd(_) => {
                    if let Some(start) = loop_stack.pop() {
                        ops[start] = Op::LoopStart(pc);
                        ops[pc] = Op::LoopEnd(start);
                    } else {
                        panic!("구문 오류: 짝이 맞지 않는 ']' 가 있습니다.");
                    }
                }
                _ => {}
            }
        }

        ops
    }

    // BFX 명령어 배열을 C# 소스코드로 변환하는 트랜스파일러
    pub fn to_csharp(ops: &[Op]) -> String {
        let mut cs = String::new();
        cs.push_str("using System;\n");
        cs.push_str("using System.Threading;\n"); // Thread.Sleep 사용을 위해 추가
        cs.push_str("class Program {\n");
        cs.push_str("    static void Main() {\n");
        cs.push_str("        long[] tape = new long[30000];\n");
        cs.push_str("        int ptr = 0;\n\n");

        for op in ops {
            match op {
                Op::Add(v) => cs.push_str(&format!("        tape[ptr] += {};\n", v)),
                Op::Mul(v) => cs.push_str(&format!("        tape[ptr] *= {};\n", v)),
                Op::Div(v) => cs.push_str(&format!("        if ({0} != 0) tape[ptr] /= {0};\n", v)),
                Op::MovePtr(d) => cs.push_str(&format!("        ptr += {};\n", d)),
                Op::Zero => cs.push_str("        tape[ptr] = 0;\n"),
                Op::MoveVal => cs.push_str("        tape[ptr + 1] += tape[ptr]; tape[ptr] = 0;\n"),
                Op::CopyVal => cs.push_str("        tape[ptr + 1] = tape[ptr];\n"),
                Op::SwapVal => cs.push_str("        { long tmp = tape[ptr]; tape[ptr] = tape[ptr + 1]; tape[ptr + 1] = tmp; }\n"),
                Op::LoopStart(_) => cs.push_str("        while (tape[ptr] != 0) {\n"),
                Op::LoopEnd(_) => cs.push_str("        }\n"),
                Op::PrintChar => cs.push_str("        Console.Write((char)(tape[ptr] & 0xFF));\n"),
                Op::PrintNum => cs.push_str("        Console.Write(tape[ptr]);\n"),
                Op::ReadChar => cs.push_str("        tape[ptr] = Console.Read();\n"),
                Op::ReadNum => cs.push_str("        if (long.TryParse(Console.ReadLine(), out long n)) tape[ptr] = n;\n"),
                Op::Sleep => cs.push_str("        if (tape[ptr] > 0) Thread.Sleep((int)(tape[ptr] * 1000));\n"), // 1초 = 1000ms
            }
        }

        cs.push_str("    }\n");
        cs.push_str("}\n");
        cs
    }
}

pub struct BfxVM {
    tape: Vec<i64>,
    ptr: usize,
}

impl BfxVM {
    pub fn new() -> Self {
        Self {
            tape: vec![0; 30000],
            ptr: 0,
        }
    }

    pub fn run(&mut self, ops: &[Op]) {
        let mut pc = 0;
        let len = ops.len();

        while pc < len {
            match unsafe { ops.get_unchecked(pc) } {
                Op::Add(val) => self.tape[self.ptr] = self.tape[self.ptr].wrapping_add(*val),
                Op::Mul(val) => self.tape[self.ptr] = self.tape[self.ptr].wrapping_mul(*val),
                Op::Div(val) => {
                    if *val != 0 {
                        self.tape[self.ptr] /= val;
                    }
                }
                Op::MovePtr(delta) => {
                    self.ptr = (self.ptr as isize + delta) as usize;
                    if self.ptr >= self.tape.len() {
                        self.tape.resize(self.ptr + 1000, 0);
                    }
                }
                Op::Zero => self.tape[self.ptr] = 0,
                Op::MoveVal => {
                    let next = self.ptr + 1;
                    self.tape[next] = self.tape[next].wrapping_add(self.tape[self.ptr]);
                    self.tape[self.ptr] = 0;
                }
                Op::CopyVal => {
                    let next = self.ptr + 1;
                    self.tape[next] = self.tape[self.ptr];
                }
                Op::SwapVal => {
                    let next = self.ptr + 1;
                    self.tape.swap(self.ptr, next);
                }
                Op::LoopStart(target) => {
                    if self.tape[self.ptr] == 0 {
                        pc = *target;
                    }
                }
                Op::LoopEnd(target) => {
                    if self.tape[self.ptr] != 0 {
                        pc = *target;
                    }
                }
                Op::PrintChar => {
                    print!("{}", self.tape[self.ptr] as u8 as char);
                    io::stdout().flush().unwrap();
                }
                Op::PrintNum => {
                    print!("{}", self.tape[self.ptr]);
                    io::stdout().flush().unwrap();
                }
                Op::ReadChar => {
                    let mut buf = [0; 1];
                    if io::stdin().read_exact(&mut buf).is_ok() {
                        self.tape[self.ptr] = buf[0] as i64;
                    }
                }
                Op::ReadNum => {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    if let Ok(num) = input.trim().parse::<i64>() {
                        self.tape[self.ptr] = num;
                    }
                }
                Op::Sleep => {
                    let secs = self.tape[self.ptr];
                    if secs > 0 {
                        thread::sleep(Duration::from_secs(secs as u64));
                    }
                }
            }
            pc += 1;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        let first_arg = &args[1];

        // "bfx build 파일명.bfx" -> .exe 바이너리로 빌드
        if first_arg == "build" {
            if args.len() < 3 {
                println!("사용법: bfx build <파일경로.bfx>");
                return;
            }
            build_exe(&args[2]);
            return;
        }

        // "bfx run 파일명.bfx" -> 즉시 인터프리터 실행
        if first_arg == "run" {
            if args.len() < 3 {
                println!("사용법: bfx run <파일경로.bfx>");
                return;
            }
            run_file(&args[2]);
            return;
        }

        if first_arg == "--help" || first_arg == "-h" || first_arg == "help" {
            println!("BFX Programming Language CLI");
            println!("  bfx                   - REPL 대화형 쉘 실행");
            println!("  bfx run <파일>         - BFX 소스코드 파일 인터프리터 실행");
            println!("  bfx build <파일>       - BFX 소스코드를 단독 실행 파일(.exe)로 빌드");
            println!("  bfx <파일>             - BFX 소스코드 파일 간편 실행");
            return;
        }

        run_file(first_arg);
        return;
    }

    // REPL 대화형 모드
    println!("==========================================");
    println!("   Welcome to BFX Interactive Shell (REPL)");
    println!("   종료하려면 'exit' 또는 'quit'을 입력하세요.");
    println!("==========================================");

    let mut vm = BfxVM::new();

    loop {
        print!("bfx> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let input_str = input.trim();

        if input_str == "exit" || input_str == "quit" {
            println!("BFX shell을 종료합니다. 안녕히 가세요!");
            break;
        }

        if !input_str.is_empty() {
            let ops = Compiler::compile(input_str);
            vm.run(&ops);
            println!();
        }
    }
}

fn run_file(file_path: &str) {
    let bfx_code = fs::read_to_string(file_path)
        .expect("파일을 읽을 수 없습니다. 경로를 확인해 주세요.");

    let ops = Compiler::compile(&bfx_code);
    let mut vm = BfxVM::new();
    vm.run(&ops);
}

// csc.exe를 활용한 .exe 빌드 함수
fn build_exe(file_path: &str) {
    println!("[1/3] BFX 소스코드를 읽는 중: {}", file_path);
    let bfx_code = fs::read_to_string(file_path)
        .expect("파일을 읽을 수 없습니다. 경로를 확인해 주세요.");

    let ops = Compiler::compile(&bfx_code);
    let cs_code = Compiler::to_csharp(&ops);

    let temp_cs = "temp_bfx.cs";
    let exe_name = file_path.replace(".bfx", "") + ".exe";

    fs::write(temp_cs, cs_code).expect("임시 C# 소스코드 생성 실패");

    println!("[2/3] 윈도우 C# 컴파일러(csc.exe)로 .exe 빌드 중...");

    let csc_paths = [
        r"C:\Windows\Microsoft.NET\Framework64\v4.0.30319\csc.exe",
        r"C:\Windows\Microsoft.NET\Framework\v4.0.30319\csc.exe",
        "csc.exe",
    ];

    let mut success = false;

    for csc in &csc_paths {
        let output = Command::new(csc)
            .arg(format!("/out:{}", exe_name))
            .arg("/nologo")
            .arg("/optimize")
            .arg(temp_cs)
            .output();

        if let Ok(out) = output {
            if out.status.success() {
                success = true;
                break;
            }
        }
    }

    let _ = fs::remove_file(temp_cs);

    if success {
        println!("[3/3] ✨ 성공적으로 독립 실행 파일이 생성되었습니다: {}", exe_name);
        println!("이제 Terminal/cmd에서 '{}' 명령어로 직접 실행할 수 있습니다!", exe_name);
    } else {
        println!("[오류] csc.exe를 찾을 수 없거나 빌드에 실패했습니다.");
    }
}