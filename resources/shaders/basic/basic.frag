#version 460 core
out vec4 FragColor;

in vec2 TexCoord;

uniform vec4 globalColor;
uniform sampler2D myTex;

void main() {
   FragColor = texture(myTex, TexCoord) * globalColor;
}