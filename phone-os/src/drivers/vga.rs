//! VGA Text Mode Buffer for x86_64
//! 
//! Provides basic text output to the screen through VGA buffer

/// VGA buffer is at physical address 0xB8000 in text mode
const VGA_BUFFER_ADDRESS: usize = 0xB8000;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

/// Color codes for VGA text mode
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

/// Create a color code from foreground and background colors
#[inline]
const fn make_color(fg: Color, bg: Color) -> u8 {
    (bg as u8) << 4 | (fg as u8)
}

/// A character with its color attribute
#[repr(C)]
#[derive(Clone, Copy)]
struct VgaCharacter {
    character: u8,
    color_code: u8,
}

/// VGA Text Buffer writer
pub struct VgaWriter {
    cursor_x: usize,
    cursor_y: usize,
    color: u8,
}

impl VgaWriter {
    /// Create a new VGA writer
    pub const fn new() -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            color: make_color(Color::LightGray, Color::Black),
        }
    }

    /// Get pointer to VGA buffer (raw pointer to avoid borrow issues)
    fn get_buffer_ptr() -> *mut [VgaCharacter; VGA_WIDTH * VGA_HEIGHT] {
        VGA_BUFFER_ADDRESS as *mut [VgaCharacter; VGA_WIDTH * VGA_HEIGHT]
    }

    /// Write a byte to the screen
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => {
                self.cursor_x = 0;
                self.cursor_y += 1;
            }
            0x20..=0x7e => {
                let index = self.cursor_y * VGA_WIDTH + self.cursor_x;
                if index < VGA_WIDTH * VGA_HEIGHT {
                    unsafe {
                        let buffer = &mut *Self::get_buffer_ptr();
                        buffer[index] = VgaCharacter {
                            character: byte,
                            color_code: self.color,
                        };
                    }
                }
                self.cursor_x += 1;
            }
            _ => {}
        }

        // Handle line wrapping
        if self.cursor_x >= VGA_WIDTH {
            self.cursor_x = 0;
            self.cursor_y += 1;
        }

        // Handle scrolling
        if self.cursor_y >= VGA_HEIGHT {
            self.scroll();
            self.cursor_y = VGA_HEIGHT - 1;
        }
    }

    /// Write a string to the screen
    pub fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }

    /// Scroll the screen up by one line
    fn scroll(&mut self) {
        unsafe {
            let buffer = &mut *Self::get_buffer_ptr();
            
            // Move all lines up by one
            for y in 1..VGA_HEIGHT {
                for x in 0..VGA_WIDTH {
                    let src = y * VGA_WIDTH + x;
                    let dst = (y - 1) * VGA_WIDTH + x;
                    buffer[dst] = buffer[src];
                }
            }

            // Clear the last line
            let blank = VgaCharacter {
                character: b' ',
                color_code: self.color,
            };
            for x in 0..VGA_WIDTH {
                buffer[(VGA_HEIGHT - 1) * VGA_WIDTH + x] = blank;
            }
        }
    }

    /// Clear the screen
    pub fn clear(&mut self) {
        unsafe {
            let buffer = &mut *Self::get_buffer_ptr();
            let blank = VgaCharacter {
                character: b' ',
                color_code: self.color,
            };
            for i in 0..(VGA_WIDTH * VGA_HEIGHT) {
                buffer[i] = blank;
            }
        }
        self.cursor_x = 0;
        self.cursor_y = 0;
    }

    /// Set the text color
    pub fn set_color(&mut self, fg: Color, bg: Color) {
        self.color = make_color(fg, bg);
    }
}

// Implement core::fmt::Write for use with write! macro
impl core::fmt::Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_str(s);
        Ok(())
    }
}
