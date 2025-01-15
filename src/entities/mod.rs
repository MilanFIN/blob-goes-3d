pub mod entity;
use entity::*;

pub mod cube;
use cube::*;

pub mod rectangle;
use rectangle::*;

pub mod empty;
use empty::*;

pub mod mover;
use mover::*;

pub mod crumbling;
use crumbling::*;

pub mod finish;
use finish::*;

pub mod switch;
use switch::*;

pub mod wireframe;
use wireframe::*;

pub mod boundingshapes;
use boundingshapes::*;

pub mod utils;

use serde::Deserialize;

use super::math;

use super::camera;
use camera::*;

use crate::effects;
use crate::fixed;
use fixed::*;

#[derive(Copy, Clone, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum EntityEnum {
    #[serde(rename = "cube")]
    Cube(Cube),
    #[serde(rename = "rectangle")]
    Rectangle(Rectangle),
    #[serde(rename = "mover")]
    Mover(Mover),
    #[serde(rename = "crumbling")]
    Crumbling(Crumbling),
    #[serde(rename = "finish")]
    Finish(Finish),
    #[serde(rename = "switch")]
    Switch(Switch),
    #[serde(rename = "wireframe")]
    Wireframe(Wireframe),
    #[serde(rename = "empty")]
    Empty(Empty),
}

