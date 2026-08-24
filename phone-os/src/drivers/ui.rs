//! UI primitives and widgets for the phone OS
//! 
//! This module provides basic UI components including buttons, labels,
//! panels, and event handling for touch interaction.

use crate::drivers::framebuffer::{Color, FramebufferWriter};
use crate::drivers::font::{FontRenderer, TextAlign};
use crate::drivers::touch::TouchEvent;

/// UI Event types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UIEvent {
    None,
    TouchDown(usize, usize),  // x, y in screen coordinates
    TouchUp(usize, usize),
    TouchMove(usize, usize),
    Click(usize, usize),
}

/// Rectangle for bounding boxes
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Rect {
    pub const fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Rect { x, y, width, height }
    }

    /// Check if a point is inside this rectangle
    pub fn contains(&self, x: usize, y: usize) -> bool {
        x >= self.x && x < self.x + self.width &&
        y >= self.y && y < self.y + self.height
    }

    /// Get the center point of the rectangle
    pub fn center(&self) -> (usize, usize) {
        (self.x + self.width / 2, self.y + self.height / 2)
    }
}

/// Base widget trait
pub trait Widget {
    /// Get the bounding rectangle of the widget
    fn bounds(&self) -> Rect;
    
    /// Draw the widget on the framebuffer
    fn draw(&self, fb: &mut FramebufferWriter, font: &FontRenderer);
    
    /// Handle a touch event, returns true if event was consumed
    fn handle_touch(&mut self, event: TouchEvent) -> bool;
    
    /// Check if widget is visible
    fn is_visible(&self) -> bool;
    
    /// Set visibility
    fn set_visible(&mut self, visible: bool);
}

/// Label widget for displaying text
pub struct Label {
    rect: Rect,
    text: &'static str,
    fg_color: Color,
    bg_color: Color,
    align: TextAlign,
    visible: bool,
}

impl Label {
    pub const fn new(text: &'static str, x: usize, y: usize, width: usize, height: usize) -> Self {
        Label {
            rect: Rect::new(x, y, width, height),
            text,
            fg_color: Color::WHITE,
            bg_color: Color::BLACK,
            align: TextAlign::Left,
            visible: true,
        }
    }

    pub fn set_text(&mut self, text: &'static str) {
        self.text = text;
    }

    pub fn set_colors(&mut self, fg: Color, bg: Color) {
        self.fg_color = fg;
        self.bg_color = bg;
    }

    pub fn set_align(&mut self, align: TextAlign) {
        self.align = align;
    }
}

impl Widget for Label {
    fn bounds(&self) -> Rect {
        self.rect
    }

    fn draw(&self, fb: &mut FramebufferWriter, font: &FontRenderer) {
        if !self.visible {
            return;
        }

        // Clear background
        unsafe {
            use core::ptr;
            let fb_addr = 0xE0000000u64;
            let width = 1080usize;
            let stride = width * 4;
            
            for dy in 0..self.rect.height {
                for dx in 0..self.rect.width {
                    let x = self.rect.x + dx;
                    let y = self.rect.y + dy;
                    if x < width && y < 1920 {
                        let fb_ptr = fb_addr as *mut u32;
                        let offset = y * stride / 4 + x;
                        ptr::write_volatile(fb_ptr.add(offset), self.bg_color.as_u32());
                    }
                }
            }
        }

        // Calculate text position based on alignment
        let text_width = font.measure_string(self.text);
        let (text_x, text_y) = match self.align {
            TextAlign::Left => (self.rect.x, self.rect.y + (self.rect.height - font.char_height()) / 2),
            TextAlign::Center => (self.rect.x + (self.rect.width - text_width) / 2, self.rect.y + (self.rect.height - font.char_height()) / 2),
            TextAlign::Right => (self.rect.x + self.rect.width - text_width, self.rect.y + (self.rect.height - font.char_height()) / 2),
        };

        font.draw_string(fb, self.text, text_x, text_y, self.fg_color, self.bg_color);
    }

    fn handle_touch(&mut self, _event: TouchEvent) -> bool {
        false // Labels don't handle touch
    }

    fn is_visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

/// Button widget
pub struct Button {
    rect: Rect,
    text: &'static str,
    normal_bg: Color,
    pressed_bg: Color,
    fg_color: Color,
    visible: bool,
    is_pressed: bool,
    on_click: Option<fn()>,
}

impl Button {
    pub const fn new(text: &'static str, x: usize, y: usize, width: usize, height: usize) -> Self {
        Button {
            rect: Rect::new(x, y, width, height),
            text,
            normal_bg: Color::BLUE,
            pressed_bg: Color::DARK_GRAY,
            fg_color: Color::WHITE,
            visible: true,
            is_pressed: false,
            on_click: None,
        }
    }

    pub fn set_on_click(&mut self, callback: extern "C" fn()) {
        self.on_click = Some(unsafe { core::mem::transmute(callback) });
    }

    pub fn set_colors(&mut self, normal: Color, pressed: Color, fg: Color) {
        self.normal_bg = normal;
        self.pressed_bg = pressed;
        self.fg_color = fg;
    }

    pub fn is_pressed(&self) -> bool {
        self.is_pressed
    }
}

impl Widget for Button {
    fn bounds(&self) -> Rect {
        self.rect
    }

