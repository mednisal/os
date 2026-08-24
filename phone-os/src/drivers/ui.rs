//! UI primitives and widgets for the phone OS
//! 
//! This module provides basic UI components including buttons, labels,
//! panels, sliders, checkboxes, and event handling for touch interaction.

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

impl UIEvent {
    /// Convert from TouchEvent to UIEvent
    pub fn from_touch(event: TouchEvent) -> Self {
        match event {
            TouchEvent::Press(x, y) => UIEvent::TouchDown(x as usize, y as usize),
            TouchEvent::Release(x, y) => UIEvent::TouchUp(x as usize, y as usize),
            TouchEvent::Move(x, y) => UIEvent::TouchMove(x as usize, y as usize),
            TouchEvent::MultiTouch(_, x, y) => UIEvent::TouchMove(x as usize, y as usize),
            TouchEvent::None => UIEvent::None,
        }
    }

    /// Get coordinates from UIEvent
    pub fn get_coords(&self) -> Option<(usize, usize)> {
        match self {
            UIEvent::TouchDown(x, y) | UIEvent::TouchUp(x, y) | UIEvent::TouchMove(x, y) | UIEvent::Click(x, y) => {
                Some((*x, *y))
            }
            UIEvent::None => None,
        }
    }
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

    fn draw(&self, _fb: &mut FramebufferWriter, _font: &FontRenderer) {
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

/// Slider widget for selecting a value in a range
pub struct Slider {
    rect: Rect,
    min_value: i32,
    max_value: i32,
    current_value: i32,
    track_color: Color,
    thumb_color: Color,
    visible: bool,
    is_dragging: bool,
    on_change: Option<fn(i32)>,
}

impl Slider {
    pub const fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Slider {
            rect: Rect::new(x, y, width, height),
            min_value: 0,
            max_value: 100,
            current_value: 50,
            track_color: Color::GRAY,
            thumb_color: Color::BLUE,
            visible: true,
            is_dragging: false,
            on_change: None,
        }
    }

    pub fn set_range(&mut self, min: i32, max: i32) {
        self.min_value = min;
        self.max_value = max;
        if self.current_value < min {
            self.current_value = min;
        } else if self.current_value > max {
            self.current_value = max;
        }
    }

    pub fn set_value(&mut self, value: i32) {
        self.current_value = value.clamp(self.min_value, self.max_value);
    }

    pub fn get_value(&self) -> i32 {
        self.current_value
    }

    pub fn set_colors(&mut self, track: Color, thumb: Color) {
        self.track_color = track;
        self.thumb_color = thumb;
    }

    pub fn set_on_change(&mut self, callback: fn(i32)) {
        self.on_change = Some(callback);
    }

    /// Calculate thumb position based on current value
    fn thumb_position(&self) -> usize {
        let range = self.max_value - self.min_value;
        if range == 0 {
            return self.rect.x + self.rect.height / 2;
        }
        let normalized = (self.current_value - self.min_value) as f32 / range as f32;
        let thumb_width = self.rect.height;
        let available_width = self.rect.width - thumb_width;
        self.rect.x + (available_width as f32 * normalized) as usize
    }
}

impl Widget for Slider {
    fn bounds(&self) -> Rect {
        self.rect
    }

    fn draw(&self, _fb: &mut FramebufferWriter, _font: &FontRenderer) {
        if !self.visible {
            return;
        }

        unsafe {
            use core::ptr;
            let fb_addr = 0xE0000000u64;
            let width = 1080usize;
            let stride = width * 4;

            // Draw track (background line)
            let track_y = self.rect.y + self.rect.height / 2;
            let track_height = self.rect.height / 3;
            for dy in 0..track_height {
                for dx in 0..self.rect.width {
                    let x = self.rect.x + dx;
                    let y = track_y - track_height / 2 + dy;
                    if x < width && y < 1920 {
                        let fb_ptr = fb_addr as *mut u32;
                        let offset = y * stride / 4 + x;
                        ptr::write_volatile(fb_ptr.add(offset), self.track_color.as_u32());
                    }
                }
            }

            // Draw thumb (circle/rectangle at current position)
            let thumb_x = self.thumb_position();
            let thumb_size = self.rect.height;
            let thumb_left = thumb_x.saturating_sub(thumb_size / 2);
            
            for dy in 0..thumb_size {
                for dx in 0..thumb_size {
                    let x = thumb_left + dx;
                    let y = track_y - thumb_size / 2 + dy;
                    if x < width && y < 1920 {
                        // Check if point is within circle
                        let cx = thumb_x;
                        let cy = track_y;
                        let dist_sq = ((x as i32 - cx as i32).pow(2) + (y as i32 - cy as i32).pow(2)) as usize;
                        if dist_sq <= (thumb_size / 2).pow(2) {
                            let fb_ptr = fb_addr as *mut u32;
                            let offset = y * stride / 4 + x;
                            ptr::write_volatile(fb_ptr.add(offset), self.thumb_color.as_u32());
                        }
                    }
                }
            }
        }
    }

