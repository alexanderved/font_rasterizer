/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use font_rasterizer::*;
use owned_ttf_parser as ttfp;

pub struct Outliner {
    pub last: Point,
    pub last_move: Option<Point>,
    pub cb: CanvasBuilder,
}

impl Outliner {
    pub fn new() -> Self {
        Self {
            last: Point::default(),
            last_move: None,
            cb: CanvasBuilder::new()
        }
    }
}

impl ttfp::OutlineBuilder for Outliner {
    fn move_to(&mut self, x: f32, y: f32) {
        // eprintln!("M {x} {y}");
        self.last = point(x, y);
        self.last_move = Some(self.last);
    }

    fn line_to(&mut self, x1: f32, y1: f32) {
        // eprintln!("L {x1} {y1}");
        let p1 = point(x1, y1);
        self.cb.curve(Curve::linear(self.last, p1));
        self.last = p1;
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        // eprintln!("Q {x1} {y1}");
        let p1 = point(x1, y1);
        let p2 = point(x2, y2);
        self.cb.curve(Curve::quadric(self.last, p1, p2));
        self.last = p2;
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        // eprintln!("C {x1} {y1} {x3} {y3}");
        let p1 = point(x1, y1);
        let p2 = point(x2, y2);
        let p3 = point(x3, y3);

        self.cb.curve(Curve::cubic(self.last, p1, p2, p3));
        self.last = p3;
    }

    fn close(&mut self) {
        // eprintln!("Z");
        if let Some(m) = self.last_move.take() {
            self.cb.curve(Curve::linear(self.last, m));
        }
    }
}