    fn draw(&self, fb: &mut FramebufferWriter, font: &FontRenderer) {
        if !self.visible {
            return;
        }

        let bg = if self.is_pressed { self.pressed_bg } else { self.normal_bg };

        // Draw button background
        unsafe {
            use core::ptr;
            let fb_addr = 0xE0000000u64;
            let width = 1080usize;
            let stride = width * 4;
            
            for dy in 0..self.rect.height {
                for dx in 0..self.rect.width {
                    let x = self.rect.x + dx;
                    let y = self.rect.y + dy;
                    if x < width && y < 1920 {
                        let fb_ptr = fb_addr as *mut u32;
                        let offset = y * stride / 4 + x;
                        
                        // Add simple border effect
                        let is_border = dx == 0 || dx == self.rect.width - 1 || 
                                       dy == 0 || dy == self.rect.height - 1;
                        let color = if is_border { Color::GRAY } else { bg };
                        ptr::write_volatile(fb_ptr.add(offset), color.as_u32());
                    }
                }
            }
        }

        // Draw button text centered
        let text_width = font.measure_string(self.text);
        let text_x = self.rect.x + (self.rect.width - text_width) / 2;
        let text_y = self.rect.y + (self.rect.height - font.char_height()) / 2;
        font.draw_string(fb, self.text, text_x, text_y, self.fg_color, bg);
    }

    fn handle_touch(&mut self, event: TouchEvent) -> bool {
        match event {
            TouchEvent::Press(x, y) | TouchEvent::Move(x, y) | TouchEvent::MultiTouch(_, x, y) => {
                if self.rect.contains(x as usize, y as usize) {
                    self.is_pressed = true;
                    return true;
                }
            }
            TouchEvent::Release(x, y) => {
                if self.is_pressed && self.rect.contains(x as usize, y as usize) {
                    self.is_pressed = false;
                    if let Some(callback) = self.on_click {
                        callback();
                    }
                    return true;
                }
                self.is_pressed = false;
            }
            _ => {}
        }
        false
    }

    fn is_visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

/// Panel widget for grouping other widgets
pub struct Panel {
    rect: Rect,
    bg_color: Color,
    border_color: Color,
    has_border: bool,
    visible: bool,
}

impl Panel {
    pub const fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Panel {
            rect: Rect::new(x, y, width, height),
            bg_color: Color::BLACK,
            border_color: Color::GRAY,
            has_border: true,
            visible: true,
        }
    }

    pub fn set_colors(&mut self, bg: Color, border: Color) {
        self.bg_color = bg;
        self.border_color = border;
    }

    pub fn set_has_border(&mut self, has_border: bool) {
        self.has_border = has_border;
    }
}

impl Widget for Panel {
    fn bounds(&self) -> Rect {
        self.rect
    }

    fn draw(&self, fb: &mut FramebufferWriter, _font: &FontRenderer) {
        if !self.visible {
            return;
        }

        unsafe {
            use core::ptr;
            let fb_addr = 0xE0000000u64;
            let width = 1080usize;
            let stride = width * 4;
            
            for dy in 0..self.rect.height {
                for dx in 0..self.rect.width {
                    let x = self.rect.x + dx;
                    let y = self.rect.y + dy;
                    if x < width && y < 1920 {
                        let fb_ptr = fb_addr as *mut u32;
                        let offset = y * stride / 4 + x;
                        
                        // Draw border if enabled
                        let is_border = self.has_border && (dx == 0 || dx == self.rect.width - 1 || 
                                                          dy == 0 || dy == self.rect.height - 1);
                        let color = if is_border { self.border_color } else { self.bg_color };
                        ptr::write_volatile(fb_ptr.add(offset), color.as_u32());
                    }
                }
            }
        }
    }

    fn handle_touch(&mut self, _event: TouchEvent) -> bool {
        false // Panels don't handle touch directly
    }

    fn is_visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

/// Screen manager to hold and manage all widgets
pub struct Screen {
    widget_count: usize,
    font: FontRenderer,
    bg_color: Color,
}

// Static storage for widgets (simplified approach for no_std)
static mut SCREEN_WIDGETS: [Option<&'static mut dyn Widget>; 32] = [const { None }; 32];

impl Screen {
    pub const fn new() -> Self {
        Screen {
            widget_count: 0,
            font: FontRenderer::new(),
            bg_color: Color::BLACK,
        }
    }

    pub fn add_widget(&mut self, widget: &'static mut dyn Widget) -> bool {
        unsafe {
            if self.widget_count < 32 {
                SCREEN_WIDGETS[self.widget_count] = Some(widget);
                self.widget_count += 1;
                true
            } else {
                false
            }
        }
    }

    pub fn clear(&mut self, _fb: &mut FramebufferWriter) {
        // Fill screen with background color
        unsafe {
            use core::ptr;
            let fb_addr = 0xE0000000u64;
            let width = 1080usize;
            let height = 1920usize;
            let stride = width * 4;
            
            for y in 0..height {
                for x in 0..width {
                    let fb_ptr = fb_addr as *mut u32;
                    let offset = y * stride / 4 + x;
                    ptr::write_volatile(fb_ptr.add(offset), self.bg_color.as_u32());
                }
            }
        }
    }

    pub fn draw_all(&mut self, fb: &mut FramebufferWriter) {
        unsafe {
            for i in 0..self.widget_count {
                if let Some(ref widget) = SCREEN_WIDGETS[i] {
                    widget.draw(fb, &self.font);
                }
            }
        }
    }

    pub fn handle_touch(&mut self, event: TouchEvent) {
        // Process widgets in reverse order (top-most first)
        unsafe {
            for i in (0..self.widget_count).rev() {
                if let Some(ref mut widget) = SCREEN_WIDGETS[i] {
                    if widget.handle_touch(event) {
                        break; // Event consumed
                    }
                }
            }
        }
    }

    pub fn set_background(&mut self, color: Color) {
        self.bg_color = color;
    }
}

impl Default for Screen {
    fn default() -> Self {
        Self::new()
    }
}
