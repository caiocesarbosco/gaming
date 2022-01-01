extern crate piston;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate graphics;
extern crate find_folder;
extern crate freetype as ft;

use constants::constants::colors::BLACK;
use piston::input::{Button, ButtonState, Key};
use piston::{ButtonEvent, RenderEvent, WindowSettings};
use piston::event_loop::{EventSettings, Events, EventLoop};
use graphics::character::CharacterCache;
use graphics::Transformed;
use opengl_graphics::{Filter, GlGraphics, GlyphCache, OpenGL, TextureSettings};
use glutin_window::GlutinWindow;

mod constants;
mod graphical;

use crate::constants::constants::{windows_resolutions::{STANDARD, PIXEL_SIZE}, strings::GAME_NAME, colors::BLUE};
use crate::graphical::character::character::Player;

fn main() {
    let opengl = OpenGL::V3_2;

    let map = graphical::map::map::make_map();

    let settings = WindowSettings::new(GAME_NAME, STANDARD).exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");

    let mut gl = GlGraphics::new(opengl);

    let event_settings = EventSettings::new();
    let mut events = Events::new(event_settings);

    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    let freetype = ft::Library::init().unwrap();
    let font = assets.join("FiraSans-Regular.ttf");

    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new(font, (), texture_settings).expect("Could not load font");

    while let Some(evt) = events.next(&mut window) {
        if let Some(r) = evt.render_args() {
            gl.draw(r.viewport(), |c, g| {
                for i in 0..map.len() {
                    for j in 0..map[i].len() {
                        
                        let pos: [f64;4] = [ PIXEL_SIZE * i as f64,
                            PIXEL_SIZE * j as f64,
                            PIXEL_SIZE * (i + 1) as f64,
                            PIXEL_SIZE * (j + 1) as f64,
                        ];

                        let colour = [map[i][j].asset[0] as f32, map[i][j].asset[1] as f32, map[i][j].asset[2] as f32, map[i][j].asset[3] as f32];
                        graphics::Rectangle::new(colour).draw(
                            pos,
                            &c.draw_state,
                            c.transform,
                            g,
                        );    
                    }
                }

                let mut player : Player = Player::new('H',BLUE,1,1);
                let character = glyphs.character(PIXEL_SIZE as u32, player.name).unwrap();

                let colour = [player.colour[0] as f32, player.colour[1] as f32, player.colour[2] as f32, player.colour[3] as f32];

                graphics::Image::new_color(colour).draw(
                    character.texture,
                    &c.draw_state,
                    c.transform.trans(player.pos_x as f64 , player.pos_y as f64),
                    g,
                    );
            });
        }
    }
}
