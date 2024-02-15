use crate::gl_traits::{Bindable, Deletable};
use crate::gl_types::VertexAttributeType;
use crate::gl_vertex_attribute::VertexAttribute;
use anyhow::{anyhow, Result};
use cgmath::{Vector2, Vector3};
use gl::types::GLint;
use std::mem::size_of;

//////////////////////////////////////////////////////////////////////////////
// - Vertex -
//////////////////////////////////////////////////////////////////////////////

pub trait Vertex {
    fn size() -> usize;
    fn attributes() -> Vec<VertexAttribute>;
}

impl Vertex for Vector2<f32> {
    fn size() -> usize {
        size_of::<Vector2<f32>>()
    }

    fn attributes() -> Vec<VertexAttribute> {
        vec![VertexAttribute::new(
            0,
            2,
            VertexAttributeType::TexCoord,
            false,
            Self::size(),
            0,
        )]
    }
}

impl Vertex for Vector3<f32> {
    fn size() -> usize {
        size_of::<Vector3<f32>>()
    }

    fn attributes() -> Vec<VertexAttribute> {
        vec![
            VertexAttribute::new(0, 3, VertexAttributeType::Position, false, Self::size(), 0),
            VertexAttribute {
                index: 1,
                size: 3, // r, g, b
                attribute_type: VertexAttributeType::Color,
                normalized: false,
                stride: Self::size(),
                offset: 3 * size_of::<f32>(), // Offset after the position
            },
        ]
    }
}

impl Vertex for cgmath::Vector4<f32> {
    fn size() -> usize {
        size_of::<Self>()
    }

    fn attributes() -> Vec<VertexAttribute> {
        vec![VertexAttribute::new(
            0,
            4,
            VertexAttributeType::Color,
            false,
            Self::size(),
            0,
        )]
    }
}

impl Vertex for u32 {
    fn size() -> usize {
        size_of::<u32>()
    }

    fn attributes() -> Vec<VertexAttribute> {
        vec![]
    }
}

//////////////////////////////////////////////////////////////////////////////
// - RgbVertex -
//////////////////////////////////////////////////////////////////////////////

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RgbVertex {
    pub position: [f32; 3], // x, y, z
    pub color: [f32; 3],    // r, g, b
}

impl Vertex for RgbVertex {
    fn size() -> usize {
        size_of::<Self>()
    }

    fn attributes() -> Vec<VertexAttribute> {
        let position_attr = VertexAttribute {
            index: 0,
            size: 3, // x, y, z
            attribute_type: VertexAttributeType::Position,
            normalized: false,
            stride: Self::size(),
            offset: 0,
        };

        let color_attr = VertexAttribute {
            index: 1,
            size: 3, // r, g, b
            attribute_type: VertexAttributeType::Color,
            normalized: false,
            stride: Self::size(),
            offset: 3 * size_of::<f32>(), // Offset after the position
        };

        vec![position_attr.clone(), color_attr.clone()]
    }
}

//////////////////////////////////////////////////////////////////////////////
// - TexturedVertex -
//////////////////////////////////////////////////////////////////////////////
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TexturedVertex {
    pub position: [f32; 3],   // x, y, z
    pub color: [f32; 4],      // r, g, b, a
    pub tex_coords: [f32; 2], // uv coordinates
}

impl Vertex for TexturedVertex {
    fn size() -> usize {
        9 * size_of::<f32>() // 3 for position + 4 for color + 2 for texture coordinates
    }

    fn attributes() -> Vec<VertexAttribute> {
        let stride = Self::size();
        let color_offset = 3 * size_of::<f32>();
        let tex_coords_offset = 7 * size_of::<f32>();
        vec![
            VertexAttribute::new(0, 3, VertexAttributeType::Position, false, stride, 0),
            VertexAttribute::new(
                1,
                4,
                VertexAttributeType::Color,
                false,
                stride,
                color_offset,
            ),
            VertexAttribute::new(
                2,
                2,
                VertexAttributeType::TexCoord,
                false,
                stride,
                tex_coords_offset,
            ),
        ]
    }
}

//////////////////////////////////////////////////////////////////////////////
// - Vertex Array Object (VAO) -
//////////////////////////////////////////////////////////////////////////////

pub struct VertexArrayObject {
    id: u32,
}

impl VertexArrayObject {
    /// Create a new Vertex Array Object.
    pub fn new() -> Result<VertexArrayObject> {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
            if id == 0 {
                return Err(anyhow!("Failed to generate a vertex array object"));
            }
        }
        Ok(VertexArrayObject { id })
    }

    pub fn new_and_bind() -> Result<VertexArrayObject> {
        let mut vao = VertexArrayObject::new()?;
        vao.bind()?;
        Ok(vao)
    }

    pub fn get_vertex_array_id(&self) -> u32 {
        self.id
    }
}

impl Bindable for VertexArrayObject {
    type Target = VertexArrayObject;

    fn bind(&mut self) -> Result<&mut Self::Target> {
        unsafe {
            gl::BindVertexArray(self.id);
        }
        Ok(self)
    }

    fn unbind(&mut self) -> Result<&mut Self::Target> {
        unsafe {
            gl::BindVertexArray(0);
        }
        Ok(self)
    }

    fn is_bound(&self) -> bool {
        let mut current_vao = 0;
        unsafe {
            gl::GetIntegerv(gl::VERTEX_ARRAY_BINDING, &mut current_vao);
        }
        current_vao == self.id as GLint
    }
}

impl Deletable for VertexArrayObject {
    fn delete(&mut self) -> Result<()> {
        if self.id != 0 {
            unsafe {
                gl::DeleteVertexArrays(1, &self.id);
            }
            self.id = 0;
        }
        Ok(())
    }
}

impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        if let Err(err) = self.delete() {
            eprintln!("Error while dropping VertexArrayObject: {}", err);
            // You might choose to log the error or take other appropriate actions here.
        }
    }
}