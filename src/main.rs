use doom_engine::graphics::mesh::Cube;
use doom_engine::graphics::{wrapper::*, Window};
use doom_engine::maths::*;
use doom_engine::{matrix, vector};
use egui_glfw::egui;
use gl::types::*;
use std::error::Error;
use std::f32::consts::PI;
use std::fs::File;
use std::io::BufReader;
use std::{mem, os::raw::c_void, ptr};
use tobj::{load_obj, LoadOptions};

static WIDTH: u32 = 1920;

static HEIGHT: u32 = 1080;

fn main() -> Result<(), Box<dyn Error>> {
    // let (models, materials) = load_obj(
    //     "resources/objects/crash/crashbandicoot.obj",
    //     &LoadOptions::default(),
    // )
    // .expect("obj loading");
    // let materials = materials.expect("material loading");

    // println!("Number of models          = {}", models.len());
    // println!("Number of materials       = {}", materials.len());

    // for (i, m) in models.iter().enumerate() {
    //     let mesh = &m.mesh;
    //     println!("");
    //     println!("model[{}].name             = \'{}\'", i, m.name);
    //     println!("model[{}].mesh.material_id = {:?}", i, mesh.material_id);

    //     println!(
    //         "model[{}].face_count       = {}",
    //         i,
    //         mesh.face_arities.len()
    //     );

    //     let mut next_face = 0;
    //     for face in 0..mesh.face_arities.len() {
    //         let end = next_face + mesh.face_arities[face] as usize;

    //         let face_indices = &mesh.indices[next_face..end];
    //         println!(" face[{}].indices          = {:?}", face, face_indices);

    //         if !mesh.texcoord_indices.is_empty() {
    //             let texcoord_face_indices = &mesh.texcoord_indices[next_face..end];
    //             println!(
    //                 " face[{}].texcoord_indices = {:?}",
    //                 face, texcoord_face_indices
    //             );
    //         }
    //         if !mesh.normal_indices.is_empty() {
    //             let normal_face_indices = &mesh.normal_indices[next_face..end];
    //             println!(
    //                 " face[{}].normal_indices   = {:?}",
    //                 face, normal_face_indices
    //             );
    //         }

    //         next_face = end;
    //     }

    //     // Normals and texture coordinates are also loaded, but not printed in
    //     // this example.
    //     println!(
    //         "model[{}].positions        = {}",
    //         i,
    //         mesh.positions.len() / 3
    //     );
    //     assert!(mesh.positions.len() % 3 == 0);

    //     for vtx in 0..mesh.positions.len() / 3 {
    //         println!(
    //             "              position[{}] = ({}, {}, {})",
    //             vtx,
    //             mesh.positions[3 * vtx],
    //             mesh.positions[3 * vtx + 1],
    //             mesh.positions[3 * vtx + 2]
    //         );
    //     }
    // }

    // for (i, m) in materials.iter().enumerate() {
    //     println!("material[{}].name = \'{}\'", i, m.name);
    //     println!(
    //         "    material.Ka = ({:#?})",
    //         m.ambient.unwrap_or([-1., -1., -1.])
    //     );
    //     println!(
    //         "    material.Kd = ({:#?})",
    //         m.diffuse.unwrap_or([-1., -1., -1.])
    //     );
    //     println!(
    //         "    material.Ks = ({:#?})",
    //         m.specular.unwrap_or([-1., -1., -1.])
    //     );
    //     println!("    material.Ns = {}", m.shininess.unwrap_or(-1.));
    //     println!("    material.d = {}", m.dissolve.unwrap_or(-1.));
    //     println!(
    //         "    material.map_Ka = {}",
    //         m.ambient_texture.as_ref().unwrap_or(&"NONE".to_string())
    //     );
    //     println!(
    //         "    material.map_Kd = {}",
    //         m.diffuse_texture.as_ref().unwrap_or(&"NONE".to_string())
    //     );
    //     println!(
    //         "    material.map_Ks = {}",
    //         m.specular_texture.as_ref().unwrap_or(&"NONE".to_string())
    //     );
    //     println!(
    //         "    material.map_Ns = {}",
    //         m.shininess_texture.as_ref().unwrap_or(&"NONE".to_string())
    //     );
    //     println!(
    //         "    material.map_Bump = {}",
    //         m.normal_texture.as_ref().unwrap_or(&"NONE".to_string())
    //     );
    //     println!(
    //         "    material.map_d = {}",
    //         m.dissolve_texture.as_ref().unwrap_or(&"NONE".to_string())
    //     );

    //     for (k, v) in &m.unknown_param {
    //         println!("    material.{} = {}", k, v);
    //     }
    // }
    // return Ok(());
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
        (2.0, 5.0, -15.0),
        (-1.5, -2.2, -2.5),
        (-3.8, -2.0, -12.3),
        (2.4, -0.4, -3.5),
        (-1.7, 3.0, -7.5),
        (1.3, -2.0, -2.5),
        (1.5, 2.0, -2.5),
        (1.5, 0.2, -1.5),
        (-1.3, 1.0, -1.5),
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

    let mut basic_shader = ShaderProgram::new(
        "resources/shaders/basic.vert",
        "resources/shaders/basic.frag",
    );

    let mut light_shader = ShaderProgram::new(
        "resources/shaders/light.vert",
        "resources/shaders/light.frag",
    );

    let light_pos = vector![2.5, 1.0, 2.0];
    let mut light = Cube::new(
        Some(
            Matrix::translation((light_pos[0], light_pos[1], light_pos[2]))
                * Matrix::scaling((0.2, 0.2, 0.2)),
        ),
        None,
        None,
    );

    let mut t;
    unsafe {
        gl::ClearColor(154. / 258., 127. / 258., 174. / 258., 1.0);
    }
    window.glfw_handle_mut().set_time(0.);
    while !window.window_handle().should_close() {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        {
            t = window.glfw_handle().get_time() as f32;
            println!("cum");
            shader_program.bind();
            //_pos_attrib.enable();
            //_tex_attrib.enable();
            _vao.bind();
            shader_program.uniform_matrix_4fv("proj", &window.camera_handle().proj());
            shader_program.uniform_matrix_4fv("view", &window.camera_handle().view());
            shader_program.uniform_3fv("color", &vector![1.0, 1.0, 1.0]);
            shader_program.uniform_3fv("light_color", &vector![1.0, 1.0, 1.0]);
            shader_program.uniform_3fv("light_pos", &light_pos);
            shader_program.uniform_3fv("view_pos", &window.camera_handle().pos());

            cube_pos.iter().for_each(|pos| {
                let m = Matrix::translation(*pos)
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

                shader_program.uniform_2dtex("tex", &texture_gatorrito);
                unsafe { gl::DrawArrays(gl::TRIANGLES, 0, 36) }
                texture_gatorrito.unbind();
            });

            _vao.unbind();
            _pos_attrib.disable();
            _tex_attrib.disable();
        }
        println!("Draw cube");
        light.draw(window.camera_handle(), &mut light_shader);

        {
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

            egui::Window::new("Quad").show(&window.ui_handle().get_egui_ctx().to_owned(), |ui| {
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

            window.end_ui();
        }

        window.update();
    }

    Ok(())
}
