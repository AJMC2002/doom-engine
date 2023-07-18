use doom_engine::graphics::{wrapper::*, Window};
use doom_engine::vector;
use egui_glfw_gl::egui;
use egui_glfw_gl::egui::{vec2, Pos2, Rect};
use egui_glfw_gl::gl;
use egui_glfw_gl::gl::types::*;
use std::{mem, os::raw::c_void, ptr};

static WIDTH: u32 = 1080;

static HEIGHT: u32 = 720;

fn main() {
    let mut window = Window::new(WIDTH, HEIGHT, "Doom Engine");

    let mut painter = egui_glfw_gl::Painter::new(window.window_handle_mut(), WIDTH, HEIGHT);
    let mut egui_ctx = egui::CtxRef::default();

    let (width, height) = window.window_handle().get_framebuffer_size();
    let native_pixels_per_point = window.window_handle().get_content_scale().0;
    let mut egui_input_state = egui_glfw_gl::EguiInputState::new(egui::RawInput {
        screen_rect: Some(Rect::from_min_size(
            Pos2::new(0f32, 0f32),
            vec2(width as f32, height as f32) / native_pixels_per_point,
        )),
        pixels_per_point: Some(native_pixels_per_point),
        ..Default::default()
    });

    let mut shader_program = ShaderProgram::new(
        "resources/shaders/basic/basic.vs",
        "resources/shaders/basic/basic.fs",
    );

    let texture = Texture2D::new("resources/textures/cat.jpg");

    // All Buffer Objects are binded and the data is stored on creation
    let _vao = VAO::new();
    let _vbo: VBO = BO::new(
        gl::STATIC_DRAW,
        Box::new([
            // positions [3] // tex [2]
            -0.5, 0.5, 0.0, 0.0, 1.0, // top right
            0.5, 0.5, 0.0, 1., 1., // bottom right
            -0.5, -0.5, 0.0, 0., 0., // bottom left
            0.5, -0.5, 0.0, 1., 0., // top left
        ]),
    );
    let _ebo: EBO = BO::new(
        gl::STATIC_DRAW,
        Box::new([
            2, 1, 0, // first Triangle
            3, 2, 1, // second Triangle
        ]),
    );

    let stride = 5 * mem::size_of::<GLfloat>() as GLsizei;

    let _pos_attrib = VertexAttrib::new(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
    let _tex_attrib = VertexAttrib::new(
        1,
        2,
        gl::FLOAT,
        gl::FALSE,
        stride,
        (3 * mem::size_of::<GLfloat>()) as *const c_void,
    );

    let mut camera_position = (
        0.0.to_string().to_owned(),
        0.0.to_string().to_owned(),
        0.0.to_string().to_owned(),
    );
    let mut camera_up = (
        0.0.to_string().to_owned(),
        0.0.to_string().to_owned(),
        0.0.to_string().to_owned(),
    );
    let mut camera_center = (
        0.0.to_string().to_owned(),
        0.0.to_string().to_owned(),
        0.0.to_string().to_owned(),
    );

    while !window.window_handle().should_close() {
        let mut camera_update = false;
        let mut camera_reset = false;
        let (width, height) = window.window_handle().get_size();
        let native_pixels_per_point = window.window_handle().get_content_scale().0;

        egui_input_state.input.time = Some(window.glfw_handle().get_time());
        egui_ctx.begin_frame(egui_input_state.input.take());
        egui_input_state.input.screen_rect = Some(Rect::from_min_size(
            Pos2::new(0f32, 0f32),
            vec2(width as f32, height as f32) / native_pixels_per_point,
        ));
        egui_input_state.input.pixels_per_point = Some(native_pixels_per_point);

        unsafe {
            gl::ClearColor(154. / 258., 127. / 258., 174. / 258., 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            let t = window.glfw_handle().get_time() as f32;
            let color = (t.sin() / 2.0) + 0.5;

            _vao.bind();
            _vbo.bind();
            _ebo.bind();
            shader_program.bind();
            _pos_attrib.enable();
            _tex_attrib.enable();
            texture.bind();
            shader_program.uniform_4f(
                "globalColor",
                vector![1.0 - color, color, color.powi(2), 1.0],
            );
            shader_program.uniform_2dtex("myTex", &texture);

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }

        egui::Window::new("Control").show(&egui_ctx, |ui| {
            ui.set_max_width(280.0);
            ui.group(|ui| {
                ui.label("camera");
                ui.horizontal(|ui| {
                    ui.set_max_width(250.0);
                    ui.label("position");
                    ui.label("x:");
                    ui.add(egui::TextEdit::singleline(&mut camera_position.0).desired_width(30.0));
                    ui.label("y:");
                    ui.add(egui::TextEdit::singleline(&mut camera_position.1).desired_width(30.0));
                    ui.label("z:");
                    ui.add(egui::TextEdit::singleline(&mut camera_position.2).desired_width(30.0));
                });
                ui.horizontal(|ui| {
                    ui.set_max_width(250.0);
                    ui.label("center");
                    ui.label("x:");
                    ui.add(egui::TextEdit::singleline(&mut camera_center.0).desired_width(30.0));
                    ui.label("y:");
                    ui.add(egui::TextEdit::singleline(&mut camera_center.1).desired_width(30.0));
                    ui.label("z:");
                    ui.add(egui::TextEdit::singleline(&mut camera_center.2).desired_width(30.0));
                });
                ui.horizontal(|ui| {
                    ui.set_max_width(250.0);
                    ui.label("up");
                    ui.label("x:");
                    ui.add(egui::TextEdit::singleline(&mut camera_up.0).desired_width(30.0));
                    ui.label("y:");
                    ui.add(egui::TextEdit::singleline(&mut camera_up.1).desired_width(30.0));
                    ui.label("z:");
                    ui.add(egui::TextEdit::singleline(&mut camera_up.2).desired_width(30.0));
                });
                ui.horizontal(|ui| {
                    if ui.button("update").clicked() {
                        camera_update = true;
                    }
                    if ui.button("reset").clicked() {
                        camera_reset = true;
                    }
                });
            });
        });

        let (egui_output, paint_cmds) = egui_ctx.end_frame();

        if !egui_output.copied_text.is_empty() {
            egui_glfw_gl::copy_to_clipboard(&mut egui_input_state, egui_output.copied_text);
        }

        let paint_jobs = egui_ctx.tessellate(paint_cmds);
        painter.paint_jobs(
            None,
            paint_jobs,
            &egui_ctx.texture(),
            native_pixels_per_point,
        );

        window.update(&mut egui_input_state);
    }
}
