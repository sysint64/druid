// Copyright 2020 The xi-editor Authors.
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

//! A type for representing text that is displayed on the screen.

use super::TextBuffer;
use crate::piet::{
    FontBuilder as _, PietText, PietTextLayout, Text as _, TextLayout as _, TextLayoutBuilder as _,
};
use crate::{theme, Env, PaintCtx, Point, RenderContext, Size};

pub struct TextLayout {
    buffer: TextBuffer,
    // this is optional so that you can create a `TextLayout` before you get passed contexts etc
    layout: Option<PietTextLayout>,
    /// The width for the purpose of line breaks; that is, the width of the view,
    /// not necessarily the width of the current text.
    width: f64,
}

impl TextLayout {
    pub fn new(
        buffer: TextBuffer,
        text: &mut PietText,
        env: &Env,
        width: impl Into<Option<f64>>,
    ) -> Self {
        let width = width.into().unwrap_or(f64::INFINITY);
        let layout = layout_for_buffer(&buffer, text, env, width);
        TextLayout {
            buffer,
            layout,
            width,
        }
    }

    pub fn update_buffer(&mut self, buffer: TextBuffer, text: &mut PietText, env: &Env) {
        self.layout = layout_for_buffer(&buffer, text, env, self.width);
        self.buffer = buffer;
    }

    pub fn update_width(&mut self, width: impl Into<Option<f64>>) {
        self.width = width.into().unwrap_or(f64::INFINITY);
        if let Some(layout) = &mut self.layout {
            layout.update_width(self.width).unwrap();
        }
    }

    pub fn draw(&self, ctx: &mut PaintCtx, point: impl Into<Point>, env: &Env) {
        if let Some(layout) = &self.layout {
            let color = env.get(theme::LABEL_COLOR);
            eprintln!("drawing text");
            ctx.draw_text(layout, point, &color);
        }
    }

    /// The size of this layout, in pixels.
    pub fn size(&self) -> Size {
        if let Some(layout) = &self.layout {
            let height = layout
                .line_metric(layout.line_count())
                .map(|metric| metric.cumulative_height)
                .unwrap_or(0.);
            Size::new(layout.width(), height)
        } else {
            Size::ZERO
        }
    }
}

fn layout_for_buffer(
    buffer: &TextBuffer,
    text: &mut PietText,
    env: &Env,
    width: f64,
) -> Option<PietTextLayout> {
    //FIXME: figure out how to resolve these from `TextBuffer`
    let font_name = env.get(theme::FONT_NAME);
    let font_size = env.get(theme::TEXT_SIZE_NORMAL);
    let font = text.new_font_by_name(font_name, font_size).build().ok()?;
    text.new_text_layout(&font, buffer.slice(..).as_ref(), width)
        .build()
        .ok()
}

impl Default for TextLayout {
    fn default() -> Self {
        TextLayout {
            buffer: TextBuffer::default(),
            width: f64::INFINITY,
            layout: None,
        }
    }
}
