use macroquad::prelude::*;

use crate::{figure::Figure, SIZE};

#[derive(Debug, Default, Clone, Copy)]
pub struct Field {
    pub figure: Option<Figure>,
    pub idxs: (usize, usize),
    pub x: f32,
    pub y: f32,
}

impl Field {
    pub fn draw_sprite(&self, sprite: Texture2D) {
        draw_texture(sprite, self.x - 3., self.y - 2., WHITE);
    }

    pub fn draw(&self, field_color: Color, sprites: &[Texture2D; 12]) {
        draw_rectangle(self.x, self.y, SIZE, SIZE, field_color);

        if let Some(figure) = self.figure {
            self.draw_sprite(sprites[figure.figure as usize + figure.team as usize * 6]);
        }
    }
}