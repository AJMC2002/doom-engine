#version 460 core
layout (location = 0) in vec3 pos;
layout (location = 1) in vec4 color;

out vec4 _color;

uniform mat4 proj;
uniform mat4 view;
uniform mat4 model;

void main() {
    gl_Position = proj * view * model * vec4(pos, 1.0);
    _color = color;
}