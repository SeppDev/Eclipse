use std::{collections::HashMap, path::PathBuf, process::Output};

use crate::{
    analyzer::{IRProgram, RandomString},
    execute,
};

use super::{llvm, string::BetterString};

pub fn codegen(
    project_dir: &PathBuf,
    program: IRProgram,
    mode: Mode,
    random_string: RandomString,
) -> PathBuf {
    use std::fs;

    let mut builder = Builder::new(mode.clone(), random_string);
    
    // Build directory
    let mut build_dir = project_dir.clone();
    build_dir.push("build");
    
    // Create build directory if it doesn't exists yet
    fs::create_dir_all(&build_dir).unwrap();
    
    let build_final = {
        let mut a = build_dir.clone();
        a.push("build.exe");
        a
    };

    let mut build_file: PathBuf = build_dir.clone();
    let command: String;

    // the "compiling" process
    match mode {
        Mode::LLVM => {
            build_file.push("main.ll");
            llvm::generate(program, &mut builder);        

            let mut temp_final = build_dir.clone();
            temp_final.push("assembly.asm");
            println!("{:?}", temp_final);

            execute(format!("clang -03 {} -o {}", build_file.to_string_lossy(), temp_final.to_string_lossy())).unwrap();

            command = format!("clang -O3 {} -o {}", build_file.to_string_lossy(), build_final.to_string_lossy());

        }
    }

    // Writing to build file
    fs::write(&build_file, builder.build().to_string()).unwrap();

    let output = match execute(command) {
        Ok(out) => out,
        Err(error) => panic!("{}", error),
    };

    if output.status.success() == false {
        println!("{}", output.status);
        panic!("{}", String::from_utf8(output.stderr).unwrap());
    }

    return build_dir.join("build.exe");
}

#[derive(Debug, Clone)]
pub enum Mode {
    LLVM,
    // MLIR,
    // CraneLift,
}

#[derive(Debug)]
pub struct Builder {
    random: RandomString,
    mode: Mode,
    constants: HashMap<String, String>,
    body: BetterString,
}
impl Builder {
    pub fn new(mode: Mode, random: RandomString) -> Self {
        let mut body = BetterString::new();
        match &mode {
            Mode::LLVM => start_llvm(&mut body),
        }

        Self {
            mode,
            random,
            constants: HashMap::new(),
            body,
        }
    }
    pub fn build(mut self) -> BetterString {
        match self.mode {
            Mode::LLVM => build_llvm(&mut self),
        }

        return self.body;
    }
    pub fn generate(&mut self) -> String {
        self.random.generate()
    }
    // pub fn contstant_string(&mut self, string: String) -> String {
    //     let name = format!(".str.{}", self.constants.len());
    //     self.constants.insert(name.clone(), string);
    //     return name;
    // }

    pub fn next_line(&mut self) {
        self.body.next_line();
    }
    pub fn space(&mut self) {
        self.body.push(' ');
    }
    pub fn push<T: ToString>(&mut self, value: T) {
        self.body.push(value);
    }
    pub fn pushln<T: ToString>(&mut self, value: T) {
        self.body.pushln(value);
    }
}

fn start_llvm(body: &mut BetterString) {
    body.pushln("target triple = \"x86_64-pc-windows-unkown\"\n");
    body.pushln("declare i32 @printf(i8*, ...)");
    body.pushln("@.str = private constant [4 x i8] c\"%d\\0A\\00\"\n");
    body.pushln("define void @print(i32 %a) local_unnamed_addr #0 {\nentry:");
    body.pushln("\t%str_ptr = getelementptr [4 x i8], [4 x i8]* @.str, i32 0, i32 0");
    body.pushln("\tcall i32 @printf(i8* %str_ptr, i32 %a)");
    body.pushln("\tret void");
    body.pushln("}");
    body.next_line();
}

fn build_llvm(builder: &mut Builder) {
    let contstants = builder.constants.clone();

    for (name, value) in contstants {
        builder.pushln(format!(
            "@{} = private constant [{} x i8] c\"{:?}\"",
            name,
            value.len(),
            value
        ));
    }
}
