use std::{
    fs,
    io::{self, Read},
    mem,
};

use glad_gl::gl;

pub struct ShaderError {
    pub error_msg: String,
}

impl From<io::Error> for ShaderError {
    fn from(from: io::Error) -> ShaderError {
        ShaderError {
            error_msg: format!("Error while processing shaders:\n{}", from.to_string()),
        }
    }
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
    fn build(filename: &'static str, shader_type: ShaderType) -> Result<Self, ShaderError> {
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
    pub fn build(vs_src: &'static str, fs_src: &'static str) -> Result<ShaderProgram, ShaderError> {
        let shaders = [
            Self::compile_shader(ShaderSource::build(vs_src, ShaderType::VertexShader)?)?,
            Self::compile_shader(ShaderSource::build(fs_src, ShaderType::FragmentShader)?)?,
        ];

        Ok(ShaderProgram {
            program_id: Self::link_shaders(&shaders)?,
        })
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.program_id);
        }
    }

    fn compile_shader(shader: ShaderSource) -> Result<u32, ShaderError> {
        unsafe {
            let shader_type = match shader.shader_type {
                ShaderType::VertexShader => gl::VERTEX_SHADER,
                ShaderType::FragmentShader => gl::FRAGMENT_SHADER,
                //ShaderType::GeometryShader => gl::GEOMETRY_SHADER,
            };

            let shader_id = match gl::CreateShader(shader_type) {
                0 => {
                    return Err(ShaderError {
                        error_msg: String::from("Failed to create shader"),
                    })
                }
                shader => shader,
            };

            let shader_src: *const i8 = mem::transmute(shader.source.get_unchecked(0));

            gl::ShaderSource(shader_id, 1, &shader_src, std::ptr::null());
            gl::CompileShader(shader_id);

            match Self::check_for_errors(
                shader_id,
                gl::COMPILE_STATUS,
                gl::GetShaderiv,
                gl::GetShaderInfoLog,
            ) {
                Ok(_) => Ok(shader_id),
                Err(msg) => Err(ShaderError {
                    error_msg: format!("Failed to compile '{}':\n{}\n", shader.filename, msg),
                }),
            }
        }
    }

    fn link_shaders(shaders: &[u32]) -> Result<u32, ShaderError> {
        unsafe {
            let program_id = gl::CreateProgram();

            for shader in shaders {
                gl::AttachShader(program_id, *shader);
            }
            gl::LinkProgram(program_id);

            match Self::check_for_errors(
                program_id,
                gl::LINK_STATUS,
                gl::GetProgramiv,
                gl::GetProgramInfoLog,
            ) {
                Ok(_) => Ok(program_id),
                Err(msg) => Err(ShaderError {
                    error_msg: format!("Failed to link shaders:\n{}", { msg }),
                }),
            }
        }
    }

    pub fn check_for_errors(
        target: u32,
        log_type: u32,
        get_status: unsafe fn(u32, u32, *mut i32),
        get_logs: unsafe fn(u32, i32, *mut i32, *mut i8),
    ) -> Result<(), String> {
        unsafe {
            let mut status: i32 = 0;
            get_status(target, log_type, &mut status);

            if status == 0 {
                let mut err_buff: Vec<u8> = vec![0; 512];
                let mut err_length = 0;

                get_logs(
                    target,
                    512,
                    &mut err_length,
                    mem::transmute(err_buff.get_unchecked_mut(0)),
                );
                return Err(String::from_utf8(err_buff)
                    .expect("Compilation error message should conform to UTF-8"));
            }
        }

        Ok(())
    }
}
