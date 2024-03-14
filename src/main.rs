use doom_engine::graphics::mesh::Cube;
use doom_engine::graphics::{wrapper::*, Window};
use doom_engine::maths::*;
use doom_engine::vector;
use egui::{Align2, RichText};
use egui_glfw::egui;
use gl::types::*;
use std::error::Error;
use std::{mem, os::raw::c_void, ptr};

static WIDTH: u32 = 1920;

static HEIGHT: u32 = 1080;

fn main() -> Result<(), Box<dyn Error>> {
    let mut window = Window::new(WIDTH, HEIGHT, "Doom Engine");

    let mut shader_program = ShaderProgram::new(
        "resources/shaders/texture.vert",
        "resources/shaders/texture.frag",
    );

    let texture_gato = Texture2D::new("resources/textures/cat.jpg");
    let texture_gatorrito = Texture2D::new("resources/textures/gatorrito.jpg");
    let texture_pog = Texture2D::new("resources/textures/pog.jpg");
    let textures = [&texture_gato, &texture_gatorrito, &texture_pog];
    let mut main_texture = 0;

    // All Buffer Objects are binded and the data is stored on creation
    let _vao = VAO::new();
    let _vbo: VBO = BO::new(
        gl::STATIC_DRAW,
        vec![
            -0.5, -0.5, -0.5, 0.0, 1.0, 0.0, 0.0, -1.0, //
            0.5, -0.5, -0.5, 1.0, 0.0, 0.0, 0.0, -1.0, //
            0.5, 0.5, -0.5, 1.0, 1.0, 0.0, 0.0, -1.0, //
            0.5, 0.5, -0.5, 1.0, 1.0, 0.0, 0.0, -1.0, //
            -0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 0.0, -1.0, //
            -0.5, -0.5, -0.5, 0.0, 0.0, 0.0, 0.0, -1.0, //
            //
            -0.5, -0.5, 0.5, 0.0, 0.0, 0.0, 0.0, 1.0, //
            0.5, -0.5, 0.5, 1.0, 0.0, 0.0, 0.0, 1.0, //
            0.5, 0.5, 0.5, 1.0, 1.0, 0.0, 0.0, 1.0, //
            0.5, 0.5, 0.5, 1.0, 1.0, 0.0, 0.0, 1.0, //
            -0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, //
            -0.5, -0.5, 0.5, 0.0, 0.0, 0.0, 0.0, 1.0, //
            //
            -0.5, 0.5, 0.5, 1.0, 0.0, -1.0, 0.0, 0.0, //
            -0.5, 0.5, -0.5, 1.0, 1.0, -1.0, 0.0, 0.0, //
            -0.5, -0.5, -0.5, 0.0, 1.0, -1.0, 0.0, 0.0, //
            -0.5, -0.5, -0.5, 0.0, 1.0, -1.0, 0.0, 0.0, //
            -0.5, -0.5, 0.5, 0.0, 0.0, -1.0, 0.0, 0.0, //
            -0.5, 0.5, 0.5, 1.0, 0.0, -1.0, 0.0, 0.0, //
            //
            0.5, 0.5, 0.5, 1.0, 0.0, 1.0, 0.0, 0.0, //
            0.5, 0.5, -0.5, 1.0, 1.0, 1.0, 0.0, 0.0, //
            0.5, -0.5, -0.5, 0.0, 1.0, 1.0, 0.0, 0.0, //
            0.5, -0.5, -0.5, 0.0, 1.0, 1.0, 0.0, 0.0, //
            0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 0.0, //
            0.5, 0.5, 0.5, 1.0, 0.0, 1.0, 0.0, 0.0, //
            //
            -0.5, -0.5, -0.5, 0.0, 1.0, 0.0, -1.0, 0.0, //
            0.5, -0.5, -0.5, 1.0, 1.0, 0.0, -1.0, 0.0, //
            0.5, -0.5, 0.5, 1.0, 0.0, 0.0, -1.0, 0.0, //
            0.5, -0.5, 0.5, 1.0, 0.0, 0.0, -1.0, 0.0, //
            -0.5, -0.5, 0.5, 0.0, 0.0, 0.0, -1.0, 0.0, //
            -0.5, -0.5, -0.5, 0.0, 1.0, 0.0, -1.0, 0.0, //
            //
            -0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 1.0, 0.0, //
            0.5, 0.5, -0.5, 1.0, 1.0, 0.0, 1.0, 0.0, //
            0.5, 0.5, 0.5, 1.0, 0.0, 0.0, 1.0, 0.0, //
            0.5, 0.5, 0.5, 1.0, 0.0, 0.0, 1.0, 0.0, //
            -0.5, 0.5, 0.5, 0.0, 0.0, 0.0, 1.0, 0.0, //
            -0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 1.0, 0.0,
        ],
    );
    let cube_pos = [
        vector!(2.0, 5.0, -15.0),
        vector!(-1.5, -2.2, -2.5),
        vector!(-3.8, -2.0, -12.3),
        vector!(2.4, -0.4, -3.5),
        vector!(-1.7, 3.0, -7.5),
        vector!(1.3, -2.0, -2.5),
        vector!(1.5, 2.0, -2.5),
        vector!(1.5, 0.2, -1.5),
        vector!(-1.3, 1.0, -1.5),
    ];

    let stride = 8 * mem::size_of::<GLfloat>() as GLsizei;

    let _pos_attrib = VertexAttrib::new(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
    let _tex_attrib = VertexAttrib::new(
        1,
        2,
        gl::FLOAT,
        gl::FALSE,
        stride,
        (3 * mem::size_of::<GLfloat>()) as *const c_void,
    );
    let normal_attrib = VertexAttrib::new(
        2,
        3,
        gl::FLOAT,
        gl::FALSE,
        stride,
        (5 * mem::size_of::<GLfloat>()) as *const c_void,
    );

    _vbo.unbind();
    _vao.unbind();
    _pos_attrib.disable();
    _tex_attrib.disable();
    normal_attrib.disable();
    texture_gato.unbind();
    texture_pog.unbind();
    texture_gatorrito.unbind();
    shader_program.unbind();

    let mut light_shader = ShaderProgram::new(
        "resources/shaders/light.vert",
        "resources/shaders/light.frag",
    );

    let mut light = Cube::new(
        Some((
            Matrix::translation(vector![2.5, 1.0, 2.0]),
            Matrix::identity(4),
            Matrix::scaling(vector![0.2, 0.2, 0.2]),
        )),
        None,
        None,
    );

    let mut light2 = Cube::new(
        Some((
            Matrix::translation(vector![-2.5, 1.0, 2.0]),
            Matrix::identity(4),
            Matrix::scaling(vector![0.2, 0.2, 0.2]),
        )),
        None,
        None,
    );

    unsafe {
        gl::ClearColor(154. / 258., 127. / 258., 174. / 258., 1.0);
    }
    window.glfw_handle_mut().set_time(0.);
    while !window.window_handle().should_close() {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        shader_program.bind();
        //_pos_attrib.enable();
        //_tex_attrib.enable();
        _vao.bind();
        shader_program.uniform_matrix_4fv("proj", &window.camera_handle().proj());
        shader_program.uniform_matrix_4fv("view", &window.camera_handle().view());
        shader_program.uniform_3fv("color", &vector![1.0, 1.0, 1.0]);
        shader_program.uniform_3fv("light_color", &vector![1.0, 1.0, 1.0]);
        shader_program.uniform_3fv("light_pos", &light.pos());
        shader_program.uniform_3fv("light_pos2", &light2.pos());
        shader_program.uniform_3fv("view_pos", &window.camera_handle().pos());

        cube_pos.iter().for_each(|pos| {
            let m = Matrix::translation(pos.clone())
                     //* Matrix::translation((
                     //    translate.0.parse::<f32>().unwrap_or(0.0),
                     //    translate.1.parse::<f32>().unwrap_or(0.0),
                     //    translate.2.parse::<f32>().unwrap_or(0.0),
                     //))
                     //* Matrix::scaling((
                     //    scale.0.parse::<f32>().unwrap_or(1.0),
                     //    scale.1.parse::<f32>().unwrap_or(1.0),
                     //    scale.2.parse::<f32>().unwrap_or(1.0),
                     //))
                     ;
            shader_program.uniform_matrix_4fv("model", &m);
            shader_program.uniform_matrix_3fv("normal", &m.to_normal());

            shader_program.uniform_2dtex("tex", textures[main_texture]);
            unsafe { gl::DrawArrays(gl::TRIANGLES, 0, 36) }
        });

        _vao.unbind();
        _pos_attrib.disable();
        _tex_attrib.disable();

        println!("Draw cube");
        light.draw(window.camera_handle(), &mut light_shader);
        light2.set_pos(vector!(-light.pos()[0], light.pos()[1], light.pos()[2]));
        light2.draw(window.camera_handle(), &mut light_shader);

        window.begin_ui();

        egui::SidePanel::left("my_side_panel").resizable(true).show(
            &window.ui_handle().get_egui_ctx().to_owned(),
            |ui| {
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
                ui.label(format!("camera: {:#?}", window.camera_handle()));
            },
        );

        if window.is_camera_still() {
            egui::Window::new("")
                .title_bar(false)
                .collapsible(false)
                .resizable(false)
                .anchor(Align2::RIGHT_TOP, (0.0, 0.0))
                .show(&window.ui_handle().get_egui_ctx().to_owned(), |ui| {
                    ui.label(RichText::new("Camera rotation is set still.").size(15.0))
                });
        }

        egui::Window::new("Objects").show(&window.ui_handle().get_egui_ctx().to_owned(), |ui| {
            ui.set_max_width(280.0);
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    egui::ComboBox::from_label("Choose a texture:")
                        .selected_text(format!("{:?}", main_texture))
                        .show_index(ui, &mut main_texture, 3, |i| format!("Texture No. {}", i))
                })
            });
            ui.group(|ui| {
                ui.label("Light");
                ui.horizontal(|ui| {
                    ui.label("time");
                    ui.label(window.glfw_handle().get_time().to_string());
                    ui.label("s.");
                });
                let mut light_pos = light.pos();
                ui.horizontal(|ui| {
                    ui.label("light_pos");
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("x");
                            ui.add(egui::Slider::new(&mut light_pos[0], -100.0..=100.0));
                        });
                        ui.horizontal(|ui| {
                            ui.label("y");
                            ui.add(egui::Slider::new(&mut light_pos[1], -15.0..=15.0));
                        });
                        ui.horizontal(|ui| {
                            ui.label("z");
                            ui.add(egui::Slider::new(&mut light_pos[2], -15.0..=15.0));
                        });
                    })
                });
                light.set_pos(light_pos);
            });
        });

        window.end_ui();

        window.update();
    }

    Ok(())
}
