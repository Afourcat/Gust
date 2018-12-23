//
//  Rust file | 2018
//  Author: Alexandre Fourcat
//  gl_utils.rs
//  module:
//! Private module. GlUtils.

use crate::vertex::Vertex;
use gl;
use gl::types::*;
use std::mem;
use std::ptr;

/// Update a vao by settings up it's sizes for each field.
/// A Buffer as to be binded.
pub(crate) unsafe fn update_vao(va: u32) {
    gl::BindVertexArray(va);
    // position (of each vertex)
    gl::VertexAttribPointer(
        0,
        2,
        gl::FLOAT,
        gl::FALSE,
        (8 * mem::size_of::<GLfloat>()) as GLsizei,
        ptr::null(),
    );
    gl::EnableVertexAttribArray(0);
    // texture coord (of each vertex)
    gl::VertexAttribPointer(
        1,
        2,
        gl::FLOAT,
        gl::FALSE,
        (8 * mem::size_of::<GLfloat>()) as GLsizei,
        (2 * mem::size_of::<GLfloat>()) as *const _,
    );
    gl::EnableVertexAttribArray(1);
    // color (of each vertex)
    gl::VertexAttribPointer(
        2,
        3,
        gl::FLOAT,
        gl::FALSE,
        (8 * mem::size_of::<GLfloat>()) as GLsizei,
        (4 * mem::size_of::<GLfloat>()) as *const _,
    );
    gl::EnableVertexAttribArray(2);
}

/// Allocate more space for a VBO or create the VBO inside the glMemory.
pub(crate) unsafe fn alloc_vbo(vb: u32, vertice: &[Vertex], buffer_type: GLenum) {
    gl::BindBuffer(gl::ARRAY_BUFFER, vb);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        (std::mem::size_of::<GLfloat>() * vertice.len() * 8) as GLsizeiptr,
        vertice.as_ptr() as *const GLvoid,
        buffer_type,
    );
}

/// Fill the VBO with a new vertice.
pub(crate) unsafe fn fill_vbo(vb: u32, vertice: &[Vertex]) {
    gl::BindBuffer(gl::ARRAY_BUFFER, vb);
    gl::BufferSubData(
        gl::ARRAY_BUFFER,
        0,
        (std::mem::size_of::<GLfloat>() * vertice.len() * 8) as GLsizeiptr,
        vertice.as_ptr() as *const GLvoid,
    );
}

/// Update the vbo by looking if it's size has changed and then updating it with the new Data.
pub(crate) unsafe fn update_vbo(vb: u32, va: u32, vertice: &[Vertex], old_len: usize, buffer_type: GLenum) {
    if old_len != vertice.len() {
        alloc_vbo(vb, vertice, buffer_type);
    } else {
        fill_vbo(vb, vertice);
    }
    update_vao(va);
}

/// Create vbo and vao.
pub fn create_vo() -> (u32, u32) {
    let (mut vao, mut vbo): (u32, u32) = (0, 0);
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::BindVertexArray(0);
    }
    (vao, vbo)
}
