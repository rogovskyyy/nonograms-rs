extern crate piston;
extern crate piston_window;
extern crate touch_visualizer;

use piston::input::*;
use piston_window::*;

use crate::game::{ Game };
use crate::board::{ Board };

enum Move {
    Up, Down, Left, Right, None
}

pub struct Draw { }

impl Draw {

    pub fn render(win: Game, map: Board) {
        let mut window: PistonWindow = WindowSettings::new("Checkers.rs", win.window_size)
                    .resizable(false)
                    .exit_on_esc(true)
                    .build()
                    .unwrap();

        let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
        let mut glyphs = window.load_font(assets.join("FiraSans-ExtraBold.ttf")).unwrap();

        //let mut touch_visualizer = TouchVisualizer::new();

        let mut current_move: Move;

        while let Some(event) = window.next() {
            //touch_visualizer.event(window.size(), &event);

            if let Some(press_args) = event.press_args() {
                current_move = match press_args {
                    Button::Keyboard(Key::Up) => Move::Up,
                    Button::Keyboard(Key::Down) => Move::Down,
                    Button::Keyboard(Key::Left) => Move::Left,
                    Button::Keyboard(Key::Right) => Move::Right,
                    _ => Move::None
                }
            }

            window.draw_2d(&event, |context, graphics, _device| {

                clear([1.0; 4], graphics);

                rectangle([0.0, 0.0, 0.0, 1.0],[175.0, 10.0, 515.0, 5.0], context.transform, graphics);
                rectangle([0.0, 0.0, 0.0, 1.0],[10.0, 175.0, 680.0, 5.0], context.transform, graphics);
                rectangle([0.0, 0.0, 0.0, 1.0],[10.0, 430.0, 680.0, 5.0], context.transform, graphics);
                rectangle([0.0, 0.0, 0.0, 1.0],[10.0, 685.0, 680.0, 5.0], context.transform, graphics);
        
                rectangle([0.0, 0.0, 0.0, 1.0],[10.0, 175.0, 5.0, 515.0], context.transform, graphics);
                rectangle([0.0, 0.0, 0.0, 1.0],[175.0, 10.0, 5.0, 680.0], context.transform, graphics);
                rectangle([0.0, 0.0, 0.0, 1.0],[430.0, 10.0, 5.0, 680.0], context.transform, graphics);
                rectangle([0.0, 0.0, 0.0, 1.0],[685.0, 10.0, 5.0, 680.0], context.transform, graphics);
        
                let mut pos = 181.0;
                for _ in 0..4 {
                    pos += 48.0;
                    rectangle([0.0, 0.0, 0.0, 1.0],[10.0, pos, 680.0, 2.0], context.transform, graphics);
                    pos += 2.0;
                }
        
                pos = 436.0;
                for _ in 0..4 {
                    pos += 48.0;
                    rectangle([0.0, 0.0, 0.0, 1.0],[10.0, pos, 680.0, 2.0], context.transform, graphics);
                    pos += 2.0;
                }
                
                pos = 181.0;
                for _ in 0..4 {
                    pos += 48.0;
                    rectangle([0.0, 0.0, 0.0, 1.0],[pos, 10.0, 2.0, 680.0], context.transform, graphics);
                    pos += 2.0;
                }
        
                pos = 436.0;
                for _ in 0..4 {
                    pos += 48.0;
                    rectangle([0.0, 0.0, 0.0, 1.0],[pos, 10.0, 2.0, 680.0], context.transform, graphics);
                pos += 2.0;
                }


                let mut y: f64 = 215.0;

                for i in 0..map.horizontal_vec.len() {
                    let mut x: f64 = 145.0;
                    let g = map.horizontal_vec[i].len();
                    if i == 5 {
                        y += 5.0;
                    }
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
                    y += 50.0;
                }

                let mut x: f64 = 201.0;

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
                    x += 50.0;
                }

                //println!("Map  | {:?}", map.map);
                //println!("Hori | {:?}", map.horizontal_vec);
                //println!("Vert | {:?}", map.vertical_vec);
            });
        }
    }
}