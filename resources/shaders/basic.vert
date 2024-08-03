#version 330 core
layout (location = 0) in vec3 position;
layout (location = 1) in vec3 color;
layout (location = 2) in vec2 coords;

out vec3 vert_color;
out vec2 tex_coords;

void main() {
    gl_Position = vec4(position, 1.0);
    vert_color = color;
    tex_coords = coords;
}
