use macroquad::prelude::*;

pub async fn draw_basic_triangles() {
    loop {
        clear_background(LIGHTGRAY);

        let vertices = vec![
            Vertex {
                position: vec3(-1.0, 0.0, 0.0),
                uv: vec2(0.0, 0.0),
                color: [255, 0, 255, 255],   // Purple
                normal: vec4(0.0, 0.0, 0.0, 0.0),
            },
            Vertex {
                position: vec3(1.0, 0.0, 0.0),
                uv: vec2(1.0, 0.0),
                color: [0, 255, 0, 255],   // Green
                normal: vec4(0.0, 0.0, 0.0, 0.0),
            },
            Vertex {
                position: vec3(0.0, 1.0, 0.0),
                uv: vec2(0.5, 1.0),
                color: [0, 0, 255, 255],   // Blue
                normal: vec4(0.0, 0.0, 0.0, 0.0),
            },
            Vertex {
                position: vec3(1.0, 1.0, -1.0),
                uv: vec2(0.5, 1.0),
                color: [255, 0, 0, 255],   // Red
                normal: vec4(0.0, 0.0, 0.0, 0.0),
            },
        ];
        let indices = vec![0, 1, 2, 1, 2, 3];

        let mesh = Mesh {
            vertices,
            indices,
            texture: None,
        };
        draw_mesh(&mesh);

        next_frame().await;
    }
}