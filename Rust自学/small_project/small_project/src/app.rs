use eframe::*;
use rand::Rng;
use std::{time, thread};


pub enum Direction {
    Up,
    Left,
    Right,
    Down,
}


pub struct MyEguiApp {
    game_start: bool,
    game_over: bool,
    points: u32,
    head_pos: egui::Rect,
    size: f32,
    head_color: egui::Color32,
    body_color: egui::Color32,
    init_len: u32,
    body_pos_vec: Vec<egui::Rect>,
    direction: Direction,
}

impl MyEguiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {

        let size_f = 20.0;
        let size_i = size_f as i32;
        let init_len = 3;
        let head_min: egui::Pos2 = egui::Pos2{
            x: (rand::thread_rng().gen_range(5..=25) * size_i) as f32, 
            y: (rand::thread_rng().gen_range(5..=25) * size_i) as f32,
        };
        let head_max: egui::Pos2 = egui::Pos2{x: head_min.x + size_f, y: head_min.y + size_f };
        let head = egui::Rect{min: head_min, max: head_max};

        // Init the direction to be always up
        let direction = Direction::Up; 

        // Init the body of snake
        let mut body_pos_vec = vec![];
        for i in 1..(init_len + 1){
            let i_float = i as f32;
            body_pos_vec.push(
                egui::Rect{
                    min: egui::Pos2 {
                        x: head_min.x,
                        y: head_min.y + size_f * i_float,
                    },
                    max: egui::Pos2 {
                        x: head_max.x,
                        y: head_max.y + size_f * i_float,
                    }
                }
            );
        }

        MyEguiApp { 
            game_start: false,
            game_over: false,
            points: 0,
            head_pos: head,
            size: size_f,
            head_color: egui::Color32::GOLD,
            body_color: egui::Color32::BLUE,
            body_pos_vec: body_pos_vec,
            init_len: init_len,
            direction: direction,
        }
    }
}


fn paint_rect(app_status: &mut MyEguiApp, ui: &mut egui::Ui, body_block_i: usize, paint_head: bool){

    let where_to_put_background = ui.painter().add(egui::Shape::Noop);
    let style = ui.visuals().widgets.inactive;
    let mut fill = app_status.body_color;
    let mut pos = app_status.body_pos_vec[body_block_i];
    if paint_head {
        fill = app_status.head_color;
        pos = app_status.head_pos;
    } 
    let stroke = style.bg_stroke;
    ui.painter().set(
        where_to_put_background, 
        epaint::RectShape {
            rounding: style.rounding,
            fill,
            stroke,
            rect: pos,
        }
    );
}


impl eframe::App for MyEguiApp {

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // If the game is over, print out the message
            if self.game_over {
                ui.heading(format!("Game over with points: {}", self.points));
                return
            }

            // if the game is not start yet, wait for start and provide a button to start for player
            if !self.game_start{
                ui.heading("Welcome to Snake Game!");
                if ui.button("Start!").clicked() {
                    self.game_start = true;
                } else {
                    return
                }
            } 

            // main logic when game starts
            else{
                ui.heading("Snake Game.");
                ui.heading(format!("Points: {}", self.points));

                // paint the head 
                paint_rect(self, ui, 0, true);

                // paint the body
                for body_block_i in 0..self.body_pos_vec.len() {
                    paint_rect(self, ui, body_block_i, false);
                }
            }

            // update the movement of snake
            // 1. when the snake move upward
            match self.direction {
                Direction::Up => {
                    self.head_pos.min.y -= self.size;
                    self.head_pos.max.y -= self.size;

                    for body_block_i in 0..self.body_pos_vec.len() {
                        self.body_pos_vec[body_block_i].min.y -= self.size;
                        self.body_pos_vec[body_block_i].max.y -= self.size;
                    }
                },
                _ => (),
            }

            // update the time
            let one_second = time::Duration::new(1, 0);
            thread::sleep(one_second);
        });
    }
}