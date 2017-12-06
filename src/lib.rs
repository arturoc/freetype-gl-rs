extern crate libc;

use libc::*;
use std::ffi::CString;

pub type Vector = c_void;

#[derive(Clone,Debug)]
#[repr(C)]
pub enum Location{
    File = 0,
    Memory,
}

#[derive(Clone,Debug)]
#[repr(C)]
pub struct FilenameOrMemory{
    filename_or_memory: *const c_void,
    size: c_uint
}

#[repr(C)]
pub struct TextureAtlas
{
    /**
     * Allocated nodes
     */
    pub nodes: *const Vector,

    /**
     *  Width (in pixels) of the underlying texture
     */
    pub width: size_t,

    /**
     * Height (in pixels) of the underlying texture
     */
    pub height: size_t,

    /**
     * Depth (in bytes) of the underlying texture
     */
    pub depth: size_t,

    /**
     * Allocated surface size
     */
    pub used: size_t,

    /**
     * Texture identity (OpenGL)
     */
    pub id: c_uint,

    /**
     * Atlas data
     */
    pub data: *mut c_uchar

}

#[derive(Clone,Debug)]
#[repr(C)]
pub struct TextureGlyph
{
    /**
     * Wide character this glyph represents
     */
    pub charcode: wchar_t,

    /**
     * Glyph id (used for display lists)
     */
    pub id: c_uint,

    /**
     * Glyph's width in pixels.
     */
    pub width: size_t,

    /**
     * Glyph's height in pixels.
     */
    pub height: size_t,

    /**
     * Glyph's left bearing expressed in integer pixels.
     */
    pub offset_x: c_int,

    /**
     * Glyphs's top bearing expressed in integer pixels.
     *
     * Remember that this is the distance from the baseline to the top-most
     * glyph scanline, upwards y coordinates being positive.
     */
    pub offset_y: c_int,

    /**
     * For horizontal text layouts, this is the horizontal distance (in
     * fractional pixels) used to increment the pen position when the glyph is
     * drawn as part of a string of text.
     */
    pub advance_x: c_float,

    /**
     * For vertical text layouts, this is the vertical distance (in fractional
     * pixels) used to increment the pen position when the glyph is drawn as
     * part of a string of text.
     */
    pub advance_y: c_float,

    /**
     * First normalized texture coordinate (x) of top-left corner
     */
    pub s0: c_float,

    /**
     * Second normalized texture coordinate (y) of top-left corner
     */
    pub t0: c_float,

    /**
     * First normalized texture coordinate (x) of bottom-right corner
     */
    pub s1: c_float,

    /**
     * Second normalized texture coordinate (y) of bottom-right corner
     */
    pub t1: c_float,

    /**
     * A vector of kerning pairs relative to this glyph.
     */
    pub kerning: *const Vector,

    /**
     * Glyph outline type (0 = None, 1 = line, 2 = inner, 3 = outer)
     */
    pub outline_type: c_int,

    /**
     * Glyph outline thickness
     */
    pub outline_thickness: c_float

}

#[derive(Clone,Debug)]
#[repr(C)]
pub struct TextureFont
{
    /**
     * Vector of glyphs contained in this font.
     */
    pub glyphs: *const Vector,

    /**
     * Atlas structure to store glyphs data.
     */
    pub atlas: *const TextureAtlas,

    /**
     * font location
     */
    pub location: Location,

    pub filename_or_memory: FilenameOrMemory,

    /**
     * Font size
     */
    pub size: c_float,

    /**
     * Whether to use autohint when rendering font
     */
    pub hinting: c_int,

    /**
     * Outline type (0 = None, 1 = line, 2 = inner, 3 = outer)
     */
    pub outline_type: c_int,

    /**
     * Outline thickness
     */
    pub outline_thickness: c_float,

    /**
     * Whether to use our own lcd filter.
     */
    pub filtering: c_int,

    /**
     * Whether to use kerning if available
     */
    pub kerning: c_int,

    /**
     * LCD filter weights
     */
    pub lcd_weights: [c_uchar;5],

    /**
     * This field is simply used to compute a default line spacing (i.e., the
     * baseline-to-baseline distance) when writing text with this font. Note
     * that it usually is larger than the sum of the ascender and descender
     * taken as absolute values. There is also no guarantee that no glyphs
     * extend above or below subsequent baselines when using this distance.
     */
    pub height: c_float,

