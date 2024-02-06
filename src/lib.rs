use color::*;


extern "C" {
    fn js_fill_rect(x: i32, y: i32, width: i32, height: i32, color: u32);
}

fn fill_rect(x: i32, y: i32, width: i32, height: i32, color: Color) {
    unsafe {
        js_fill_rect(x, y, width, height, color.to_u32());
    }
}


#[no_mangle]
pub extern "C" fn render() {
    fill_rect(10, 20, 100, 50, Color::rgb(0, 0, 255));
}


mod color;
