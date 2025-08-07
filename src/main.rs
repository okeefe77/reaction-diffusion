mod reaction;
mod display;

use crate::reaction::Reaction;
use crate::display::ColorBuffer;

use nannou::prelude::*;
use nannou::image::{ Rgba, RgbaImage };
use nannou::wgpu::{ Texture, TextureUsages };



struct Model {
    window: WindowId,
    reaction: Reaction,
    buffer: ColorBuffer
}

fn model(app: &App) -> Model {
    let width = 400;
    let height = 300;

    let window = app.new_window()
        .size(width, height)
        .title("Reaction Diffusion")
        .view(view)
        .build()
        .unwrap();

    let mut reaction = Reaction::new(width, height, 1.0, 0.5, 0.055, 0.062);
    reaction.seed(width / 2, height / 2, 5);

    let buffer = ColorBuffer::new(width, height);

    Model { window, reaction, buffer }
}


fn view(app: &App, model: &Model, frame: Frame) {
    let img = RgbaImage::from_vec(
        model.buffer.width(),
        model.buffer.height(),
        model.buffer.data()
    ).unwrap();

    let texture = Texture::load_from_image_buffer(
        frame.device_queue_pair().device(),
        frame.device_queue_pair().queue(),
        TextureUsages::TEXTURE_BINDING,
        &img
    );

    let draw = app.draw();
    draw.texture(&texture);
    draw.to_frame(app, &frame).unwrap();
}


fn update(_app: &App, model: &mut Model, _update: Update) {
    model.reaction.step();

    model.buffer.clear(Rgba([255; 4]));
    for ((x, y), b_concentration) in model.reaction.grid_data() {
        let g = 255 - (255.0 * b_concentration).floor() as u8; 
        model.buffer.set_pixel(x, y, Rgba([g, g, g, 255])); 
    }
}


fn main() {
    nannou::app(model).update(update).run();
}
