//! Every traits needed by drawable object
//!

use crate::shader::Shader;
use crate::shader::DEFAULT_SHADER;
use crate::texture::Texture;
use crate::vertex::{Vertex, VertexArray};
use crate::vertex_buffer::{Primitive, VertexBuffer};
use gl;
use nalgebra::Matrix4;
use nalgebra::Vector2;

//----------------------------------------------------------------------------
//
//
//                             BLENDMODE : ENUM
//
//
//----------------------------------------------------------------------------

lazy_static! {
    static ref DEFAULT_CONTEXT: Context<'static> = Context::default();
}

lazy_static! {
    pub static ref IDENTITY: Matrix4<f32> = Matrix4::identity();
}

#[derive(Debug)]
/// Blend mode needed to draw
pub enum BlendMode {
    Alpha,
    Beta,
    Ceta,
}

impl BlendMode {
    fn blend_to_gl(&self) {
        match self {
            BlendMode::Alpha => unsafe { gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA) },
            _ => {}
        }
    }

    pub fn unactive(&self) {
        unsafe {
            gl::Disable(gl::BLEND);
        }
    }

    pub fn active(&self) {
        unsafe {
            gl::Enable(gl::BLEND);
            match self {
                BlendMode::Alpha => {
                    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
                }
                BlendMode::Beta => {
                    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
                }
                BlendMode::Ceta => {
                    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
                }
            }
        }
    }
}

//----------------------------------------------------------------------------
//
//
//                             CONTEXT : STRUCT
//
//
//----------------------------------------------------------------------------

#[derive(Debug)]
/// Context needed to handle a draw of a vertex array
/// A context is needed by the drawer to handle the drawing
/// process a default context can be use ether
pub struct Context<'a> {
    texture: Option<&'a Texture>,
    shader: &'a Shader,
    transform: Vec<(String, &'a Matrix4<f32>)>,
    blend_mode: BlendMode,
}

impl<'a> Context<'a> {
    /// Create a new context from texture, shader, transform, blend_mode
    pub fn new(
        texture: Option<&'a Texture>,
        shader: &'a Shader,
        transform: Vec<(String, &'a Matrix4<f32>)>,
        blend_mode: BlendMode,
    ) -> Context<'a> {
        Context {
            texture,
            shader,
            transform,
            blend_mode,
        }
    }

    /// Apply texture on the context
    pub fn apply_texture(&mut self, id: i32) {
        if let Some(texture) = self.texture {
            texture.active(id);
        }
    }

    /// Apply the blendmode to the current context
    pub fn apply_blendmode(&mut self) {
        self.blend_mode.active();
    }

    /// Apply final shader (transformation)
    pub fn setup_shader(&self) {
        self.shader.activate();
        for (name, mat) in &self.transform {
            self.shader.uniform_mat4f(name.as_str(), mat);
        }
    }

    /// Setup the draw for the final system you don't have to implement it in a normal drawable
    pub fn setup_draw(&mut self) {
        self.apply_texture(0);
        self.apply_blendmode();
        self.setup_shader();
    }
}

impl<'a> Default for Context<'a> {
    /// Default Context implementation
    fn default() -> Context<'a> {
        Context {
            texture: None,
            shader: &*DEFAULT_SHADER,
            transform: vec![("transform".to_string(), &*IDENTITY)],
            blend_mode: BlendMode::Alpha,
        }
    }
}

//----------------------------------------------------------------------------
//
//
//                             TRAIT PART
//
//
//----------------------------------------------------------------------------

/// Trait that should be implemented by all backend. It define a all way to draw something from
/// some vertex / vertex_array / vertex_buffer.
pub trait Drawer: Sized {
    /// Function call by the final user to draw a `Drawable`.
    fn draw<T: Drawable>(&mut self, drawable: &T) {
        drawable.draw(self);
    }

    /// Function call by the final user to draw a `DrawableMut`.
    fn draw_mut<T: DrawableMut>(&mut self, drawable: &mut T) {
        drawable.draw_mut(self);
    }

    fn draw_with_context<T: Drawable>(&mut self, drawable: &T, context: &mut Context);

    fn draw_with_context_mut<T: DrawableMut>(&mut self, drawable: &mut T, context: &mut Context);

    /// Draw from a slice of vertex.
    fn draw_vertices(&self, vertices: &[Vertex], primitive: Primitive, context: &mut Context);

    /// Draw from the vertex array.
    fn draw_vertex_array(&self, vertices: &VertexArray, context: &mut Context);

    /// Draw from a vertex buffer.
    fn draw_vertex_buffer(&self, vertex_buffer: &VertexBuffer, context: &mut Context);

    unsafe fn draw_from_raw(&self, raw: *const std::ffi::c_void, len: usize, context: &mut Context);

    /// Get the center of the window.
    fn center(&self) -> Vector2<f32>;

    /// Get the sizes of the current window.
    fn sizes(&self) -> Vector2<f32>;

    /// Get the projection of the current window.
    fn projection(&self) -> Matrix4<f32>;
}

/// Trait that can be use to draw on window
pub trait Drawable {
    /// Draw the drawable structure, you need a Drawer(Where the struct will be draw)
    fn draw<T: Drawer>(&self, target: &mut T);

    /// Draw with a particular context
    fn draw_with_context<T: Drawer>(&self, target: &mut T, context: &mut Context);

    /// Update the openGL state of the drawable entity
    /// Should be call often so be carefull when implementing.
    fn update(&mut self);
}

pub trait DrawableMut: Drawable {
    /// Mutable version of draw function.
    fn draw_mut<T: Drawer>(&mut self, target: &mut T) {
        self.draw(target);
    }

    /// Mutable draw context function.
    fn draw_with_context_mut<T: Drawer>(&mut self, target: &mut T, context: &mut Context) {
        self.draw_with_context(target, context);
    }
}
