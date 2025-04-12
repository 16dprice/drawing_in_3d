use macroquad::prelude::*;

pub fn handle_input(
    camera: &mut Camera3D,
    pitch: &mut f32,
    yaw: &mut f32,
    movement_speed: f32,
    rotation_speed: f32,
) {
    // Update camera rotation
    if is_key_down(KeyCode::Left) {
        *yaw -= rotation_speed;
    }
    if is_key_down(KeyCode::Right) {
        *yaw += rotation_speed;
    }
    if is_key_down(KeyCode::Up) {
        *pitch += rotation_speed;
    }
    if is_key_down(KeyCode::Down) {
        *pitch -= rotation_speed;
    }

    // Clamp pitch to prevent camera flipping
    *pitch = pitch.clamp(-89.0, 89.0);
    
    // Calculate forward direction based on yaw and pitch
    let yaw_rad = yaw.to_radians();
    let pitch_rad = pitch.to_radians();
    let forward = vec3(
        yaw_rad.cos() * pitch_rad.cos(),
        pitch_rad.sin(),
        yaw_rad.sin() * pitch_rad.cos(),
    ).normalize();
    
    // Update camera target based on position and forward direction
    camera.target = camera.position + forward;

    // Handle movement (WASD)
    let right = vec3(forward.z, 0.0, -forward.x).normalize();
    
    if is_key_down(KeyCode::W) {
        // Move forward along the camera's forward direction (excluding y component for true FPS movement)
        let forward_xz = vec3(forward.x, 0.0, forward.z).normalize();
        camera.position += forward_xz * movement_speed;
    }
    if is_key_down(KeyCode::S) {
        // Move backward
        let forward_xz = vec3(forward.x, 0.0, forward.z).normalize();
        camera.position -= forward_xz * movement_speed;
    }
    if is_key_down(KeyCode::A) {
        camera.position += right * movement_speed;
    }
    if is_key_down(KeyCode::D) {
        camera.position -= right * movement_speed;
    }
}