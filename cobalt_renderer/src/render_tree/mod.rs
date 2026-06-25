// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License

//! Renderer objects that define what is drawn
//!
//! There are 4 levels to the render tree
//! - [`RenderPassNode`] defines where content is rendered to in sequential passes
//! - [`ProgramNode`] defines what shader program is run. Can be added to multiple render passes
//! - [`StateGroupNode`] defines the graphics state (such as fill mode, blending etc)
//! - [`RenderableNode`] defines what vertex data is drawn (the content)
//!
//! Shader state values and textures can be set on all nodes through various means
//! - By state value methods on state groups and renderables, which implement the [`StateContainer`] trait
//! - Through [`DefaultState`] (which implements state container) attached to a render pass and program
//! - By constant state value methods on programs
//!
//! Shader state from child nodes overwrites any parent state

mod default_state;
mod program_node;
mod render_pass_node;
mod renderable_node;
mod state_container;
mod state_group_node;

pub use default_state::*;
pub use program_node::*;
pub use render_pass_node::*;
pub use renderable_node::*;
pub use state_container::*;
pub use state_group_node::*;
