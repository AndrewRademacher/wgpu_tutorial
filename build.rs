use glob::glob;
use rayon::prelude::*;
use shaderc::{Compiler, ShaderKind};
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let shaders = scan_shaders("src/shaders/**/*");

    shaders.par_iter().for_each(|shader| {
        let source = read_to_string(&shader.input).unwrap();
        let mut compiler = Compiler::new().unwrap();
        let spirv = compiler
            .compile_into_spirv(
                &source,
                shader.kind,
                shader.input.to_str().unwrap(),
                "main",
                None,
            )
            .unwrap();
        let mut output = File::create(&shader.output).unwrap();
        output.write_all(spirv.as_binary_u8()).unwrap();
    });
}

#[derive(Debug)]
struct Shader {
    input: PathBuf,
    output: PathBuf,
    kind: shaderc::ShaderKind,
}

impl Shader {
    fn new(input: PathBuf) -> Shader {
        let ex = input.extension().unwrap().to_str().unwrap();
        let kind = match ex {
            "frag" => ShaderKind::Fragment,
            "vert" => ShaderKind::Vertex,
            e => panic!("Unknown extension: {:?}", e),
        };
        let mut output = input.clone();
        output.set_extension(format!("{}.spv", ex));

        Shader {
            input,
            output,
            kind,
        }
    }
}

fn scan_shaders(pattern: &str) -> Vec<Shader> {
    glob(pattern)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| match entry.extension() {
            Some(ex) => match ex.to_str() {
                Some("spv") => false,
                Some(_) => true,
                None => true,
            },
            None => false,
        })
        .map(|entry| {
            println!("cargo:rerun-if-changed={}", entry.to_str().unwrap());
            Shader::new(entry)
        })
        .collect()
}
