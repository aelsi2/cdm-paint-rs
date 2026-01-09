mod rotate;

use rotate::Rotate;

use crate::graphics::Color;
use crate::graphics::Fill;
use crate::graphics::Point;
use crate::graphics::SCREEN_HEIGHT;
use crate::graphics::SCREEN_WIDTH;
use crate::graphics::Tool;
use crate::shapes;
use crate::shapes::Shape;
use alloc::boxed::Box;
use alloc::collections::VecDeque;

#[derive(Clone, Copy, Default, Hash, Eq, PartialEq)]
#[repr(u8)]
pub enum EditorMode {
    #[default]
    Normal = 0,
    Menu = 1,
}

#[derive(Clone, Copy, Default, Hash, Eq, PartialEq)]
#[repr(u8)]
pub enum MenuRow {
    #[default]
    Tool = 0,
    Color = 1,
    Fill = 2,
}

pub struct Editor {
    pub mode: EditorMode,
    pub tool: Tool,
    pub color: Color,
    pub fill: Fill,
    pub cur1: Point,
    pub cur2: Option<Point>,
    pub cur_menu: MenuRow,
}

impl Editor {
    pub const fn new() -> Self {
        Editor {
            mode: EditorMode::Normal,
            tool: Tool::Line,
            color: Color::White,
            fill: Fill::On,
            cur1: Point::new(15, 15),
            cur2: None,
            cur_menu: MenuRow::Tool,
        }
    }

    pub fn move_cursor(&mut self, vec: (i16, i16)) {
        let (x, y) = vec;
        if self.mode == EditorMode::Normal {
            let (cx, cy) = self.cur1.into();
            let cx = (cx + x) as usize % SCREEN_WIDTH;
            let cy = (cy + y) as usize % SCREEN_HEIGHT;
            self.cur1 = Point::new(cx as i16, cy as i16)
        } else {
            self.cur_menu = self.cur_menu.add(y);
            match self.cur_menu {
                MenuRow::Tool => {
                    self.tool = self.tool.add(x);
                }
                MenuRow::Color => {
                    self.color = self.color.add(x);
                }
                MenuRow::Fill => {
                    self.fill = self.fill.add(x);
                }
            }
        }
    }

    pub fn toggle_mode(&mut self) {
        self.cur2 = None;
        self.mode = self.mode.next();
    }

    pub fn needs_cur2(&self) -> bool {
        match self.tool {
            Tool::FloodFill | Tool::Pixel | Tool::Clear => false,
            _ => self.cur2 == None,
        }
    }

    pub fn set_cur2(&mut self) {
        self.cur2 = Some(self.cur1);
    }

    pub fn reset_cur2(&mut self) {
        self.cur2 = None;
    }

    pub fn enqueue(&mut self, queue: &mut VecDeque<Box<dyn Shape>>) {
        let tool = self.tool;
        let pt1 = self.cur1;
        let pt2 = match self.cur2 {
            Some(pt) => pt,
            None => Point::ZERO,
        };
        let color = self.color;
        let fill = self.fill;
        self.cur2 = None;
        let shape: Box<dyn Shape> = match tool {
            Tool::Clear => Box::new(shapes::Clear::new(color)),
            Tool::Line => Box::new(shapes::Line::new(pt1, pt2, color)),
            Tool::Pixel => Box::new(shapes::Pixel::new(pt1, color)),
            Tool::FloodFill => Box::new(shapes::FloodFill::new(pt1, color)),
            Tool::Rect => Box::new(shapes::Rect::new(pt1, pt2, color, fill)),
            Tool::Ellipse => Box::new(shapes::Ellipse::new(pt1, pt2, color, fill)),
        };
        queue.push_back(shape);
    }
}
