#version 460 core

layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 col;

out vec3 vCol;

uniform mat4 world;

void main() {
    gl_Position = world * vec4(pos.x, pos.y, pos.z, 1.0);
    vCol = col;
}