    /**
     * This field is the distance that must be placed between two lines of
     * text. The baseline-to-baseline distance should be computed as:
     * ascender - descender + linegap
     */
    pub linegap: c_float,

    /**
     * The ascender is the vertical distance from the horizontal baseline to
     * the highest 'character' coordinate in a font face. Unfortunately, font
     * formats define the ascender differently. For some, it represents the
     * ascent of all capital latin characters (without accents), for others it
     * is the ascent of the highest accented character, and finally, other
     * formats define it as being equal to bbox.yMax.
     */
    pub ascender: c_float,

    /**
     * The descender is the vertical distance from the horizontal baseline to
     * the lowest 'character' coordinate in a font face. Unfortunately, font
     * formats define the descender differently. For some, it represents the
     * descent of all capital latin characters (without accents), for others it
     * is the ascent of the lowest accented character, and finally, other
     * formats define it as being equal to bbox.yMin. This field is negative
     * for values below the baseline.
     */
    pub descender: c_float,

    /**
     * The position of the underline line for this face. It is the center of
     * the underlining stem. Only relevant for scalable formats.
     */
    pub underline_position: c_float,

    /**
     * The thickness of the underline for this face. Only relevant for scalable
     * formats.
     */
    pub underline_thickness: c_float

}

#[link(name = "freetype-gl")]
#[link(name = "freetype")]
#[link(name = "z")]
#[link(name = "GL")]
extern{
  	fn texture_atlas_new( width: c_uint,
                     height: c_uint,
                     depth: c_uint ) -> *const TextureAtlas;

  	fn texture_font_new_from_file( atlas: *const TextureAtlas,
                              pt_size: c_float,
                              filename: *const c_char ) -> *const TextureFont;

    fn texture_font_new_from_memory( atlas: *const TextureAtlas,
                                pt_size: c_float,
                                memory_base: *const c_void,
                                memory_size: size_t ) -> *const TextureFont;


  	fn texture_font_load_glyphs( font: *const TextureFont,
                            charcodes: *const wchar_t ) -> size_t;


    fn texture_font_get_glyph( font: *const TextureFont,
	                          charcode: wchar_t ) -> *const TextureGlyph;

    fn texture_glyph_get_kerning( glyph: *const TextureGlyph,
		                        charcode: wchar_t ) -> c_float;
}

impl TextureFont{
		pub fn load(path: &str, pt_size: f32, depth: u32) -> TextureFont{
            unsafe{
                let tex_atlas = texture_atlas_new(512,512,depth);
                println!("allocated tex atlas  {},{}x{}", (*tex_atlas).width, (*tex_atlas).height, (*tex_atlas).depth);
                let c_path = CString::new(path.as_bytes()).unwrap().as_ptr();
                let tex_font = texture_font_new_from_file( tex_atlas, pt_size, c_path);

                let mut glyphs_i32: Vec<wchar_t> = Vec::new();
                for c in 32 as wchar_t..255{
                    glyphs_i32.push(c);
                }
                glyphs_i32.push(0i32);
                texture_font_load_glyphs(tex_font, &glyphs_i32[0]);
                (*tex_font).clone()
            }
		}

		pub fn load_from_memory(font_data: Vec<u8>, pt_size: f32, depth: u32) -> TextureFont{
            unsafe{
                let tex_atlas = texture_atlas_new(512,512,depth);
                println!("allocated tex atlas  {},{}x{}", (*tex_atlas).width, (*tex_atlas).height, (*tex_atlas).depth);
                let tex_font = texture_font_new_from_memory( tex_atlas, pt_size, font_data.as_ptr() as *const c_void, font_data.len() as size_t);

                let mut glyphs_i32: Vec<wchar_t> = Vec::new();
                for c in 32 as wchar_t..255{
                    glyphs_i32.push(c as wchar_t);
                }
                glyphs_i32.push(0i32);
                texture_font_load_glyphs(tex_font, &glyphs_i32[0]);
                (*tex_font).clone()
            }
		}

		pub fn get_glyph(&self, c: wchar_t) -> TextureGlyph{
            unsafe{
                let glyph = texture_font_get_glyph(self,c as wchar_t);
                (*glyph).clone()
            }
		}
}

impl TextureGlyph{
		pub fn get_kerning(&self, c: char) -> f32{
            unsafe{
                texture_glyph_get_kerning(self,c as wchar_t)
            }
		}
}
