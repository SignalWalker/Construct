#![feature(vec_resize_with)]

extern crate gfx_backend_gl as gl;
extern crate gfx_hal as hal;
extern crate ground_control;
extern crate painter;
extern crate shaderc;

//use painter::architect::birch::*;
use ground_control::winit::*;
use ground_control::ControlMason;
use painter::architect::{import::*, *};
use painter::color::ColorMason;
use painter::style::StyleMason;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Result;
use std::iter::FromIterator;
use std::time::Instant;

use hal::*;

mod shader;

fn main() {
    let mut event_loop = winit::EventsLoop::new();
    let win_builder = WindowBuilder::new()
        .with_dimensions(ground_control::winit::dpi::LogicalSize::new(640.0, 480.0))
        .with_resizable(true)
        .with_transparency(true)
        .with_decorations(false)
        .with_title("Construct".into());

    let (window, mut adapters, mut surface) = {
        let window = {
            let builder =
                gl::config_context(gl::glutin::ContextBuilder::new(), ColorFormat::SELF, None)
                    .with_vsync(false);
            gl::glutin::GlWindow::new(win_builder, builder, &event_loop).unwrap()
        };
        let surface = gl::Surface::from_window(window);
        let adapters = surface.enumerate_adapters();
        (window, adapters, surface);
    };

    let mut done = false;
    let mut stale = true;
    let mut resized = true;

    let file = File::open("resources/test.stn").unwrap();
    let bufread = BufReader::new(file);

    let mut mason = MetaMason::new();
    let read_time = Instant::now();
    let (mut arch, mut map) = Architect::from_buffer(bufread).unwrap();
    let rem = mason.handle_stones(&mut arch, &mut map);
    for r in rem {
        if arch.stones.nodes()[r].is_some() {
            arch.stones.remove(r);
        }
    }
    arch.stones.clean();
    println!(
        "Read Time: {:?}, Tree:\n{}",
        read_time.elapsed(),
        arch.to_xml()
    );

    //println!("Colors: {:?}", mason.color.colors);
    //println!("Actions: {:?}", mason.settings.control.key_tree);

    let mut buf: Vec<Color> = Vec::new();

    while !done {
        if resized {
            let size = renderer.size();
            buf.resize_with((size[0] * size[1]) as usize, || [0, 0, 0, 255]);
            stale = true;
            resized = false;
        }
        if stale {
            use flint::lightcycle::plane::*;
            let frame_time = Instant::now();
            let size = *renderer.size();
            painter::paint_tree(
                Buffer::from_slice([size[0] as usize, size[1] as usize].into(), &mut buf[..]),
                &mason.color.colors,
                &arch.stones,
                Some([0, 0, 0, 255]),
            );
            renderer.render_frame(Some(&buf[..]));
            println!("Frame: {:?}", frame_time.elapsed());
            stale = false
        }
        match event_loop {
            EventLoop::GL(ref mut lp) => {
                use flint::glium::glutin::{Event, WindowEvent};
                lp.poll_events(|ev| match ev {
                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        ..
                    } => done = true,
                    Event::WindowEvent {
                        event: WindowEvent::KeyboardInput { input, device_id },
                        ..
                    } => {
                        for action in mason
                            .settings
                            .control
                            .handle_key(flint::glutin_key_to_ground(input))
                        {
                            println!("Action: {}", action);
                            match &action[..] {
                                "key_info" => println!("ScanCode: {}", input.scancode),
                                "reload" => stale = true,
                                "quit" => done = true,
                                _ => (),
                            }
                        }
                    }
                    Event::WindowEvent {
                        event: WindowEvent::Resized(new),
                        ..
                    } => {
                        *renderer.size_mut() = [new.width as u32, new.height as u32];
                        resized = true;
                    }
                    _ => (),
                });
            }
            EventLoop::VK(ref mut lp) => panic!("Not Implemented"),
        }
    }
}

struct FileGet;

impl FileGetter for FileGet {
    fn get(&mut self, path: &str) -> Result<BufReader<File>> {
        let file = File::open(path)?;
        Ok(BufReader::new(file))
    }
}

struct SettingsMason {
    control: ControlMason,
}

impl Default for SettingsMason {
    fn default() -> SettingsMason {
        SettingsMason {
            control: ControlMason::default(),
        }
    }
}

impl StoneMason for SettingsMason {
    fn handle_stones(
        &mut self,
        arch: &mut Architect,
        map: &mut HashMap<String, Vec<usize>>,
    ) -> HashSet<usize> {
        let mut res = self.control.handle_stones(arch, map);
        for s in match map.get("meta") {
            Some(v) => v,
            None => return res,
        } {
            res.insert(*s);
        }
        res
    }
}

struct MetaMason {
    import: ImportMason<FileGet>,
    settings: SettingsMason,
    color: ColorMason,
    style: StyleMason,
}

impl MetaMason {
    pub fn new() -> MetaMason {
        MetaMason {
            import: ImportMason::new(FileGet),
            settings: SettingsMason::default(),
            color: ColorMason::default(),
            style: StyleMason::new(),
        }
    }
}

impl StoneMason for MetaMason {
    fn handle_stones(
        &mut self,
        arch: &mut Architect,
        map: &mut HashMap<String, Vec<usize>>,
    ) -> HashSet<usize> {
        let rem_i = self.import.handle_stones(arch, map);
        let rem_s = self.settings.handle_stones(arch, map);
        let rem_c = self.color.handle_stones(arch, map);
        let rem_st = self.style.handle_stones(arch, map);
        HashSet::from_iter(
            rem_i
                .iter()
                .chain(rem_s.iter().chain(rem_c.iter().chain(rem_st.iter())))
                .cloned(),
        )
    }
}
