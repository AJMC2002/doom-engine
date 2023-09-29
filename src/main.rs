use doom_engine::graphics::{wrapper::*, Window};
use doom_engine::maths::*;
use egui_glfw::egui;
use gl::types::*;
use std::f32::consts::PI;
use std::{mem, os::raw::c_void, ptr};

static WIDTH: u32 = 1400;

static HEIGHT: u32 = 800;

fn main() {
    let mut window = Window::new(WIDTH, HEIGHT, "Doom Engine");

    let mut shader_program = ShaderProgram::new(
        "resources/shaders/texture.vert",
        "resources/shaders/texture.frag",
    );

    let texture_gato = Texture2D::new("resources/textures/cat.jpg");
    let texture_gatorrito = Texture2D::new("resources/textures/gatorrito.jpg");
    let texture_pog = Texture2D::new("resources/textures/pog.jpg");

    // All Buffer Objects are binded and the data is stored on creation
    let _vao = VAO::new();
    let _vbo: VBO = BO::new(
        gl::STATIC_DRAW,
        vec![
            // positions [3] // tex [2]
            -0.5, 0.5, 0., 0.0, 1.0, // top right
            0.5, 0.5, 0., 1., 1., // bottom right
            -0.5, -0.5, 0., 0., 0., // bottom left
            0.5, -0.5, 0., 1., 0., // top left
        ],
    );
    let _ebo: EBO = BO::new(
        gl::STATIC_DRAW,
        vec![
            2, 1, 0, // first Triangle
            3, 2, 1, // second Triangle
        ],
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

    _ebo.unbind();
    _vbo.unbind();
    _vao.unbind();
    _pos_attrib.disable();
    _tex_attrib.disable();
    texture_gato.unbind();
    texture_pog.unbind();
    texture_gatorrito.unbind();
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
        }

        {
            t = window.glfw_handle().get_time() as f32;

            // shader_program.bind();
            // _pos_attrib.enable();
            // _tex_attrib.enable();
            _vao.bind();
            _ebo.bind();
            // shader_program.uniform_4fv(
            //     "color",
            //     vector![1.0 - color, color, color.powi(2), 1.0],
            // );
            shader_program.uniform_matrix_4fv(
                "proj",
                &Matrix::projection_perspective(PI / 2., WIDTH as f32 / HEIGHT as f32, 0.1, 100.),
            );
            shader_program.uniform_matrix_4fv("view", &Matrix::translation((0., 0., -1.5)));

            shader_program.uniform_matrix_4fv(
                "model",
                &(Matrix::translation((0., 0.5 * (1.5 * t).sin(), 0.))
                    * Matrix::translation((
                        translate.0.parse::<f32>().unwrap_or(0.0),
                        translate.1.parse::<f32>().unwrap_or(0.0),
                        translate.2.parse::<f32>().unwrap_or(0.0),
                    ))
                    * Matrix::rotation((0.12, t, 0.))
                    * Matrix::rotation(rotate)
                    * Matrix::scaling((
                        scale.0.parse::<f32>().unwrap_or(0.0),
                        scale.1.parse::<f32>().unwrap_or(0.0),
                        scale.2.parse::<f32>().unwrap_or(0.0),
                    ))),
            );

            texture_gato.bind();
            shader_program.uniform_2dtex("tex", &texture_gato);
            unsafe { gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null()) }
            texture_gato.unbind();

            shader_program.uniform_matrix_4fv(
                "model",
                &(Matrix::translation(((2. * t).sin(), 0., (2. * t).cos()))
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
                    ))),
            );

            texture_gatorrito.bind();
            shader_program.uniform_2dtex("tex", &texture_gatorrito);
            unsafe { gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null()) }
            texture_gatorrito.unbind();

            shader_program.uniform_matrix_4fv(
                "model",
                &(Matrix::translation((0.2 * (t).sin(), (2. * t).sin(), (2. * t).cos()))
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
                    ))),
            );

            texture_pog.bind();
            shader_program.uniform_2dtex("tex", &texture_pog);
            unsafe { gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null()) }
            texture_pog.unbind();

            _ebo.unbind();
            _vao.unbind();
            // _pos_attrib.disable();
            // _tex_attrib.disable();
            shader_program.unbind();
        }

        {
            window.ui_begin_frame();

            let egui_ctx = window.ui_handle().get_egui_ctx().to_owned();
            egui::SidePanel::left("my_side_panel")
                .resizable(true)
                .show(&egui_ctx, |ui| {
                    ui.heading("Hello World!");
                    if ui.button("Quit").clicked() {
                        window.window_handle_mut().set_should_close(true);
                    }

                    egui::ComboBox::from_label("Version")
                        .width(150.0)
                        .selected_text("foo")
                        .show_ui(ui, |ui| {
                            egui::CollapsingHeader::new("Dev")
                                .default_open(true)
                                .show(ui, |ui| {
                                    ui.label("contents");
                                });
                        });

                    ui.label(format!(
                        "window content scale: {:?}",
                        window.window_handle().get_content_scale()
                    ));
                    ui.label(format!(
                        "monitor content scale: {:?}",
                        window
                            .glfw_handle_mut()
                            .with_connected_monitors(|_, monitors| {
                                monitors
                                    .iter()
                                    .map(|monitor| monitor.get_content_scale())
                                    .collect::<Vec<_>>()
                            })
                    ));
                    ui.label(format!(
                        "monitor physical size in mm: {:?}",
                        window
                            .glfw_handle_mut()
                            .with_connected_monitors(|_, monitors| {
                                monitors
                                    .iter()
                                    .map(|monitor| monitor.get_physical_size())
                                    .collect::<Vec<_>>()
                            })
                    ));
                    ui.label(format!(
                        "monitor physical size in inch: {:?}",
                        window
                            .glfw_handle_mut()
                            .with_connected_monitors(|_, monitors| {
                                monitors
                                    .iter()
                                    .map(|monitor| {
                                        let mm = monitor.get_physical_size();
                                        (mm.0 as f32 / 25.4, mm.1 as f32 / 25.4)
                                    })
                                    .collect::<Vec<_>>()
                            })
                    ));
                    ui.label(format!(
                        "monitor positions: {:?}",
                        window
                            .glfw_handle_mut()
                            .with_connected_monitors(|_, monitors| {
                                monitors
                                    .iter()
                                    .map(|monitor| monitor.get_pos())
                                    .collect::<Vec<_>>()
                            })
                    ));
                    ui.label(format!(
                        "window position: {:?}",
                        window.window_handle().get_pos()
                    ));
                });

            egui::Window::new("Quad").show(&window.ui_handle().get_egui_ctx(), |ui| {
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
        }

        window.update();
    }
}
