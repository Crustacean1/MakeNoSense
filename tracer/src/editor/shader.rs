use glium::program::ProgramChooserCreationError;
use glium::ProgramCreationError;
use std::io::Read;
use std::{fs, io};

use crate::AppError;

impl From<io::Error> for AppError {
    fn from(from: io::Error) -> AppError {
        AppError {
            error_msg: format!("Error while loading shaders:\n{}", from.to_string()),
        }
    }
}

impl From<ProgramCreationError> for AppError {
    fn from(from: ProgramCreationError) -> AppError {
        AppError {
            error_msg: format!("Error while compiling shaders:\n{}", from.to_string()),
        }
    }
}

impl From<ProgramChooserCreationError> for AppError {
    fn from(from: ProgramChooserCreationError) -> AppError {
        AppError {
            error_msg: format!("Error while compiling shaders:\n{}", from.to_string()),
        }
    }
}

trait Shader {
    type Uniform;
    fn load(data: &Self::Uniform) {}
}

enum ShaderType {
    VertexShader,
    FragmentShader,
}

struct ShaderSource {
    pub filename: &'static str,
    pub source: Vec<u8>,
    pub shader_type: ShaderType,
}

impl ShaderSource {
    fn build(filename: &'static str, shader_type: ShaderType) -> Result<Self, AppError> {
        Ok(ShaderSource {
            filename,
            source: Self::open_file(filename)?,
            shader_type,
        })
    }

    fn open_file(filepath: &str) -> Result<Vec<u8>, io::Error> {
        let mut file = fs::File::open(filepath)?;
        let file_size = file.metadata()?.len() as usize;

        let mut file_buffer: Vec<u8> = Vec::with_capacity(file_size);

        file.read_to_end(&mut file_buffer)?;

        Ok(file_buffer)
    }
}

pub struct ShaderProgram {
    pub program_id: u32,
}

impl ShaderProgram {
    pub fn build(
        display: &glium::Display,
        vs_src: &str,
        fs_src: &str,
    ) -> Result<glium::Program, AppError> {
        let (vertex, fragment) = (
            ShaderSource::open_file(vs_src)?,
            ShaderSource::open_file(fs_src)?,
        );

        let (vertex, fragment) = (
            std::str::from_utf8(vertex.as_slice()).expect("Non UTF8-characters in shader file"),
            std::str::from_utf8(fragment.as_slice()).expect("Non UTF8-characters in shader file"),
        );

        let program = glium::Program::from_source(display, vertex, fragment, None)?;
        Ok(program)
    }
}
