//! Metal framework bindings — GPU-accelerated graphics and compute.

pub mod types;
pub mod device;
pub mod command_queue;
pub mod command_buffer;
pub mod encoder;
pub mod pipeline;
pub mod library;
pub mod buffer;
pub mod texture;
pub mod sampler;
pub mod vertex;
pub mod render_pass;
pub mod depth_stencil;

pub use types::*;
pub use device::MTLDevice;
pub use command_queue::MTLCommandQueue;
pub use command_buffer::MTLCommandBuffer;
pub use encoder::{MTLRenderCommandEncoder, MTLComputeCommandEncoder, MTLBlitCommandEncoder};
pub use pipeline::{MTLRenderPipelineDescriptor, MTLRenderPipelineState};
pub use library::{MTLLibrary, MTLFunction};
pub use buffer::MTLBuffer;
pub use texture::{MTLTexture, MTLTextureDescriptor};
pub use sampler::{MTLSamplerDescriptor, MTLSamplerState};
pub use vertex::{MTLVertexDescriptor, MTLVertexAttributeDescriptor, MTLVertexBufferLayoutDescriptor};
pub use render_pass::{MTLRenderPassDescriptor, MTLRenderPassColorAttachmentDescriptor};
pub use depth_stencil::{MTLDepthStencilDescriptor, MTLDepthStencilState};
