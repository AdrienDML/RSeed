// External imports
use gl::types::*;
use std::{collections::HashMap, ffi::CString};
// Crate imports
//use rseed_res::{self, RessourceLoader};

#[derive(Debug,Error)]
pub enum Error {
    CanNotDetermineShaderTypeForResource {
        name: String,
    },
    CompileError {
        name: String,
        message: String,
    },
    LinkError {
        name: String,
        message: String,
    },
}

const POSSIBLE_EXT: [(&str, gl::types::GLenum); 6] = [
    (".vert", gl::VERTEX_SHADER),
    (".tesc", gl::TESS_CONTROL_SHADER),
    (".tese", gl::TESS_EVALUATION_SHADER),
    (".geom", gl::GEOMETRY_SHADER),
    (".frag", gl::FRAGMENT_SHADER),
    (".comp", gl::COMPUTE_SHADER),
];



pub struct ShaderProgram {
    _id: GLuint,
    shaders : Vec<Shader>,
    //uniforms : HashMap<String, UType>,
    gl: gl::Gl,
}

impl ShaderProgram {
    // pub fn from_res(gl: &gl::Gl, res: &RessourceLoader, name: &str) -> Result<Self, Error> {
    //     let ressources_names : Vec<(String,GLenum)> = POSSIBLE_EXT
    //         .iter()
    //         // get all coresponding names
    //         .map(|(file_ext, stype)| (format!("{}{}", name, file_ext),*stype) )
    //         // filter out the ones that don't exists
    //         .partition(|(name,stype)| res.name_to_path(&(**name).to_owned()).exists())
    //         .0;

    //     // create the actual shaders from ressources
    //     let shaders = ressources_names
    //         .iter()
    //         .map(|(ressource_name,stype)| Shader::from_res(gl, res, ressource_name, *stype))
    //         .collect::<Result<Vec<Shader>, Error>>()?;
    //     // link the shaders into a Progra
    //     ShaderProgram::from_shaders(gl, shaders).map_err(|message| Error::LinkError {
    //         name: name.to_owned(),
    //         message,
    //     })
    // }

    pub fn from_shaders(gl: &gl::Gl, shaders: Vec<Shader>) -> Result<Self, String> {
        let id = unsafe { gl.CreateProgram() };

        for shader in shaders.clone() {
            unsafe {
                gl.AttachShader(id, shader.id());
            }
        }
        unsafe {
            gl.LinkProgram(id);
        }

        let mut success: GLint = 1;
        unsafe {
            gl.GetProgramiv(id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl.GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_empty_cstring_with_len(len as usize);

            unsafe {
                gl.GetProgramInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(format!("{}", error.to_string_lossy().into_owned()));
        }

        // for shader in shaders {
        //     unsafe {
        //         gl.DetachShader(id, shader.id());
        //     }
        // }

        Ok(Self {
            _id: id,
            //uniforms : HashMap::new(),
            shaders,
            gl: gl.clone(),
        })
    }

    pub fn id(&self) -> GLuint {
        self._id
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.UseProgram(self._id);
        }
    }
    
    pub fn unbind(&self) {
        unsafe {
            self.gl.UseProgram(0 as GLuint);
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self._id);
        }
    }
}

// shader struct
#[derive(Clone)]
pub struct Shader {
    _id: GLuint,
    source: CString,
    gl: gl::Gl,
}

impl Shader {

    // pub fn from_res(gl: &gl::Gl, res: &RessourceLoader, name: &str, stype : GLenum) -> Result<Shader, Error> {
    //     let source = res.load_cstring(name).map_err(|e| Error::ResourceLoad {
    //         name: name.to_owned(),
    //         inner: e,
    //     })?;

    //     Shader::from_source(gl, source, stype).map_err(|e| Error::CompileError {
    //         name: name.to_owned(),
    //         message: e,
    //     })
    // }

    pub fn from_source(gl: &gl::Gl, source: CString, shader_type: GLenum) -> Result<Self, String> {
        let id = unsafe { gl.CreateShader(shader_type) };

        unsafe {
            gl.ShaderSource(id, 1, &source[..].as_ptr(), std::ptr::null());
            gl.CompileShader(id);
        }

        // check if the compilation was successfull and print the error message if it isn't.
        let mut success: GLint = 1;
        unsafe {
            gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl.GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_empty_cstring_with_len(len as usize);

            unsafe {
                gl.GetShaderInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(format!(
                "There was an error in the shader compilation: {}\n",
                error.to_string_lossy().into_owned()
            ));
        }

        Ok(Self {
            _id: id,
            source,
            gl: gl.clone(),
        })
    }

    pub fn id(&self) -> GLuint {
        self._id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteShader(self._id);
        }
    }
}


