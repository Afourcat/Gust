//! Every traits needed by drawable object
//!

use nalgebra::{Vector2};
use nalgebra;
use texture::Texture;
use shader::Shader;
use std::rc::Rc;
use nalgebra::Matrix4;
use gl;

//----------------------------------------------------------------------------
//
//
//                             BLENDMODE : ENUM
//
//
//----------------------------------------------------------------------------

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
}

//----------------------------------------------------------------------------
//
//
//                             CONTEXT : STRUCT
//
//
//----------------------------------------------------------------------------

/// Context needed to handle a draw of a vertex array
/// A context is needed by the drawer to handle the drawing
/// process a default context can be use ether
pub struct Context<'a> {
	texture: Option<&'a Texture>,
	shader: Shader,
    transform: Matrix4<f32>,
	blend_mode: BlendMode,
}

impl<'a> Context<'a> {

    /// Create a new context from texture, shader, transform, blend_mode
    pub fn new(
        texture: Option<&'a Texture>,
        shader: Shader,
        transform: Option<Matrix4<f32>>,
        blend_mode: BlendMode
    ) -> Context<'a> {
        Context {
            texture: texture,
            shader: shader,
            transform: transform.unwrap_or(Matrix4::identity()),
            blend_mode: blend_mode
        }
    }

    /// Apply texture on the context
    pub fn apply_texture(&self, id: i32) {
        if let Some(texture) = self.texture {
            texture.active(id);
        }
    }

    /// Apply the graphical projection
    pub fn apply_projection(&mut self, projection: &Matrix4<f32>) {
        self.transform = projection * self.transform;
    }

    /// Apply final shader (transformation)
    pub fn setup_shader(&self) {
        self.shader.activate();
        self.shader.uniform_mat4f("transform", &self.transform);
    }
}

impl<'a> Default for Context<'a> {

    /// Default Context implementation
	fn default() -> Context<'a> {
		Context {
			texture: None,
			shader: Shader::default(),
            transform: Matrix4::identity(),
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
	/// Function that draw on itself
	fn draw<T: Drawable>(&mut self, drawable: &T);

    /// Draw with context fonction if you want to define you own fonction
    fn draw_with_context<T: Drawable>(&mut self, drawable: &T, context: &mut Context);

    fn get_projection(&self) -> &Matrix4<f32>;
}

/// Trait that can be use to draw on window
pub trait Drawable {

    /// Draw the drawable structure, you need a Drawer(Where the struct will be draw)
    fn draw<T: Drawer>(&self, window: &mut T);

    /// Draw with a particular context
    fn draw_with_context<T: Drawer>(&self, window: &mut T, context: &mut Context);

    /// Assign a texture to a drawable
    fn set_texture(&mut self, texture: &Rc<Texture>);

    /// Update the openGL state of the drawable entity
    /// Should be call often so be carefull when implementing.
    fn update(&mut self);

    /// Setup the draw for the final system you don't have to implement it in a normal drawable
    fn setup_draw<T: Drawer>(&self, context: &mut Context, window: &T) {
        context.apply_projection(window.get_projection());
        context.apply_texture(0);
        //context.apply_blendmode();
        context.setup_shader();
    }
}

/// Trait defining movable structures as sprite or higher
pub trait Movable {
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
}
