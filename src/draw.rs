extern crate piston;
extern crate piston_window;
extern crate touch_visualizer;

use piston::input::*;
use piston_window::*;

use crate::game::{ Game };
use crate::board::{ Board };
use crate::moves::{ Moves };

pub struct Draw { }

impl Draw {
    pub fn render(win: Game, map: Board) {
        let mut window: PistonWindow = WindowSettings::new("Checkers.rs", win.window_size)
                    .resizable(false)
                    .exit_on_esc(true)
                    .build()
                    .unwrap();

        let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
        let mut glyphs = window.load_font(assets.join("FiraSans-Regular.ttf")).unwrap();

        let mut position: [[usize; 10]; 10] = [[0; 10]; 10];
        let mut user_position: [usize; 2] = [1, 1];

        while let Some(event) = window.next() {

            if let Some(press_args) = event.press_args() {
                match press_args {
                    Button::Keyboard(Key::Up) => Moves::current_move(Moves::Up, &mut user_position),
                    Button::Keyboard(Key::Right) => Moves::current_move(Moves::Right, &mut user_position),
                    Button::Keyboard(Key::Down) => Moves::current_move(Moves::Down, &mut user_position),
                    Button::Keyboard(Key::Left) => Moves::current_move(Moves::Left, &mut user_position),
                    Button::Keyboard(Key::Space) => Moves::set_current_position(&mut user_position, &mut position),
                    _ => ()
                };
                println!("------------------------------");
                for i in 0..10 {
                    println!("{:?}", position[i]);
                }
            }

            window.draw_2d(&event, |context, graphics, _device| {

                clear([1.0; 4], graphics);
                // x axis
                rectangle([0.0, 0.0, 0.0, 1.0], [175.0, 5.0, 515.0, 5.0], context.transform, graphics);
                rectangle([0.0, 0.0, 0.0, 1.0], [5.0, 175.0, 690.0, 5.0], context.transform, graphics);
                rectangle([0.0, 0.0, 0.0, 1.0], [5.0, 690.0, 690.0, 5.0], context.transform, graphics);
                // y axis
                rectangle([0.0, 0.0, 0.0, 1.0], [5.0, 175.0, 5.0, 515.0], context.transform, graphics);
                rectangle([0.0, 0.0, 0.0, 1.0], [175.0, 5.0, 5.0, 690.0], context.transform, graphics);
                rectangle([0.0, 0.0, 0.0, 1.0], [690.0, 5.0, 5.0, 690.0], context.transform, graphics);
                
                let(mut x_axis, mut y_axis) = (180.0 , 180.0);

                for _ in 0..9 {
                    x_axis += 46.6;
                    rectangle([0.0, 0.0, 0.0, 1.0], [x_axis, 5.0, 5.0, 690.0], context.transform, graphics);
                    x_axis += 5.0;
                }

                for _ in 0..9 {
                    y_axis += 46.6;
                    rectangle([0.0, 0.0, 0.0, 1.0], [5.0, y_axis, 690.0, 5.0], context.transform, graphics);
                    y_axis += 5.0;
                }

                x_axis = 183.0;
                y_axis = 183.0;

                for j in 0..10 {
                    for i in 0..10 {
                        if position[j][i] == 1 { 
                            rectangle([0.0, 0.0, 0.0, 1.0], [x_axis, y_axis, 41.0, 41.0], context.transform, graphics);
                        }
                        x_axis += 51.6;
                    }
                    x_axis = 183.0;
                    y_axis += 51.6;
                }

                let mut y: f64 = 214.0;

                for i in 0..map.horizontal_vec.len() {
                    let mut x: f64 = 145.0;
                    let g = map.horizontal_vec[i].len();
                    for j in (0..g).rev() {
                        let transform = context.transform.trans(x, y);
                        text::Text::new_color([0.0, 0.0, 0.0, 1.0], 25).draw(
                            &map.horizontal_vec[i][j].to_string(),
                            &mut glyphs,
                            &context.draw_state,
                            transform, graphics
                        ).unwrap();
                
                        glyphs.factory.encoder.flush(_device);
                        x -= 28.0;
                    }
                    y += 51.5;
                }

                let mut x: f64 = 198.0;

                for i in 0..map.vertical_vec.len() {
                    let mut y: f64 = 165.0;
                    let g = map.vertical_vec[i].len();
                    if i == 5 {
                        x += 5.0;
                    }
                    for j in (0..g).rev() {
                        let transform = context.transform.trans(x, y);
                        text::Text::new_color([0.0, 0.0, 0.0, 1.0], 25).draw(
                            &map.vertical_vec[i][j].to_string(),
                            &mut glyphs,
                            &context.draw_state,
                            transform, graphics
                        ).unwrap();
                
                        glyphs.factory.encoder.flush(_device);
                        y -= 31.0;
                    }
                    x += 51.0;
                }

                Moves::if_win(position, map.map);
            });
        }
    }
}