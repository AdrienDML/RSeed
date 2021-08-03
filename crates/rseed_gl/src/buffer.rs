use gl::{self, types::*};
use std::mem::size_of;

pub trait BufferType {
    const TYPE: GLuint;
}

pub struct VertexBufferType;
impl BufferType for VertexBufferType {
    const TYPE: GLuint = gl::ARRAY_BUFFER;
}

pub type VertexBuffer = Buffer<VertexBufferType>;

pub struct IndexBufferType;
impl BufferType for IndexBufferType {
    const TYPE: GLuint = gl::ELEMENT_ARRAY_BUFFER;
}

pub type IndexBuffer = Buffer<IndexBufferType>;

/// Generic Buffer struct for all OpenGL buffer types
pub struct Buffer<B>
where
    B: BufferType,
{
    gl: gl::Gl,
    id: GLuint,
    _marker: std::marker::PhantomData<B>,
}

impl<B> Buffer<B>
where
    B: BufferType,
{
    /// Creates a new buffer
    pub fn new(gl: &gl::Gl) -> Self {
        let mut id: GLuint = 0;
        unsafe { gl.GenBuffers(1, &mut id) }
        Self {
            gl: gl.clone(),
            id,
            _marker: std::marker::PhantomData,
        }
    }

    /// Set the buffer data on the GPU
    pub fn set_data<T>(&self, data: &[T]) {
        self.bind();
        unsafe {
            self.gl.BufferData(
                B::TYPE,
                (data.len() * size_of::<T>()) as GLsizeiptr,
                data.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            )
        }
    }

    pub fn id(&self) -> GLuint {
        self.id
    }
    pub fn bind(&self) {
        unsafe { self.gl.BindBuffer(B::TYPE, self.id) };
    }
    pub fn unbind(&self) {
        unsafe { self.gl.BindBuffer(B::TYPE, 0 as GLuint) };
    }
}
