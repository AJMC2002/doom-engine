#version 460 core
out vec4 FragColor;

in vec4 _color;

void main() {
   FragColor = _color;
}