#version 330 core

in vec2 uv;

out vec3 colour;

void main() {
    vec2 origin = vec2(0.5, 0.5);
    vec2 d = abs(uv - origin);
    colour = vec3(0.5 - (d.x + d.y), 0.05, 0.05);
}
