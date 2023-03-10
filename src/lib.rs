/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! Rasterization for linear, quadric and cubic Bezier curves that was designed
//! for font rasterizarion.
//!
//! ```
//! use font_rasterizer::*;
//!
//! // Add outline curves to CanvasBuilder
//! let canvas_builder = CanvasBuilder::new(width, height)
//!     .add_curve(Linear(l0, l1))
//!     .add_curve(Quadric(q0, q1, q2))
//!     .add_curve(Cubic(c0, c1, c2, c3));
//!
//! // Draw outlines and fill them
//! let canvas = canvas_builder.build();
//!
//! // Iterate over resultant pixel alphas
//! canvas.iter()
//!     .for_each(|alpha| {
//!         // ...
//!     })
//! ```

#![cfg_attr(feature = "no_std", no_std)]

#[cfg(feature = "no_std")]
extern crate alloc;

pub mod canvas;
pub mod math;

pub use canvas::*;
pub use math::*;