    fn handle_touch(&mut self, event: TouchEvent) -> bool {
        match event {
            TouchEvent::Press(x, y) => {
                let thumb_x = self.thumb_position();
                let thumb_rect = Rect::new(
                    thumb_x.saturating_sub(self.rect.height / 2),
                    self.rect.y,
                    self.rect.height,
                    self.rect.height,
                );
                if thumb_rect.contains(x as usize, y as usize) {
                    self.is_dragging = true;
                    return true;
                }
                // Also allow clicking on track to jump to position
                if self.rect.contains(x as usize, y as usize) {
                    self.update_value_from_x(x as usize);
                    return true;
                }
            }
            TouchEvent::Move(x, _y) | TouchEvent::MultiTouch(_, x, _y) => {
                if self.is_dragging && self.rect.contains(x as usize, self.rect.y + self.rect.height / 2) {
                    self.update_value_from_x(x as usize);
                    return true;
                }
            }
            TouchEvent::Release(_x, _y) => {
                if self.is_dragging {
                    self.is_dragging = false;
                    return true;
                }
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

impl Slider {
    fn update_value_from_x(&mut self, x: usize) {
        let thumb_width = self.rect.height;
        let available_start = self.rect.x + thumb_width / 2;
        let available_end = self.rect.x + self.rect.width - thumb_width / 2;
        
        let clamped_x = x.clamp(available_start, available_end);
        let range = self.max_value - self.min_value;
        let available_width = available_end - available_start;
        
        if available_width > 0 {
            let normalized = (clamped_x - available_start) as f32 / available_width as f32;
            let new_value = self.min_value + (normalized * range as f32) as i32;
            let old_value = self.current_value;
            self.current_value = new_value.clamp(self.min_value, self.max_value);
            
            if self.current_value != old_value {
                if let Some(callback) = self.on_change {
                    callback(self.current_value);
                }
            }
        }
    }
}

/// Checkbox widget for boolean selection
pub struct Checkbox {
    rect: Rect,
    label: &'static str,
    checked: bool,
    check_color: Color,
    bg_color: Color,
    label_color: Color,
    visible: bool,
    on_toggle: Option<fn(bool)>,
}

impl Checkbox {
    pub const fn new(label: &'static str, x: usize, y: usize, size: usize) -> Self {
        Checkbox {
            rect: Rect::new(x, y, size, size),
            label,
            checked: false,
            check_color: Color::WHITE,
            bg_color: Color::DARK_GRAY,
            label_color: Color::WHITE,
            visible: true,
            on_toggle: None,
        }
    }

    pub fn set_checked(&mut self, checked: bool) {
        self.checked = checked;
    }

    pub fn is_checked(&self) -> bool {
        self.checked
    }

    pub fn set_colors(&mut self, check: Color, bg: Color, label: Color) {
        self.check_color = check;
        self.bg_color = bg;
        self.label_color = label;
    }

    pub fn set_on_toggle(&mut self, callback: Option<fn(bool)>) {
        self.on_toggle = callback;
    }

    /// Get the full bounding box including label
    pub fn full_bounds(&self, font: &FontRenderer) -> Rect {
        let label_width = font.measure_string(self.label);
        let total_width = self.rect.width + label_width + 8; // 8px gap
        Rect::new(self.rect.x, self.rect.y, total_width, self.rect.height.max(font.char_height()))
    }
}

impl Widget for Checkbox {
    fn bounds(&self) -> Rect {
        self.rect
    }

    fn draw(&self, fb: &mut FramebufferWriter, font: &FontRenderer) {
        if !self.visible {
            return;
        }

        unsafe {
            use core::ptr;
            let fb_addr = 0xE0000000u64;
            let width = 1080usize;
            let stride = width * 4;

            // Draw checkbox background
            for dy in 0..self.rect.height {
                for dx in 0..self.rect.width {
                    let x = self.rect.x + dx;
                    let y = self.rect.y + dy;
                    if x < width && y < 1920 {
                        let fb_ptr = fb_addr as *mut u32;
                        let offset = y * stride / 4 + x;
                        
                        // Draw border
                        let is_border = dx == 0 || dx == self.rect.width - 1 || 
                                       dy == 0 || dy == self.rect.height - 1;
                        let color = if is_border { self.label_color } else { self.bg_color };
                        ptr::write_volatile(fb_ptr.add(offset), color.as_u32());
                    }
                }
            }

            // Draw checkmark if checked
            if self.checked {
                // Simple checkmark pattern
                let margin = 2;
                let check_size = self.rect.height - 2 * margin;
                for dy in 0..check_size {
                    for dx in 0..check_size {
                        // Draw a simple check pattern
                        let in_check = (dx >= dy / 2 && dx <= check_size - dy / 3) ||
                                      (dy >= check_size / 3 && dx >= dy / 3 && dx <= check_size * 2 / 3);
                        if in_check {
                            let x = self.rect.x + margin + dx;
                            let y = self.rect.y + margin + dy;
                            if x < width && y < 1920 {
                                let fb_ptr = fb_addr as *mut u32;
                                let offset = y * stride / 4 + x;
                                ptr::write_volatile(fb_ptr.add(offset), self.check_color.as_u32());
                            }
                        }
                    }
                }
            }

            // Draw label
            if !self.label.is_empty() {
                let label_x = self.rect.x + self.rect.width + 8;
                let label_y = self.rect.y + (self.rect.height - font.char_height()) / 2;
                font.draw_string(fb, self.label, label_x, label_y, self.label_color, Color::BLACK);
            }
        }
    }

    fn handle_touch(&mut self, event: TouchEvent) -> bool {
        if let TouchEvent::Release(x, y) = event {
            if self.rect.contains(x as usize, y as usize) {
                self.checked = !self.checked;
                if let Some(callback) = self.on_toggle {
                    callback(self.checked);
                }
                return true;
            }
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

/// Progress bar widget for showing progress
pub struct ProgressBar {
    rect: Rect,
    min_value: i32,
    max_value: i32,
    current_value: i32,
    bg_color: Color,
    fill_color: Color,
    border_color: Color,
    has_border: bool,
    visible: bool,
}

impl ProgressBar {
    pub const fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        ProgressBar {
            rect: Rect::new(x, y, width, height),
            min_value: 0,
            max_value: 100,
            current_value: 0,
            bg_color: Color::DARK_GRAY,
            fill_color: Color::GREEN,
            border_color: Color::GRAY,
            has_border: true,
            visible: true,
        }
    }

    pub fn set_range(&mut self, min: i32, max: i32) {
        self.min_value = min;
        self.max_value = max;
        if self.current_value < min {
            self.current_value = min;
        } else if self.current_value > max {
            self.current_value = max;
        }
    }

    pub fn set_value(&mut self, value: i32) {
        self.current_value = value.clamp(self.min_value, self.max_value);
    }

    pub fn get_value(&self) -> i32 {
        self.current_value
    }

    pub fn get_percentage(&self) -> f32 {
        let range = self.max_value - self.min_value;
        if range == 0 {
            return 0.0;
        }
        (self.current_value - self.min_value) as f32 / range as f32
    }

    pub fn set_colors(&mut self, bg: Color, fill: Color, border: Color) {
        self.bg_color = bg;
        self.fill_color = fill;
        self.border_color = border;
    }

    pub fn set_has_border(&mut self, has_border: bool) {
        self.has_border = has_border;
    }
}

impl Widget for ProgressBar {
    fn bounds(&self) -> Rect {
        self.rect
    }

    fn draw(&self, _fb: &mut FramebufferWriter, _font: &FontRenderer) {
        if !self.visible {
            return;
        }

        unsafe {
            use core::ptr;
            let fb_addr = 0xE0000000u64;
            let width = 1080usize;
            let stride = width * 4;

            // Draw background
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

            // Draw filled portion
            let fill_width = ((self.rect.width as f32) * self.get_percentage()) as usize;
            for dy in 0..self.rect.height {
                for dx in 0..fill_width {
                    let x = self.rect.x + dx;
                    let y = self.rect.y + dy;
                    if x < width && y < 1920 {
                        let fb_ptr = fb_addr as *mut u32;
                        let offset = y * stride / 4 + x;
                        ptr::write_volatile(fb_ptr.add(offset), self.fill_color.as_u32());
                    }
                }
            }

            // Draw border if enabled
            if self.has_border {
                for dy in 0..self.rect.height {
                    for dx in 0..self.rect.width {
                        let x = self.rect.x + dx;
                        let y = self.rect.y + dy;
                        if x < width && y < 1920 {
                            let is_border = dx == 0 || dx == self.rect.width - 1 || 
                                           dy == 0 || dy == self.rect.height - 1;
                            if is_border {
                                let fb_ptr = fb_addr as *mut u32;
                                let offset = y * stride / 4 + x;
                                ptr::write_volatile(fb_ptr.add(offset), self.border_color.as_u32());
                            }
                        }
                    }
                }
            }
        }
    }

    fn handle_touch(&mut self, _event: TouchEvent) -> bool {
        false // Progress bars don't handle touch
    }

    fn is_visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}
