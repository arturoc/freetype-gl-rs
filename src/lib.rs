use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use std::mem;
use std::slice;
use std::ptr;

mod ffi;

#[cfg(all(debug_assertions, windows))]
mod link_windowsd;
#[cfg(any(not(debug_assertions), not(windows)))]
mod link;

pub struct TextureFont{
    font: *mut ffi::texture_font_t,
    bytes: Option<Vec<u8>>,
}

impl Drop for TextureFont{
    fn drop(&mut self){
        unsafe{ ffi::texture_font_delete(self.font) }
    }
}

unsafe fn char_to_utf8(c: char) -> Vec<u8>{
    let c = c.to_string();
    CString::from_vec_unchecked(c.as_bytes().to_vec()).as_bytes().to_vec()
}

impl TextureFont{
	pub fn load(path: &str, pt_size: f32, depth: usize) -> Option<TextureFont>{
        unsafe{
            let tex_atlas = ffi::texture_atlas_new(512, 512, depth as u64);
            if tex_atlas == ptr::null_mut() {
                return None
            }

            // println!("allocated tex atlas  {},{}x{}", (*tex_atlas).width, (*tex_atlas).height, (*tex_atlas).depth);
            let path = CString::new(path.as_bytes()).unwrap();
            let c_path = path.as_ptr();
            let tex_font = ffi::texture_font_new_from_file( tex_atlas, pt_size, c_path);
            if tex_font == ptr::null_mut() {
                return None
            }

            let latin1 = (32 as u8 .. 255).collect::<Vec<_>>();
            let glyphs_i32 = CString::new(latin1).unwrap();
            ffi::texture_font_load_glyphs(tex_font, glyphs_i32.into_raw());
            // println!("loaded {} glyphs", ffi::vector_size((*tex_font).glyphs));

            Some(TextureFont{
                font: tex_font,
                bytes: None,
            })
        }
	}

	pub fn load_from_memory(font_data: Vec<u8>, pt_size: f32, depth: usize) -> Option<TextureFont>{
        unsafe{
            let tex_atlas = ffi::texture_atlas_new(512,512, depth as u64);
            if tex_atlas == ptr::null_mut() {
                return None
            }

            // println!("allocated tex atlas  {},{}x{}", (*tex_atlas).width, (*tex_atlas).height, (*tex_atlas).depth);
            let tex_font = ffi::texture_font_new_from_memory(
                tex_atlas,
                pt_size,
                font_data.as_ptr() as *const c_void,
                font_data.len() as u64);
            if tex_font == ptr::null_mut() {
                return None
            }

            let latin1 = (32 as u8 .. 255).collect::<Vec<_>>();
            let glyphs_i32 = CString::new(latin1).unwrap();
            ffi::texture_font_load_glyphs(tex_font, glyphs_i32.into_raw());

            // println!("loaded {} glyphs", ffi::vector_size((*tex_font).glyphs));

            Some(TextureFont{
                font: tex_font,
                bytes: Some(font_data),
            })
        }
	}

    #[inline]
	pub fn glyph(&self, c: char) -> Option<TextureGlyph>{
        unsafe{
            let glyph = ffi::texture_font_get_glyph(self.font, char_to_utf8(c).as_ptr() as *const c_char);
            if glyph != ptr::null_mut() {
                Some(TextureGlyph{
                    glyph
                })
            }else{
                None
            }
        }
	}

    #[inline]
	pub fn glyph_by_freetype_id(&self, glyph_id: u32) -> Option<TextureGlyph>{
        unsafe{
            let glyph = ffi::texture_font_get_glyph_by_id(self.font, glyph_id);
            if glyph != ptr::null_mut() {
                Some(TextureGlyph{
                    glyph
                })
            }else{
                None
            }
        }
	}

    /// Font size
    #[inline]
    pub fn size(&self) -> f32{
		unsafe{ (*self.font).size }
	}

    /// Whether to use autohint when rendering font
    #[inline]
    pub fn hinting(&self) -> i32{
		unsafe{ (*self.font).hinting }
	}

