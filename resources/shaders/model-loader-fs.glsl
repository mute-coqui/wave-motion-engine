#version 330 core
out vec4 FragColor;

in vec3 ourColor;
in vec2 TexCoord;

// texture sampler
uniform sampler2D diffuse_texture;

void main()
{
    FragColor = vec4(1.0, 0.5, 0.2, 1.0);
}

