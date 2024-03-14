#version 460 core
out vec4 FragColor;

in vec3 _frag_pos;
in vec2 _tex_coords;
in vec3 _normals;

uniform sampler2D tex;
uniform vec3 color;
uniform vec3 light_color;
uniform vec3 light_pos;
uniform vec3 light_pos2;
uniform vec3 view_pos;

void main() {
    float ambient_factor = 0.1;
    vec3 ambient = ambient_factor * light_color;

    vec3 norm = normalize(_normals);
    vec3 light_dir = normalize(light_pos - _frag_pos);
    vec3 light_dir2 = normalize(light_pos2 - _frag_pos);
    float diff = max(dot(norm, light_dir), 0.0);
    float diff2 = max(dot(norm, light_dir2), 0.0);
    vec3 diffuse = diff * light_color;
    vec3 diffuse2 = diff2 * light_color;

    float specular_factor = 0.5;
    vec3 view_dir = normalize(view_pos - _frag_pos);
    vec3 reflect_dir = reflect(-light_dir, norm);
    float spec = pow(max(dot(view_dir, reflect_dir), 0.0), 32);
    vec3 specular = specular_factor * spec * light_color;

    vec4 result = vec4((ambient + diffuse + diffuse2 + specular) * color, 1.0);
    FragColor = texture(tex, _tex_coords) * result;
}
