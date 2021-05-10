use nannou::{color};
use nannou::prelude::*;
use nalgebra as na;
use na::{Matrix6};
use std::collections::HashSet;
use std::time::Duration;

// num nodes to generate positions for
const NODES: u32 = 6;
const BOX_PADDING: f32 = 25.0;
const NODE_RADIUS: u32 = 5;
// number of seconds to delay between model updates
const DELAY_SECONDS: u64 = 3;

#[derive(Debug)]
struct Model {
    node_locations: Vec<Point2>,
    adjacency_matrix: Matrix6<f32>,
    infected: HashSet<u32>,
    generation: usize,
    time_since_last: std::time::Duration
}

fn main() {
    nannou::app(model).simple_window(view).update(update).run();
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::RefreshSync);
    Model {
        node_locations: get_random_node_points(NODES),
        adjacency_matrix: Matrix6::new(0.0, 1.0, 0.0, 0.0,  0.0,  0.0,
                                    1.0, 0.0, 1.0,  0.0,  0.0, 0.0,
                                    0.0, 1.0, 0.0,  1.0,  0.0,  0.0,
                                    0.0, 0.0, 1.0,  0.0,  0.0,  0.0,
                                    0.0, 0.0, 0.0,  0.0,  0.0,  0.0,
                                    0.0, 0.0, 0.0,  0.0,  0.0,  0.0),
        infected: {
            let mut s = HashSet::new();
            s.insert(0);
            s
        },
        generation: 0,
        time_since_last: Duration::new(0, 0)
    }
}

fn update(_app: &App, _model: &mut Model, update: Update) {

    if update.since_last.as_secs() < DELAY_SECONDS && _model.time_since_last.as_secs() < DELAY_SECONDS
    {
        _model.time_since_last += update.since_last;
        return;
    }
    _model.time_since_last = Duration::new(0, 0);
    
    let mut temp_queue: HashSet<u32> = HashSet::new();
    for i in 0..(NODES-1) {
        for j in (i+1)..(NODES) {
            if _model.adjacency_matrix[(i as usize, j as usize)] > 0.0
            {
                if _model.infected.contains(&i) || _model.infected.contains(&j)
                {
                    temp_queue.insert(i);
                    temp_queue.insert(j);
                }
            }
        }
    }
    _model.infected.extend(&temp_queue);
    _model.generation += 1;
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();

    draw.background().color(BLACK);
    draw
        .text(format!("generation {}", _model.generation).as_str())
        .h(win.h())
        .w(win.w())
        .align_text_top()
        .left_justify();
    let graph_rect = Rect::from_w_h(
        win.w() - BOX_PADDING,
        win.h() - BOX_PADDING
    );
    
    for (i, node) in _model.node_locations.iter().enumerate()
    {
        let color = {
            match _model.infected.contains(&(i as u32))
            {
                true => color::rgb(1.0, 0.0, 0.0),
                false => color::rgb(0.0, 1.0, 0.0)  
            }
        };

        draw.ellipse()
            .x_y(node.x * graph_rect.w()/2.0, node.y * graph_rect.h()/2.0)
            .radius(NODE_RADIUS as f32)
            .color(color);
    }    

    let _ = draw.to_frame(app, &frame);
}

fn get_random_node_points(n: u32) -> Vec<Point2<f32>> {
    (0..n)
        .map(|_| {
            let coord = || {
                let v: f32 = random();
                let sign: bool = random();
                if sign { v } else { -v }
            };
            pt2(coord(), coord())
        })
        .collect()
}