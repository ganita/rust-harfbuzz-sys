// Copyright 2017 Sreejith Krishnan R <sreejith@ganita.io>
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file
// except in compliance with the License. You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the
// License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND,
// either express or implied. See the License for the specific language governing permissions
// and limitations under the License.


extern crate harfbuzz_sys;
extern crate clap;

use std::path::{Path};
use std::fs::File;
use std::io::Read;
use std::ptr;
use std::ffi::CStr;

use clap::{Arg, App};

use harfbuzz_sys::hb_ot_math_constant_t;

fn main() {
    let matches = App::new("Harfbuzz Rust Sample")
        .version("0.1.0")
        .about("Print Opentype MATH constants from font file if present")
        .arg(Arg::with_name("FONT")
            .help("Sets the input font file to use")
            .required(true)
            .index(1))
        .get_matches();

    let file = matches.value_of("FONT").unwrap();

    dump_math_table(Path::new(file));
}

fn dump_math_table(path: &Path) {
    if !path.exists() {
        panic!("Font file not found at path {:?}", path);
    }

    let mut file = File::open(path).expect(&format!("Cannot open font file {:?}", path));

    let mut bytes: Vec<u8> = Vec::new();
    file.read_to_end(&mut bytes).expect(&format!("Cannot read font file {:?}", path));
    let file_size = bytes.len() as u32;

    unsafe {
        let blob = harfbuzz_sys::hb_blob_create(
            CStr::from_bytes_with_nul_unchecked(&bytes).as_ptr(),
            file_size,
            harfbuzz_sys::hb_memory_mode_t::HB_MEMORY_MODE_READONLY,
            ptr::null_mut(),
            None
        );

        let face = harfbuzz_sys::hb_face_create(blob, 0);
        let font = harfbuzz_sys::hb_font_create(face);

        if harfbuzz_sys::hb_ot_math_has_data(face) == 0 {
            println!("This font does not have math constants table");
            return;
        }

        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_SCRIPT_PERCENT_SCALE_DOWN);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_SCRIPT_SCRIPT_PERCENT_SCALE_DOWN);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_DELIMITED_SUB_FORMULA_MIN_HEIGHT);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_DELIMITED_SUB_FORMULA_MIN_HEIGHT);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_DISPLAY_OPERATOR_MIN_HEIGHT);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_MATH_LEADING);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_AXIS_HEIGHT);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_ACCENT_BASE_HEIGHT);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_FLATTENED_ACCENT_BASE_HEIGHT);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_SUBSCRIPT_SHIFT_DOWN);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_SUBSCRIPT_TOP_MAX);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_SUBSCRIPT_BASELINE_DROP_MIN);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_SUPERSCRIPT_SHIFT_UP);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_SUPERSCRIPT_SHIFT_UP_CRAMPED);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_SUPERSCRIPT_SHIFT_UP);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_SUPERSCRIPT_BOTTOM_MIN);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_SUPERSCRIPT_BASELINE_DROP_MAX);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_SUB_SUPERSCRIPT_GAP_MIN);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_SUPERSCRIPT_BOTTOM_MAX_WITH_SUBSCRIPT);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_SPACE_AFTER_SCRIPT);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_UPPER_LIMIT_GAP_MIN);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_UPPER_LIMIT_BASELINE_RISE_MIN);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_LOWER_LIMIT_GAP_MIN);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_LOWER_LIMIT_BASELINE_DROP_MIN);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_STACK_TOP_SHIFT_UP);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_STACK_TOP_DISPLAY_STYLE_SHIFT_UP);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_STACK_BOTTOM_SHIFT_DOWN);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_STACK_BOTTOM_DISPLAY_STYLE_SHIFT_DOWN);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_STACK_GAP_MIN);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_STACK_DISPLAY_STYLE_GAP_MIN);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_STRETCH_STACK_TOP_SHIFT_UP);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_STRETCH_STACK_BOTTOM_SHIFT_DOWN);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_STRETCH_STACK_GAP_ABOVE_MIN);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_STRETCH_STACK_GAP_BELOW_MIN);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_FRACTION_NUMERATOR_SHIFT_UP);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_FRACTION_NUMERATOR_DISPLAY_STYLE_SHIFT_UP);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_FRACTION_DENOMINATOR_SHIFT_DOWN);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_FRACTION_DENOMINATOR_DISPLAY_STYLE_SHIFT_DOWN);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_FRACTION_NUMERATOR_GAP_MIN);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_FRACTION_NUM_DISPLAY_STYLE_GAP_MIN);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_FRACTION_RULE_THICKNESS);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_FRACTION_DENOMINATOR_GAP_MIN);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_FRACTION_DENOM_DISPLAY_STYLE_GAP_MIN);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_SKEWED_FRACTION_HORIZONTAL_GAP);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_SKEWED_FRACTION_VERTICAL_GAP);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_OVERBAR_VERTICAL_GAP);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_OVERBAR_RULE_THICKNESS);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_OVERBAR_EXTRA_ASCENDER);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_UNDERBAR_VERTICAL_GAP);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_UNDERBAR_RULE_THICKNESS);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_UNDERBAR_EXTRA_DESCENDER);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_RADICAL_VERTICAL_GAP);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_RADICAL_DISPLAY_STYLE_VERTICAL_GAP);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_RADICAL_RULE_THICKNESS);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_RADICAL_EXTRA_ASCENDER);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_RADICAL_KERN_BEFORE_DEGREE);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_RADICAL_KERN_AFTER_DEGREE);
        print_constant(font, hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_RADICAL_DEGREE_BOTTOM_RAISE_PERCENT);

        harfbuzz_sys::hb_blob_destroy(blob);
        harfbuzz_sys::hb_face_destroy(face);
        harfbuzz_sys::hb_font_destroy(font);
    }
}

fn print_constant(font: *mut harfbuzz_sys::hb_font_t, constant: harfbuzz_sys::hb_ot_math_constant_t) {
    println!("{:?} : {}", constant, unsafe { harfbuzz_sys::hb_ot_math_get_constant(font, constant) });
}