    /// Mode the font is rendering its next glyph
    #[inline]
    pub fn rendermode(&self) -> RenderMode{
		unsafe{ mem::transmute((*self.font).rendermode) }
	}

    /// Outline thickness
    #[inline]
    pub fn outline_thickness(&self) -> f32{
		unsafe{ (*self.font).outline_thickness }
	}

    /// Whether to use our own lcd filter.
    #[inline]
    pub fn filtering(&self) -> i32{
		unsafe{ (*self.font).filtering }
	}

    /// LCD filter weights
    #[inline]
    pub fn lcd_weights(&self) -> [ :: std :: os :: raw :: c_uchar ; 5usize ]{
		unsafe{ (*self.font).lcd_weights }
	}

    /// Whether to use kerning if available
    #[inline]
    pub fn kerning(&self) -> i32{
		unsafe{ (*self.font).kerning }
	}

    /// This field is simply used to compute a default line spacing (i.e., the
    /// baseline-to-baseline distance) when writing text with this font. Note
    /// that it usually is larger than the sum of the ascender and descender
    /// taken as absolute values. There is also no guarantee that no glyphs
    /// extend above or below subsequent baselines when using this distance.
    #[inline]
    pub fn height(&self) -> f32{
		unsafe{ (*self.font).height }
	}

    /// This field is the distance that must be placed between two lines of
    /// text. The baseline-to-baseline distance should be computed as:
    /// ascender - descender + linegap
    #[inline]
    pub fn linegap(&self) -> f32{
		unsafe{ (*self.font).linegap }
	}

    /// The ascender is the vertical distance from the horizontal baseline to
    /// the highest 'character' coordinate in a font face. Unfortunately, font
    /// formats define the ascender differently. For some, it represents the
    /// ascent of all capital latin characters (without accents), for others it
    /// is the ascent of the highest accented character, and finally, other
    /// formats define it as being equal to bbox.yMax.
    #[inline]
    pub fn ascender(&self) -> f32{
		unsafe{ (*self.font).ascender }
	}

    /// The descender is the vertical distance from the horizontal baseline to
    /// the lowest 'character' coordinate in a font face. Unfortunately, font
    /// formats define the descender differently. For some, it represents the
    /// descent of all capital latin characters (without accents), for others it
    /// is the ascent of the lowest accented character, and finally, other
    /// formats define it as being equal to bbox.yMin. This field is negative
    /// for values below the baseline.
    #[inline]
    pub fn descender(&self) -> f32{
		unsafe{ (*self.font).descender }
	}

    /// The position of the underline line for this face. It is the center of
    /// the underlining stem. Only relevant for scalable formats.
    #[inline]
    pub fn underline_position(&self) -> f32{
		unsafe{ (*self.font).underline_position }
	}

    /// The thickness of the underline for this face. Only relevant for scalable
    /// formats.
    #[inline]
    pub fn underline_thickness(&self) -> f32{
		unsafe{ (*self.font).underline_thickness }
	}

    #[inline]
    pub fn atlas(&self) -> TextureAtlas{
        TextureAtlas{ atlas: unsafe{ (*self.font).atlas } }
    }

    pub unsafe fn face(&self) -> ffi::FT_Face{
        (*self.font).face
    }
}

#[repr(u32)]
pub enum RenderMode{
    Normal = 0,
    OutlineEdge = 1,
    OutlinePositive = 2,
    OutlineNegatice = 3,
    SignedDistanceField = 4,
}

pub struct TextureGlyph{
    glyph: *mut ffi::texture_glyph_t
}

impl std::fmt::Debug for TextureGlyph{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TextureGlyph")
            .field("codepoint", &self.codepoint())
            .field("glyph_id", &self.glyph_id())
            .field("offset_x", &self.offset_x())
            .field("offset_y", &self.offset_y())
            .field("width", &self.width())
            .field("height", &self.height())
            .field("s0", &self.s0())
            .field("t0", &self.t0())
            .field("s1", &self.s1())
            .field("t1", &self.t1())
            .finish()
    }
}

impl TextureGlyph{
    #[inline]
	pub fn kerning(&self, c: char) -> f32{
        unsafe{
            ffi::texture_glyph_get_kerning(self.glyph, char_to_utf8(c).as_ptr() as *const c_char)
        }
	}

