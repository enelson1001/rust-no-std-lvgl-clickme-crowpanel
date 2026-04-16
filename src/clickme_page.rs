use crate::{display::Screen, lvgl_sys};
use core::fmt::Write;
use core::sync::atomic::{AtomicBool, Ordering};
use heapless::String;
use lvgl_sys::*;

/*******************************************************************************************
 *  CONSTANTS  - most of theses are used to reduce typing
 ******************************************************************************************/
//const COLOR_WHITE: lv_color_t = color_from_rgb(0xFF, 0xFF, 0xFF);
const COLOR_BLACK: lv_color_t = color_from_rgb(0x00, 0x00, 0x00);
const EVT_CLICKED: lv_event_code_t = lv_event_code_t_LV_EVENT_CLICKED;

// The shared state between UI events and the Embassy task
pub static BTN_CLICKED: AtomicBool = AtomicBool::new(false);

pub struct ClickMePage {
    count_label: *mut lv_obj_t,
    btn_label: *mut lv_obj_t,
}

impl ClickMePage {
    pub fn new(screen: Screen) -> Self {
        unsafe {
            // --- Get pointer to the screen ---
            let screen_ptr = screen.as_ptr();

            // --- Style the Screen ---
            lv_obj_set_style_bg_color(screen_ptr, color_from_rgb(53, 56, 57), 0);
            lv_obj_set_style_bg_opa(screen_ptr, 255, 0);

            // --- Create Count Label ---
            let count_label = lv_label_create(screen_ptr);
            lv_obj_align(count_label, LV_ALIGN_TOP_MID as u8, 0, 40);
            lv_obj_set_style_text_color(count_label, COLOR_BLACK, 0);
            lv_obj_set_style_text_font(count_label, &lv_font_montserrat_28, 0);
            //lv_obj_set_style_text_font(count_label, &gotham_bold_80, 0); // to try a custom font(digits 0-9) comment out previous line
            lv_label_set_text(count_label, c"0".as_ptr());

            // --- Create Button ---
            let btn = lv_btn_create(screen_ptr);
            lv_obj_set_size(btn, 160, 70);
            lv_obj_align(btn, LV_ALIGN_LEFT_MID as u8, 20, 0);

            // --- Add the click event for button ---
            lv_obj_add_event_cb(
                btn,
                Some(btn_event_handler),
                EVT_CLICKED,
                core::ptr::null_mut(),
            );

            // --- Create Button Label ---
            let btn_label = lv_label_create(btn);
            lv_obj_align(btn_label, LV_ALIGN_CENTER as u8, 0, 0);
            lv_obj_set_style_text_color(btn_label, COLOR_BLACK, 0);
            lv_obj_set_style_text_font(btn_label, &lv_font_montserrat_28, 0);
            lv_label_set_text(btn_label, c"Click Me".as_ptr());

            Self {
                count_label,
                btn_label,
            }
        }
    }

    pub fn set_counter_value(&self, value: u32) {
        // Create a fixed-capacity string on the stack (16 or 32 bytes is plenty)
        let mut s: String<32> = String::new();
        if write!(s, "{}\0", value).is_ok() {
            unsafe {
                lv_label_set_text(self.count_label, s.as_ptr() as *const _);
            }
        }
    }

    pub fn set_btn_text(&self, btn_state: bool) {
        unsafe {
            if btn_state {
                lv_label_set_text(self.btn_label, c"Click Me!".as_ptr());
            } else {
                lv_label_set_text(self.btn_label, c"Clicked!".as_ptr());
            }
        }
    }
}

// C-style callback for LVGL
#[esp_hal::ram]
unsafe extern "C" fn btn_event_handler(_e: *mut lv_event_t) {
    BTN_CLICKED.store(true, Ordering::Relaxed);
}

// To use a custom font or image you must create an unsafe extern "C" like below.
// This must match the name used in the custom font C file exactly
// Multiple custom fonts can be added by repeating this pattern with different names and C files.
// For example:
// unsafe extern "C" {
//     pub static gotham_bold_80: lvgl_sys::lv_font_t;
//     pub static gotham_bold_40: lvgl_sys::lv_font_t;
//
//     // You can even mix types in here
//     pub static img_hand: lvgl_sys::lv_img_dsc_t;
// }
unsafe extern "C" {
    pub static gotham_bold_80: lv_font_t;
}

/*******************************************************************************************
 *  LVGL Helpers - mostly static inline code that bindgen did not create from lglg/src
 ******************************************************************************************/

#[inline(always)]
const fn color_from_rgb(r: u8, g: u8, b: u8) -> lv_color_t {
    // Scale 8-bit colors down to 5-6-5 bit depths
    // Red:   8 bits -> 5 bits (r >> 3)
    // Green: 8 bits -> 6 bits (g >> 2)
    // Blue:  8 bits -> 5 bits (b >> 3)
    let r5 = (r >> 3) as u16;
    let g6 = (g >> 2) as u16;
    let b5 = (b >> 3) as u16;

    // Manually pack the RGB565 bits:
    // Red   is bits 11-15
    // Green is bits 5-10
    // Blue  is bits 0-4
    let full = (r5 << 11) | (g6 << 5) | b5;

    lv_color_t { full }
}