impl EntityEnum {
    pub fn set_x_offset(&mut self, offset: Fixed) {
        match self {
            EntityEnum::Cube(a) => a.set_x_offset(offset),
            EntityEnum::Rectangle(a) => a.set_x_offset(offset),
            EntityEnum::Mover(a) => a.set_x_offset(offset),
            EntityEnum::Crumbling(a) => a.set_x_offset(offset),
            EntityEnum::Finish(a) => a.set_x_offset(offset),
            EntityEnum::Switch(a) => a.set_x_offset(offset),
            EntityEnum::Wireframe(a) => a.set_x_offset(offset),
            EntityEnum::Empty(_a) => {}
        }
    }
    pub fn set_y_offset(&mut self, offset: Fixed) {
        match self {
            EntityEnum::Cube(a) => a.set_y_offset(offset),
            EntityEnum::Rectangle(a) => a.set_y_offset(offset),
            EntityEnum::Mover(a) => a.set_y_offset(offset),
            EntityEnum::Crumbling(a) => a.set_y_offset(offset),
            EntityEnum::Finish(a) => a.set_y_offset(offset),
            EntityEnum::Switch(a) => a.set_y_offset(offset),
            EntityEnum::Wireframe(a) => a.set_y_offset(offset),
            EntityEnum::Empty(_a) => {}
        }
    }
    pub fn set_z_offset(&mut self, offset: Fixed) {
        match self {
            EntityEnum::Cube(a) => a.set_z_offset(offset),
            EntityEnum::Rectangle(a) => a.set_z_offset(offset),
            EntityEnum::Mover(a) => a.set_z_offset(offset),
            EntityEnum::Crumbling(a) => a.set_z_offset(offset),
            EntityEnum::Finish(a) => a.set_z_offset(offset),
            EntityEnum::Switch(a) => a.set_z_offset(offset),
            EntityEnum::Wireframe(a) => a.set_z_offset(offset),
            EntityEnum::Empty(_a) => {}
        }
    }
    pub fn set_x_rotation(&mut self, rot: Fixed) {
        match self {
            EntityEnum::Cube(a) => a.set_x_rotation(rot),
            EntityEnum::Rectangle(a) => a.set_x_rotation(rot),
            EntityEnum::Mover(a) => a.set_x_rotation(rot),
            EntityEnum::Crumbling(a) => a.set_x_rotation(rot),
            EntityEnum::Finish(a) => a.set_x_rotation(rot),
            EntityEnum::Switch(a) => a.set_x_rotation(rot),
            EntityEnum::Wireframe(a) => a.set_x_rotation(rot),
            EntityEnum::Empty(_a) => {}
        }
    }
    pub fn set_y_rotation(&mut self, rot: Fixed) {
        match self {
            EntityEnum::Cube(a) => a.set_y_rotation(rot),
            EntityEnum::Rectangle(a) => a.set_y_rotation(rot),
            EntityEnum::Mover(a) => a.set_y_rotation(rot),
            EntityEnum::Crumbling(a) => a.set_y_rotation(rot),
            EntityEnum::Finish(a) => a.set_y_rotation(rot),
            EntityEnum::Switch(a) => a.set_y_rotation(rot),
            EntityEnum::Wireframe(a) => a.set_y_rotation(rot),
            EntityEnum::Empty(_a) => {}
        }
    }
    pub fn set_z_rotation(&mut self, rot: Fixed) {
        match self {
            EntityEnum::Cube(a) => a.set_z_rotation(rot),
            EntityEnum::Rectangle(a) => a.set_z_rotation(rot),
            EntityEnum::Mover(a) => a.set_z_rotation(rot),
            EntityEnum::Crumbling(a) => a.set_z_rotation(rot),
            EntityEnum::Finish(a) => a.set_z_rotation(rot),
            EntityEnum::Switch(a) => a.set_z_rotation(rot),
            EntityEnum::Wireframe(a) => a.set_z_rotation(rot),
            EntityEnum::Empty(_a) => {}
        }
    }
    pub fn reload_rotation_matrices(&mut self) {
        match self {
            EntityEnum::Cube(a) => a.reload_rotation_matrices(),
            EntityEnum::Rectangle(a) => a.reload_rotation_matrices(),
            EntityEnum::Mover(a) => a.reload_rotation_matrices(),
            EntityEnum::Crumbling(a) => a.reload_rotation_matrices(),
            EntityEnum::Finish(a) => a.reload_rotation_matrices(),
            EntityEnum::Switch(a) => a.reload_rotation_matrices(),
            EntityEnum::Wireframe(a) => a.reload_rotation_matrices(),
            EntityEnum::Empty(_a) => {}
        }
    }
    pub fn refresh_model_matrix(&mut self) {
        match self {
            EntityEnum::Cube(a) => a.refresh_model_matrix(),
            EntityEnum::Rectangle(a) => a.refresh_model_matrix(),
            EntityEnum::Mover(a) => a.refresh_model_matrix(),
            EntityEnum::Crumbling(a) => a.refresh_model_matrix(),
            EntityEnum::Finish(a) => a.refresh_model_matrix(),
            EntityEnum::Switch(a) => a.refresh_model_matrix(),
            EntityEnum::Wireframe(a) => a.refresh_model_matrix(),
            EntityEnum::Empty(_a) => {}
        }
    }
    //todo: rename to set_scale at some point to recalculate points at a different scale from original size
    pub fn set_size(&mut self, size: Fixed) {
        match self {
            EntityEnum::Cube(a) => a.set_size(size),
            //doesn't implement this
            EntityEnum::Rectangle(_a) => {}
            EntityEnum::Mover(_a) => {}
            EntityEnum::Crumbling(_a) => {}
            EntityEnum::Finish(_a) => {}
            EntityEnum::Switch(_a) => {}
            EntityEnum::Wireframe(_a) => {}
            EntityEnum::Empty(_a) => {}
        }
    }
    pub fn recalculate_points(&mut self) {
        match self {
            EntityEnum::Cube(a) => a.recalculate_points(),
            EntityEnum::Rectangle(a) => a.recalculate_points(),
            EntityEnum::Mover(a) => a.recalculate_points(),
            EntityEnum::Crumbling(a) => a.recalculate_points(),
            EntityEnum::Finish(a) => a.recalculate_points(),
            EntityEnum::Switch(a) => a.recalculate_points(),
            EntityEnum::Wireframe(a) => a.recalculate_points(),
            EntityEnum::Empty(_a) => {}
        }
    }
    #[allow(dead_code)]
    pub fn set_vertex(&mut self, point: [Fixed; 3], index: i32) {
        match self {
            EntityEnum::Cube(a) => a.set_vertex(point, index),
            EntityEnum::Rectangle(a) => a.set_vertex(point, index),
            EntityEnum::Mover(a) => a.set_vertex(point, index),
            EntityEnum::Crumbling(a) => a.set_vertex(point, index),
            EntityEnum::Finish(a) => a.set_vertex(point, index),
            EntityEnum::Switch(a) => a.set_vertex(point, index),
            EntityEnum::Wireframe(a) => a.set_vertex(point, index),
            EntityEnum::Empty(_a) => {}
        }
    }
    pub fn render(&mut self, camera: &Camera, page: u16) {
        match self {
            EntityEnum::Cube(a) => a.render(camera, page),
            EntityEnum::Rectangle(a) => a.render(camera, page),
            EntityEnum::Mover(a) => a.render(camera, page),
            EntityEnum::Crumbling(a) => a.render(camera, page),
            EntityEnum::Empty(_a) => {}
            EntityEnum::Finish(a) => a.render(camera, page),
            EntityEnum::Switch(a) => a.render(camera, page),
            EntityEnum::Wireframe(a) => a.render(camera, page),
        }
    }
    pub fn distance_from_camera(&self, camera: &Camera) -> Fixed {
        match self {
            EntityEnum::Cube(a) => a.distance_from_camera(camera),
            EntityEnum::Rectangle(a) => a.distance_from_camera(camera),
            EntityEnum::Mover(a) => a.distance_from_camera(camera),
            EntityEnum::Crumbling(a) => a.distance_from_camera(camera),
            EntityEnum::Finish(a) => a.distance_from_camera(camera),
            EntityEnum::Switch(a) => a.distance_from_camera(camera),
            EntityEnum::Wireframe(a) => a.distance_from_camera(camera),
            EntityEnum::Empty(_a) => Fixed::const_new(999),
        }
    }
    pub fn bounding_box(&self) -> BoundingBox {
        match self {
            EntityEnum::Cube(a) => a.bounding_box(),
            EntityEnum::Rectangle(a) => a.bounding_box(),
            EntityEnum::Mover(a) => a.bounding_box(),
            EntityEnum::Crumbling(a) => a.bounding_box(),
            EntityEnum::Finish(a) => a.bounding_box(),
            EntityEnum::Switch(a) => a.bounding_box(),
            EntityEnum::Wireframe(a) => a.bounding_box(),
            EntityEnum::Empty(_a) => BoundingBox::default(),
        }
    }
    pub fn bounding_cylinder(&self) -> BoundingCylinder {
        match self {
            EntityEnum::Cube(a) => a.bounding_cylinder(),
            EntityEnum::Rectangle(a) => a.bounding_cylinder(),
            EntityEnum::Mover(a) => a.bounding_cylinder(),
            EntityEnum::Crumbling(a) => a.bounding_cylinder(),
            EntityEnum::Finish(a) => a.bounding_cylinder(),
            EntityEnum::Switch(a) => a.bounding_cylinder(),
            EntityEnum::Wireframe(a) => a.bounding_cylinder(),
            EntityEnum::Empty(_a) => BoundingCylinder::default(),
        }
    }
    pub fn get_y(&self) -> Fixed {
        match self {
            EntityEnum::Cube(a) => a.get_y(),
            EntityEnum::Rectangle(a) => a.get_y(),
            EntityEnum::Mover(a) => a.get_y(),
            EntityEnum::Crumbling(a) => a.get_y(),
            EntityEnum::Finish(a) => a.get_y(),
            EntityEnum::Switch(a) => a.get_y(),
            EntityEnum::Wireframe(a) => a.get_y(),
            EntityEnum::Empty(_a) => Fixed::const_new(-999),
        }
    }
    pub fn set_color(&mut self, color: u16) {
        match self {
            EntityEnum::Cube(a) => a.set_color(color),
            EntityEnum::Rectangle(a) => a.set_color(color),
            EntityEnum::Mover(a) => a.set_color(color),
            EntityEnum::Crumbling(a) => a.set_color(color),
            EntityEnum::Finish(a) => a.set_color(color),
            EntityEnum::Switch(a) => a.set_color(color),
            EntityEnum::Wireframe(a) => a.set_color(color),
            EntityEnum::Empty(_a) => {}
        }
    }

