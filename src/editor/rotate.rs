use crate::graphics::Color;
use crate::graphics::Fill;
use crate::graphics::Tool;
use crate::editor::MenuRow;
use crate::editor::EditorMode;

pub trait Rotate {
    fn next(self) -> Self;
    fn prev(self) -> Self;

    fn add(self, addend: i16) -> Self
    where
        Self: Sized,
    {
        if addend > 0 {
            self.next()
        } else if addend < 0 {
            self.prev()
        } else {
            self
        }
    }
}

impl Rotate for Tool {
    fn next(self) -> Self {
        const MAP: [Tool; 6] = [
            Tool::Line,
            Tool::Rect,
            Tool::Ellipse,
            Tool::FloodFill,
            Tool::Clear,
            Tool::Pixel,
        ];
        MAP[self as usize]
    }

    fn prev(self) -> Self {
        const MAP: [Tool; 6] = [
            Tool::Clear,
            Tool::Pixel,
            Tool::Line,
            Tool::Rect,
            Tool::Ellipse,
            Tool::FloodFill,
        ];
        MAP[self as usize]
    }
}

impl Rotate for Color {
    fn next(self) -> Self {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }

    fn prev(self) -> Self {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

impl Rotate for Fill {
    fn next(self) -> Self {
        match self {
            Fill::Off => Fill::On,
            Fill::On => Fill::Off,
        }
    }

    fn prev(self) -> Self {
        match self {
            Fill::Off => Fill::On,
            Fill::On => Fill::Off,
        }
    }
}

impl Rotate for MenuRow {
    fn next(self) -> Self {
        match self {
            MenuRow::Tool => MenuRow::Color,
            MenuRow::Color => MenuRow::Fill,
            MenuRow::Fill => MenuRow::Tool,
        }
    }

    fn prev(self) -> Self {
        match self {
            MenuRow::Tool => MenuRow::Fill,
            MenuRow::Color => MenuRow::Tool,
            MenuRow::Fill => MenuRow::Color,
        }
    }
}

impl Rotate for EditorMode {
    fn next(self) -> Self {
        match self {
            EditorMode::Normal => EditorMode::Menu,
            EditorMode::Menu => EditorMode::Normal,
        }
    }

    fn prev(self) -> Self {
        match self {
            EditorMode::Normal => EditorMode::Menu,
            EditorMode::Menu => EditorMode::Normal,
        }
    }
}
