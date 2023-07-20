use doom_engine::graphics::{wrapper::*, Window};
use doom_engine::maths::*;
use doom_engine::vector;
use egui_glfw_gl::egui;
use egui_glfw_gl::egui::{vec2, Pos2, Rect};
use egui_glfw_gl::gl;
use egui_glfw_gl::gl::types::*;
use std::f32::consts::PI;
use std::{mem, os::raw::c_void, ptr};

static WIDTH: u32 = 1600;

static HEIGHT: u32 = 800;

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
        "resources/shaders/basic/basic.vert",
        "resources/shaders/basic/basic.frag",
    );

    let texture_gato = Texture2D::new("resources/textures/cat.jpg");
    let texture_gatorrito = Texture2D::new("resources/textures/gatorrito.jpg");
    let texture_pog = Texture2D::new("resources/textures/pog.jpg");

    // All Buffer Objects are binded and the data is stored on creation
    let _vao = VAO::new();
    let _vbo: VBO = BO::new(
        gl::STATIC_DRAW,
        Box::new([
            // positions [3] // tex [2]
            -0.5, 0.5, 0., 0.0, 1.0, // top right
            0.5, 0.5, 0., 1., 1., // bottom right
            -0.5, -0.5, 0., 0., 0., // bottom left
            0.5, -0.5, 0., 1., 0., // top left
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

    // _pos_attrib.disable();
    // _tex_attrib.disable();
    _ebo.unbind();
    _vbo.unbind();
    _vao.unbind();
    texture_gato.unbind();
    texture_gatorrito.unbind();
    texture_pog.unbind();
    shader_program.unbind();

    let mut scale = (
        1.0.to_string().to_owned(),
        1.0.to_string().to_owned(),
        1.0.to_string().to_owned(),
    );
    let mut rotate = (0.0, 0.0, 0.0);
    let mut translate = (
        0.0.to_string().to_owned(),
        0.0.to_string().to_owned(),
        0.0.to_string().to_owned(),
    );

    unsafe {
        gl::ClearColor(154. / 258., 127. / 258., 174. / 258., 1.0);
    }

    let mut t;

    while !window.window_handle().should_close() {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            t = window.glfw_handle().get_time() as f32;
            let color = (t.sin() / 2.0) + 0.5;

            shader_program.bind();
            _vao.bind();
            _ebo.bind();
            texture_pog.bind();
            // shader_program.uniform_4fv(
            //     "globalColor",
            //     vector![1.0 - color, color, color.powi(2), 1.0],
            // );
            shader_program.uniform_matrix_4fv(
                "proj",
                Matrix::projection_perspective(PI / 2., WIDTH as f32 / HEIGHT as f32, 0.1, 100.),
            );
            shader_program.uniform_matrix_4fv("view", Matrix::translation((0., 0., -1.5)));
            shader_program.uniform_matrix_4fv(
                "model",
                Matrix::translation((
                    translate.0.parse::<f32>().unwrap_or(0.0),
                    translate.1.parse::<f32>().unwrap_or(0.0),
                    translate.2.parse::<f32>().unwrap_or(0.0),
                )) * Matrix::rotation((0.12, t, 0.))
                    * Matrix::rotation(rotate)
                    * Matrix::scaling((
                        scale.0.parse::<f32>().unwrap_or(0.0),
                        scale.1.parse::<f32>().unwrap_or(0.0),
                        scale.2.parse::<f32>().unwrap_or(0.0),
                    )),
            );
            shader_program.uniform_2dtex("myTex", &texture_pog);

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());

            texture_pog.unbind();
            texture_gatorrito.bind();
            shader_program.uniform_matrix_4fv(
                "model",
                Matrix::translation(((2. * t).sin(), 0., (2. * t).cos()))
                    * Matrix::translation((
                        translate.0.parse::<f32>().unwrap_or(0.0),
                        translate.1.parse::<f32>().unwrap_or(0.0),
                        translate.2.parse::<f32>().unwrap_or(0.0),
                    ))
                    * Matrix::rotation((0.12, 2. * PI * t, 0.))
                    * Matrix::rotation(rotate)
                    * Matrix::scaling((
                        scale.0.parse::<f32>().unwrap_or(0.0),
                        scale.1.parse::<f32>().unwrap_or(0.0),
                        scale.2.parse::<f32>().unwrap_or(0.0),
                    )),
            );
            shader_program.uniform_2dtex("myTex", &texture_gatorrito);

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());

            _vao.unbind();
            _ebo.unbind();
            texture_gatorrito.unbind();
            shader_program.unbind();
        }

        unsafe {
            gl::Disable(gl::DEPTH_TEST);
        }

        let (width, height) = window.window_handle().get_size();
        let native_pixels_per_point = window.window_handle().get_content_scale().0;

        egui_input_state.input.time = Some(window.glfw_handle().get_time());
        egui_ctx.begin_frame(egui_input_state.input.take());
        egui_input_state.input.screen_rect = Some(Rect::from_min_size(
            Pos2::new(0f32, 0f32),
            vec2(width as f32, height as f32) / native_pixels_per_point,
        ));
        egui_input_state.input.pixels_per_point = Some(native_pixels_per_point);

        egui::Window::new("Quad").show(&egui_ctx, |ui| {
            ui.set_max_width(280.0);
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label("time");
                    ui.label(window.glfw_handle().get_time().to_string());
                    ui.label("s.");
                });
                ui.horizontal(|ui| {
                    ui.set_max_width(250.0);
                    ui.label("scale");
                    ui.label("x:");
                    ui.add(egui::TextEdit::singleline(&mut scale.0).desired_width(30.0));
                    ui.label("y:");
                    ui.add(egui::TextEdit::singleline(&mut scale.1).desired_width(30.0));
                    ui.label("z:");
                    ui.add(egui::TextEdit::singleline(&mut scale.2).desired_width(30.0));
                });
                ui.horizontal(|ui| {
                    ui.label("rotate");
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("x");
                            ui.add(egui::Slider::new(&mut rotate.0, 0.0..=(2.0 * PI)));
                        });
                        ui.horizontal(|ui| {
                            ui.label("y");
                            ui.add(egui::Slider::new(&mut rotate.1, 0.0..=(2.0 * PI)));
                        });

                        ui.horizontal(|ui| {
                            ui.label("z");
                            ui.add(egui::Slider::new(&mut rotate.2, 0.0..=(2.0 * PI)));
                        });
                    })
                });
                ui.horizontal(|ui| {
                    ui.set_max_width(250.0);
                    ui.label("translate");
                    ui.label("x:");
                    ui.add(egui::TextEdit::singleline(&mut translate.0).desired_width(30.0));
                    ui.label("y:");
                    ui.add(egui::TextEdit::singleline(&mut translate.1).desired_width(30.0));
                    ui.label("z:");
                    ui.add(egui::TextEdit::singleline(&mut translate.2).desired_width(30.0));
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

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }

        window.update(&mut egui_input_state);
    }
}
