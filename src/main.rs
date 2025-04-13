#![allow(unused)]
mod movement;
mod torus;
mod examples;

use std::{f32::consts::PI, fmt::DebugList, vec};

use macroquad::prelude::*;
use movement::handle_input;
use torus::Torus;

const BACKGROUND_COLOR: Color = color_u8!(0x18, 0x18, 0x18, 0xFF);

fn rotate_point_around_x_axis(point: &mut Vec3, rotation_angle: f32) {
    let old_y = point.y;
    let old_z = point.z;

    let new_y = old_y * f32::cos(rotation_angle) + old_z * f32::sin(rotation_angle);
    let new_z = -1.0 * old_y * f32::sin(rotation_angle) + old_z * f32::cos(rotation_angle);

    point.y = new_y;
    point.z = new_z;
}

fn rotate_point_around_y_axis(point: &mut Vec3, rotation_angle: f32) {
    let old_x = point.x;
    let old_z = point.z;

    let new_x = old_x * f32::cos(rotation_angle) - old_z * f32::sin(rotation_angle);
    let new_z = old_x * f32::sin(rotation_angle) + old_z * f32::cos(rotation_angle);

    point.x = new_x;
    point.z = new_z;
}

fn main_conf() -> Conf {
    Conf {
        fullscreen: true,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    let mut camera = Camera3D {
        position: vec3(0.0, 1.5, 10.0),
        target: vec3(0.0, 0.0, 0.0),
        up: vec3(0.0, 1.0, 0.0),
        ..Default::default()
    };

    let mut yaw: f32 = 270.0; // Initial horizontal rotation (in degrees)
    let mut pitch: f32 = -10.0; // Initial vertical rotation (in degrees)
    let movement_speed = 0.1;
    let rotation_speed = 1.0;

    let mut torus_rotation_angle = 0.0;

    loop {

        clear_background(BACKGROUND_COLOR);

        handle_input(&mut camera, &mut pitch, &mut yaw, movement_speed, rotation_speed);

        set_camera(&camera);

        let mut vertices = Vec::<Vertex>::new();

        let torus = Torus { r: 0.5, R: 2.0 };
        let mut phi = 0.0;

        let major_axis_resolution: usize = 100;
        let minor_axis_resolution: usize = 20;
        let delta_phi = 2.0 * PI / major_axis_resolution as f32;

        for i in 0..major_axis_resolution {
            for j in 0..minor_axis_resolution {
                let theta = (j as f32) * 2.0 * PI / minor_axis_resolution as f32;

                let r = if i < major_axis_resolution / 2 {
                    ((510.0 / major_axis_resolution as f32) * (i as f32)) as u8
                } else {
                    ((-510.0 / major_axis_resolution as f32) * (i as f32) + 510.0) as u8
                };

                vertices.push(Vertex {
                    position: torus.get_point(theta, phi),
                    uv: vec2(0.5, 1.0),
                    color: [r, 0, 255, 255],
                    normal: vec4(0.0, 0.0, 0.0, 0.0),
                });
            }
            
            phi += delta_phi;
        }

        torus_rotation_angle += 2.0 * PI * (get_frame_time() / 10.0);
        while torus_rotation_angle > 2.0 * PI {
            torus_rotation_angle -= 2.0 * PI;
        }
        for v in &mut vertices {
            rotate_point_around_x_axis(&mut v.position, torus_rotation_angle);
            // rotate_point_around_y_axis(&mut v.position, torus_rotation_angle);
        }

        let mut indices = Vec::<u16>::new();
        for i in 0..major_axis_resolution as u16 {
            let starting_idx = i * minor_axis_resolution as u16;
            let next_starting_idx = ((i + 1) % major_axis_resolution as u16) * minor_axis_resolution as u16;

            for j in 0..minor_axis_resolution as u16 {
                if j == minor_axis_resolution as u16 - 1 {
                    indices.push(starting_idx + j);
                    indices.push(starting_idx);
                    indices.push(next_starting_idx + j);

                    indices.push(starting_idx);
                    indices.push(next_starting_idx + j);
                    indices.push(next_starting_idx);
                } else {
                    indices.push(starting_idx + j);
                    indices.push(starting_idx + j + 1);
                    indices.push(next_starting_idx + j);

                    indices.push(starting_idx + j + 1);
                    indices.push(next_starting_idx + j);
                    indices.push(next_starting_idx + j + 1);
                }
            }
        }

        // max vertices and indices
        // draw_call_vertex_capacity: 10000
        // draw_call_index_capacity: 5000
        // so should do 5000 indices at a time
        for i in 0..((indices.len() / 3000) + 1) {
            draw_mesh(&Mesh {
                vertices: vertices.clone(),
                indices: indices[
                    (i * 3000)..usize::min((i + 1) * 3000, indices.len())
                ].to_vec(),
                texture: None
            });
        }

        // set_default_camera();
        // draw_fps();

        next_frame().await;
    }
}