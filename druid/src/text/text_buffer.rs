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

//! A type for representing editable, selectable text buffers.

use std::borrow::Cow;

use xi_rope::interval::{Interval, IntervalBounds};
use xi_rope::spans::{Spans, SpansBuilder};
use xi_rope::{Rope, RopeDelta};

use crate::{Color, Data};

#[derive(Debug, Clone)]
pub struct Style {
    font: Option<()>,
    color: Color,
}

#[derive(Debug, Clone, Default)]
pub struct TextBuffer {
    buffer: Rope,
    styles: Spans<Style>,
}

impl TextBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    /// The length of the buffer, in utf8 code units.
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Return a slice of the text.
    ///
    /// This borrows from the underlying text when possible.
    pub fn slice(&self, range: impl IntervalBounds) -> Cow<str> {
        self.buffer.slice_to_cow(range)
    }

    /// Replace a slice of the buffer.
    ///
    /// The new region will have empty styles. (change this?)
    ///
    /// # Panics
    ///
    /// This will panic if the range extends beyond the bounds of the buffer,
    /// or if the range falls between a codepoint boundary.
    pub fn replace(&mut self, range: impl IntervalBounds, new: impl Into<Rope>) {
        let new = new.into();
        let new_styles = SpansBuilder::new(new.len()).build();
        let interval = range.into_interval(self.buffer.len());
        self.buffer.edit(interval, new);
        self.styles.edit(interval, new_styles);
    }

    /// Append new text onto the end of the buffer.
    pub fn push(&mut self, new: impl Into<Rope>) {
        let iv = Interval::new(self.buffer.len(), self.buffer.len());
        self.replace(iv, new)
    }
}

impl Data for TextBuffer {
    fn same(&self, other: &TextBuffer) -> bool {
        self.buffer.ptr_eq(&other.buffer) && self.styles.ptr_eq(&other.styles)
    }
}

impl<T: AsRef<str>> From<T> for TextBuffer {
    fn from(src: T) -> TextBuffer {
        let buffer = Rope::from(src.as_ref());
        let styles = SpansBuilder::new(buffer.len()).build();
        TextBuffer { buffer, styles }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_test() {
        let mut buffer = TextBuffer::from("hello friends");
        buffer.replace(..5, "goodbye");
        buffer.push(" I'm sick of");
        assert_eq!(buffer.slice(..).as_ref(), "goodbye friends I'm sick of");
    }
}
