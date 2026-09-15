// remeber sweet16 from CinderROS?? WELL DONT YOU WORRY ITS BACK >:3
pub static FONT_DATA: &[u8] = include_bytes!("font.f8");

pub fn glyph(c: u8) -> &'static [u8] {
    let o = (c as usize) * 16;
    &FONT_DATA[o..o /*funy face*/+ 16] //8x16 1 bit per px just like how i did it last time :3
}