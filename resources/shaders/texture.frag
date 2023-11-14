#version 460 core
out vec4 FragColor;

in vec2 _tex_coords;

uniform sampler2D tex;
uniform vec4 color;
uniform vec4 light_color;

void main() {
   FragColor = mix(texture(tex, _tex_coords), color * light_color, 0.5);
}
