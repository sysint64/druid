// Copyright 2020 The druid Authors.
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

//! A focus widget.

use druid::widget::prelude::*;

use druid::{
    commands, Data, FocusNode, HotKey, KbKey, Point, Rect, SysMods, Widget, WidgetPod,
};

/// A widget that allow focus to be given to this widget and its descendants.
pub struct Focus<T> {
    child: WidgetPod<T, Box<dyn Widget<T>>>,
    auto_focus: bool,
    focus_requested: bool,
    focus_node: FocusNode,
}

impl<T: Data> Focus<T> {
    /// Create a new Focus widget with a child
    pub fn new(child: impl Widget<T> + 'static) -> Self {
        Focus {
            child: WidgetPod::new(child).boxed(),
            focus_node: FocusNode::empty(),
            auto_focus: false,
            focus_requested: false,
        }
    }

    /// Builder-style method to set the `Focus`'s auto focus.
    /// Has focus when the widget is created.
    /// When multiple widgets are auto-focused, the last created widget will gain the focus.
    pub fn with_auto_focus(mut self, auto_focus: bool) -> Self {
        self.auto_focus = auto_focus;
        self
    }
}

impl<T: Data> Widget<T> for Focus<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        let previous_focus_node = ctx.focus_node();
        ctx.set_focus_node(self.focus_node);

        if self.auto_focus && !self.focus_requested {
            self.focus_requested = true;
            ctx.request_focus();
        }

        self.child.event(ctx, event, data, env);

        match event {
            Event::MouseDown(_) => {
                ctx.request_focus();
                ctx.request_paint();
            }
            Event::KeyDown(key_event) if !ctx.is_handled => {
                match key_event {
                    // Tab and shift+tab
                    k_e if HotKey::new(None, KbKey::Tab).matches(k_e) => {
                        ctx.focus_next();
                    }
                    k_e if HotKey::new(SysMods::Shift, KbKey::Tab).matches(k_e) => {
                        ctx.focus_prev();
                    }
                    _ => (),
                };

                ctx.request_paint();
            }
            Event::Command(cmd) if cmd.is(commands::REQUEST_FOCUS) => {
                let widget_id = *cmd.get_unchecked(commands::REQUEST_FOCUS);

                if widget_id == ctx.widget_id() {
                    ctx.request_focus();
                    ctx.request_paint();
                }
            }
            _ => (),
        }

        ctx.set_focus_node(previous_focus_node);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        let previous_focus_node = ctx.focus_node();

        match event {
            LifeCycle::WidgetAdded => {
                self.focus_node.widget_id = Some(ctx.widget_id());
                ctx.set_focus_node(self.focus_node);
                ctx.register_for_focus();
            }
            LifeCycle::FocusChanged(value) => {
                self.focus_node.is_focused = *value;

                ctx.submit_command(
                    commands::FOCUS_NODE_FOCUS_CHANGED
                        .with(*value)
                        .to(ctx.widget_id()),
                );
                ctx.request_paint();
            }
            _ => (),
        }

        ctx.set_focus_node(self.focus_node);
        self.child.lifecycle(ctx, event, data, env);
        ctx.set_focus_node(previous_focus_node);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        let previous_focus_node = ctx.focus_node();
        ctx.set_focus_node(self.focus_node);
        self.child.update(ctx, data, env);
        ctx.set_focus_node(previous_focus_node);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let previous_focus_node = ctx.focus_node();
        ctx.set_focus_node(self.focus_node);
        let size = self.child.layout(ctx, &bc, data, env);
        let rect = Rect::from_origin_size(Point::ORIGIN, size);
        self.child.set_layout_rect(ctx, data, env, rect);
        ctx.set_focus_node(previous_focus_node);

        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let previous_focus_node = ctx.focus_node();
        ctx.set_focus_node(self.focus_node);
        self.child.paint(ctx, data, env);
        ctx.set_focus_node(previous_focus_node);
    }
}
