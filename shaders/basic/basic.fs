#version 330 core
out vec4 FragColor;
uniform vec4 globalColor;
void main() {
   FragColor = globalColor;
}