    pub fn tick(
        &mut self,
        effects: &effects::InputPlayerEffects,
    ) -> Option<effects::OutputPlayerEffects> {
        match self {
            EntityEnum::Cube(a) => a.tick(effects),
            EntityEnum::Rectangle(a) => a.tick(effects),
            EntityEnum::Mover(a) => a.tick(effects),
            EntityEnum::Crumbling(a) => a.tick(effects),
            EntityEnum::Finish(a) => a.tick(effects),
            EntityEnum::Switch(a) => a.tick(effects),
            EntityEnum::Wireframe(a) => a.tick(effects),
            EntityEnum::Empty(_a) => None,
        }
    }
    pub fn get_id(&self) -> i16 {
        match self {
            EntityEnum::Cube(a) => a.get_id(),
            EntityEnum::Rectangle(a) => a.get_id(),
            EntityEnum::Mover(a) => a.get_id(),
            EntityEnum::Crumbling(a) => a.get_id(),
            EntityEnum::Finish(a) => a.get_id(),
            EntityEnum::Switch(a) => a.get_id(),
            EntityEnum::Wireframe(a) => a.get_id(),
            EntityEnum::Empty(_a) => -1,
        }
    }
    pub fn set_id(&mut self, id: i16) {
        match self {
            EntityEnum::Cube(a) => a.set_id(id),
            EntityEnum::Rectangle(a) => a.set_id(id),
            EntityEnum::Mover(a) => a.set_id(id),
            EntityEnum::Crumbling(a) => a.set_id(id),
            EntityEnum::Finish(a) => a.set_id(id),
            EntityEnum::Switch(a) => a.set_id(id),
            EntityEnum::Wireframe(a) => a.set_id(id),
            EntityEnum::Empty(_a) => {}
        }
    }
}
