//! UI Demo for Phone OS
//! 
//! This module demonstrates the UI system with a simple home screen
//! containing buttons, labels, and panels.

use crate::drivers::framebuffer::{Color, FramebufferWriter};
use crate::drivers::font::{FontRenderer, TextAlign};
use crate::drivers::touch::{TouchEvent, poll_touch};
use crate::drivers::ui::{Screen, Button, Label, Panel, Widget, Rect};

// Static widget storage for demo
static mut DEMO_SCREEN: Option<Screen> = None;
static mut HOME_LABEL: Option<Label> = None;
static mut BUTTON1: Option<Button> = None;
static mut BUTTON2: Option<Button> = None;
static mut BUTTON3: Option<Button> = None;
static mut TOP_PANEL: Option<Panel> = None;
static mut BOTTOM_PANEL: Option<Panel> = None;

static mut CLICK_COUNT: usize = 0;
static mut BUTTON1_PRESSED: bool = false;
static mut BUTTON2_PRESSED: bool = false;
static mut BUTTON3_PRESSED: bool = false;

/// Callback for button 1
extern "C" fn on_button1_click() {
    unsafe {
        BUTTON1_PRESSED = !BUTTON1_PRESSED;
        if let Some(ref mut btn) = BUTTON1 {
            btn.set_colors(
                if BUTTON1_PRESSED { Color::GREEN } else { Color::BLUE },
                Color::DARK_GRAY,
                Color::WHITE,
            );
        }
    }
}

/// Callback for button 2
extern "C" fn on_button2_click() {
    unsafe {
        BUTTON2_PRESSED = !BUTTON2_PRESSED;
        if let Some(ref mut btn) = BUTTON2 {
            btn.set_colors(
                if BUTTON2_PRESSED { Color::GREEN } else { Color::BLUE },
                Color::DARK_GRAY,
                Color::WHITE,
            );
        }
    }
}

/// Callback for button 3
extern "C" fn on_button3_click() {
    unsafe {
        BUTTON3_PRESSED = !BUTTON3_PRESSED;
        if let Some(ref mut btn) = BUTTON3 {
            btn.set_colors(
                if BUTTON3_PRESSED { Color::GREEN } else { Color::BLUE },
                Color::DARK_GRAY,
                Color::WHITE,
            );
        }
    }
}

/// Initialize the demo UI
pub unsafe fn init_demo_ui() {
    // Create top panel (status bar)
    TOP_PANEL = Some(Panel::new(0, 0, 1080, 40));
    TOP_PANEL.as_mut().unwrap().set_colors(Color::DARK_GRAY, Color::GRAY);
    
    // Create bottom panel (navigation bar)
    BOTTOM_PANEL = Some(Panel::new(0, 1880, 1080, 40));
    BOTTOM_PANEL.as_mut().unwrap().set_colors(Color::DARK_GRAY, Color::GRAY);
    
    // Create welcome label
    HOME_LABEL = Some(Label::new("Welcome to PhoneOS!", 50, 100, 980, 60));
    HOME_LABEL.as_mut().unwrap().set_colors(Color::WHITE, Color::BLACK);
    HOME_LABEL.as_mut().unwrap().set_align(TextAlign::Center);
    
    // Create instruction label
    let instruction = Label::new("Tap the buttons below:", 50, 200, 980, 40);
    // Note: Can't add to static, would need proper initialization
    
    // Create three demo buttons
    BUTTON1 = Some(Button::new("Button 1", 100, 300, 280, 80));
    BUTTON1.as_mut().unwrap().set_on_click(on_button1_click);
    
    BUTTON2 = Some(Button::new("Button 2", 400, 300, 280, 80));
    BUTTON2.as_mut().unwrap().set_on_click(on_button2_click);
    
    BUTTON3 = Some(Button::new("Button 3", 700, 300, 280, 80));
    BUTTON3.as_mut().unwrap().set_on_click(on_button3_click);
    
    // Create screen
    DEMO_SCREEN = Some(Screen::new());
    DEMO_SCREEN.as_mut().unwrap().set_background(Color::BLACK);
    
    // Add widgets to screen (would need proper implementation)
}

/// Draw the demo UI
pub unsafe fn draw_demo_ui(fb: &mut FramebufferWriter) {
    let font = FontRenderer::new();
    
    // Draw panels
    if let Some(ref panel) = TOP_PANEL {
        panel.draw(fb, &font);
    }
    if let Some(ref panel) = BOTTOM_PANEL {
        panel.draw(fb, &font);
    }
    
    // Draw labels
    if let Some(ref label) = HOME_LABEL {
        label.draw(fb, &font);
    }
    
    // Draw buttons
    if let Some(ref button) = BUTTON1 {
        button.draw(fb, &font);
    }
    if let Some(ref button) = BUTTON2 {
        button.draw(fb, &font);
    }
    if let Some(ref button) = BUTTON3 {
        button.draw(fb, &font);
    }
    
    // Draw click count
    let count_text = "Touch events received";
    font.draw_string(fb, count_text, 50, 500, Color::LIGHT_GRAY, Color::BLACK);
}

/// Handle input for demo UI
pub unsafe fn handle_demo_input() {
    let event = poll_touch();
    
    if event != TouchEvent::None {
        // Convert touch event coordinates
        match event {
            TouchEvent::Press(x, y) | TouchEvent::Release(x, y) | TouchEvent::Move(x, y) => {
                let ui_event = crate::drivers::ui::UIEvent::TouchDown(x as usize, y as usize);
                // Would process through screen handler
            }
            _ => {}
        }
    }
}

/// Main demo loop
pub unsafe fn run_demo_loop(fb: &mut FramebufferWriter) {
    init_demo_ui();
    
    // Initial draw
    draw_demo_ui(fb);
    
    // In real implementation, this would be the main event loop
    // For now, just demonstrate the UI is ready
    let font = FontRenderer::new();
    font.draw_string(fb, "UI System Ready", 50, 600, Color::GREEN, Color::BLACK);
    font.draw_string(fb, "Framebuffer: 1080x1920", 50, 650, Color::LIGHT_GRAY, Color::BLACK);
    font.draw_string(fb, "Touch Driver: Active", 50, 700, Color::LIGHT_GRAY, Color::BLACK);
    font.draw_string(fb, "Font Renderer: 8x16", 50, 750, Color::LIGHT_GRAY, Color::BLACK);
}
