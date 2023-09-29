#version 460 core
out vec4 FragColor;

in vec2 _tex_coords;

uniform sampler2D tex;

void main() {
   FragColor = texture(tex, _tex_coords);
}