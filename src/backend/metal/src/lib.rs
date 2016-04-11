// Copyright 2016 The Gfx-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[macro_use]
extern crate log;
extern crate objc;
extern crate cocoa;
extern crate gfx_core;

use cocoa::base::{selector, id, class, nil};
use cocoa::foundation::{NSUInteger};

mod factory;

pub use self::factory::Factory;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Resources {}

impl gfx_core::Resources for Resources {
    type Buffer              = ();
    type Shader              = ();
    type Program             = ();
    type PipelineStateObject = ();
    type Texture             = ();
    type RenderTargetView    = ();
    type DepthStencilView    = ();
    type ShaderResourceView  = ();
    type UnorderedAccessView = ();
    type Sampler             = ();
    type Fence               = ();
}

pub struct Device {
    mtl_device: id
}
