use crate::util;
use gl;
use sdl2;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use crate::parser;
use std;
use std::io::Write;

const RES_HEIGHT: usize = 240;
const RES_WIDTH:  usize = 256;

pub fn do_window() {
    let filepath = std::path::Path::new("test/test_roms/Dragon Warrior (USA).nes");
    let rom = parser::parse_rom(filepath.to_str().unwrap()).unwrap();
    let ctx = sdl2::init().unwrap();
    let video_subsystem = ctx.video().unwrap();
    //let mut timer = ctx.timer().unwrap();
    let window = video_subsystem.window("Window", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .target_texture()
        .present_vsync()
        .index(util::find_sdl_gl_driver().unwrap())
        .build().unwrap();

    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
    canvas.window().gl_set_context_to_current().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let surf_data = rom.chr_rom.clone();
    let mut buf: [u8; RES_HEIGHT * RES_WIDTH] = [0; RES_HEIGHT * RES_WIDTH];
    for i in 0..(200) {
        buf[i] = surf_data[i];
    }
    //let surface = Surface::from_data(&mut buf, 10, 20, 0, sdl2::pixels::PixelFormatEnum::Unknown).unwrap();

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_target(texture_creator.default_pixel_format(), 20, 10)
        .unwrap();
    for val in buf.iter() {
        print!("{},",val);
        std::io::stdout().flush().unwrap();
    }
    texture.update(None, &buf, 10).unwrap();
    canvas.copy(&texture, None, Rect::new(0, 0, 200, 100)).unwrap();
    canvas.present();

    let joystick_subsystem = ctx.joystick().unwrap();

    println!("Number of controllers: {}", joystick_subsystem.num_joysticks().unwrap());
    let controller = joystick_subsystem.open(0).unwrap();

    //println!("Controller mapping: {}", controller.mapping());

    let mut events = ctx.event_pump().unwrap();
    'event: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit{..} => break 'event,
                Event::KeyDown{keycode: Some(Keycode::A), ..} => {println!("Hello"); canvas.present()},
                Event::JoyAxisMotion{value: val, ..} => {println!("joypad: {:?}", val); canvas.present()},
                _               => continue,
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}