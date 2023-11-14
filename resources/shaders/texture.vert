#version 460 core
layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 tex_coords;
layout (location = 2) in vec3 normals;

out vec3 _frag_pos;
out vec2 _tex_coords;
out vec3 _normals;

uniform mat4 proj;
uniform mat4 view;
uniform mat4 model;
uniform mat3 normal;

void main() {
    gl_Position = proj * view * model * vec4(pos, 1.0);
    _frag_pos = vec3(model * vec4(pos, 1.0));
    _tex_coords = tex_coords;
    _normals = normal * normals;
}
