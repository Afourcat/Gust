//! Every traits needed by drawable object
//!

use nalgebra::{Vector2};
use nalgebra;
use texture::Texture;
use shader::Shader;
use nalgebra::Matrix4;
use gl;
use shader::DEFAULT_SHADER;

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
    Ceta
}

impl BlendMode {
    fn blend_to_gl(&self) {
        match self {
            BlendMode::Alpha => unsafe { gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA) },
            _ => {},
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
                },
                BlendMode::Beta => {
                    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
                },
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
        blend_mode: BlendMode
    ) -> Context<'a> {
        Context {
            texture,
            shader,
            transform,
            blend_mode
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

/// Trait defining a drawer
pub trait Drawer {
    fn draw_mut<T: Drawable>(&mut self, drawable: &mut T) {
        self.draw(drawable);
    }

    /// Function that draw on itself
    fn draw<T: Drawable>(&mut self, drawable: &T);

    /// Draw with context fonction if you want to define you own fonction
    fn draw_with_context_mut<T: Drawable>(&mut self, drawable: &mut T, context: &mut Context) {
        self.draw_with_context(drawable, context);
    }

    fn draw_with_context<T: Drawable>(&mut self, drawable: &mut T, context: &mut Context) {
        drawable.draw_with_context(context);
    }

    fn get_projection(&self) -> &Matrix4<f32>;

    fn get_center(&self) -> Vector2<f32>;

    fn get_sizes(&self) -> Vector2<f32>;

    fn projection(&self) -> &Matrix4<f32>;
}

/// Trait that can be use to draw on window
pub trait Drawable {

    /// Draw the drawable structure, you need a Drawer(Where the struct will be draw)
    fn draw<T: Drawer>(&self, window: &mut T);

    /// Draw with a particular context
    fn draw_with_context(&self, context: &mut Context);

    /// Draw as mutable
    fn draw_mut<T: Drawer>(&mut self, window: &mut T) {
        self.draw(window);
    }

    /// Draw with context as mutable
    fn draw_with_context_mut(&mut self, context: &mut Context) {
        self.draw_with_context(context);
    }

    /// Update the openGL state of the drawable entity
    /// Should be call often so be carefull when implementing.
    fn update(&mut self);

    /// Setup the draw for the final system you don't have to implement it in a normal drawable
    fn setup_draw(&self, context: &mut Context) {
        context.apply_texture(0);
        context.apply_blendmode();
        context.setup_shader();
    }
}

/// Trait defining movable structures as sprite or higher
pub trait Movable {
    /// Move the sprite off the offset
    fn contain<T: nalgebra::Scalar + From<f32> + Into<f32>>(&self, offset: Vector2<T>) -> bool;

    /// Move the sprite off the offset
    fn translate<T: nalgebra::Scalar + From<f32> + Into<f32>>(&mut self, offset: Vector2<T>);

    /// Set position of the sprite
    fn set_position<T: nalgebra::Scalar + From<f32> + Into<f32>>(&mut self, pos: Vector2<T>);

    /// Get current position
    fn get_position(&self) -> Vector2<f32>;

    /// Scale the sprite from a factor
    fn scale<T: nalgebra::Scalar + From<f32> + Into<f32>>(&mut self, factor: Vector2<T>);

    /// Set the scale of the sprite
    fn set_scale<T: nalgebra::Scalar + From<f32> + Into<f32>>(&mut self, vec: Vector2<T>);

    /// Get the current scale
    fn get_scale(&self) -> Vector2<f32>;

    fn rotate<T: nalgebra::Scalar + Into<f32>>(&mut self, angle: T);

    fn set_rotation<T: nalgebra::Scalar + Into<f32>>(&mut self, angle: T);

    fn get_rotation(&self) -> f32;

    fn set_origin<T: nalgebra::Scalar + Into<f32>>(&mut self, origin: Vector2<T>);

    fn get_origin(&self) -> Vector2<f32>;
}
