#version 460 core
layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 tex_coords;

out vec2 _tex_coords;

uniform mat4 proj;
uniform mat4 view;
uniform mat4 model;

void main() {
    gl_Position = proj * view * model * vec4(pos, 1.0);
    _tex_coords = tex_coords;
}