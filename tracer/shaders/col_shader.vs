#version 460 core

layout(location = 0) in vec2 pos;
layout(location = 1) in vec4 col;

out vec4 vCol;

uniform mat4 world;

void main() {
    gl_Position = world * vec4(pos.xy,0.0, 1.0);
    vCol = col;
}