    /// Unicode codepoint this glyph represents in UTF-32 LE encoding.
    #[inline]
    pub fn codepoint(&self) -> u32 {
        unsafe{ (*self.glyph).codepoint }
    }

    /// Unicode codepoint this glyph represents in UTF-32 LE encoding.
    #[inline]
    pub fn glyph_id(&self) -> u32 {
        unsafe{ (*self.glyph).glyph_id }
    }

    /// Glyph's width in pixels.
    #[inline]
    pub fn width(&self) ->  usize{
        unsafe{ (*self.glyph).width as usize}
    }

    /// Glyph's height in pixels.
    #[inline]
    pub fn height(&self) ->  usize{
        unsafe{ (*self.glyph).height as usize }
    }

    /// Glyph's left bearing expressed in integer pixels.
    #[inline]
    pub fn offset_x(&self) ->  i32{
        unsafe{ (*self.glyph).offset_x }
    }

    /// Glyphs's top bearing expressed in integer pixels.
    ///
    /// Remember that this is the distance from the baseline to the top-most
    /// glyph scanline, upwards y coordinates being positive.
    #[inline]
    pub fn offset_y(&self) ->  i32{
        unsafe{ (*self.glyph).offset_y }
    }

    /// For horizontal text layouts, this is the horizontal distance (in
    /// fractional pixels) used to increment the pen position when the glyph is
    /// drawn as part of a string of text.
    #[inline]
    pub fn advance_x(&self) ->  f32{
        unsafe{ (*self.glyph).advance_x }
    }

    /// For vertical text layouts, this is the vertical distance (in fractional
   /// pixels) used to increment therendermode_t pen position when the glyph is drawn as
   /// part of a string of text.
   #[inline]
    pub fn advance_y(&self) ->  f32 {
        unsafe{ (*self.glyph).advance_y }
    }

    /// First normalized texture coordinate (x) of top-left corner
    #[inline]
    pub fn s0(&self) ->  f32 {
        unsafe{ (*self.glyph).s0 }
    }

    /// Second normalized texture coordinate (y) of top-left corner
    #[inline]
    pub fn t0(&self) ->  f32 {
        unsafe{ (*self.glyph).t0 }
    }

    /// First normalized texture coordinate (x) of bottom-right corner
    #[inline]
    pub fn s1(&self) ->  f32 {
        unsafe{ (*self.glyph).s1 }
    }

    /// Second normalized texture coordinate (y) of bottom-right corner
    #[inline]
    pub fn t1(&self) ->  f32 {
        unsafe{ (*self.glyph).t1 }
    }

    /// Mode this glyph was rendered
    #[inline]
    pub fn rendermode(&self) ->  RenderMode {
        unsafe{ mem::transmute((*self.glyph).rendermode) }
    }

    /// Glyph outline thickness
    #[inline]
    pub fn outline_thickness(&self) -> f32 {
        unsafe{ (*self.glyph).outline_thickness }
    }
}


pub struct TextureAtlas{
    atlas: *mut ffi::texture_atlas_t
}

impl TextureAtlas{
    /// Width (in pixels) of the underlying texture
    #[inline]
	pub fn width(&self) -> usize{
		unsafe{ (*self.atlas).width as usize }
	}

    /// Height (in pixels) of the underlying texture
    #[inline]
	pub fn height(&self) -> usize{
		unsafe{ (*self.atlas).height as usize }
	}

    /// Depth (in bytes) of the underlying texture
    #[inline]
	pub fn depth(&self) -> usize{
		unsafe{ (*self.atlas).depth as usize }
	}

    /// Allocated surface size
    #[inline]
	pub fn used(&self) -> usize{
		unsafe{ (*self.atlas).used as usize }
	}

    /// Texture identity (OpenGL)
    #[inline]
	pub fn id(&self) -> u32{
		unsafe{ (*self.atlas).id }
	}

    /// Atlas data
    #[inline]
	pub fn data(&self) -> &[u8]{
		unsafe{ slice::from_raw_parts((*self.atlas).data, self.width() * self.height() * self.depth()) }
	}

}
