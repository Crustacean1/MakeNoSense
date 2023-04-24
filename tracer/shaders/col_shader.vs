#version 420 core

layout(location = 0) in vec2 pos;
layout(location = 1) in vec4 col;

out vec4 vCol;

uniform mat3 world;

void main() {
    gl_Position = vec4(world * vec3(pos.xy, 1.0),1.0);
    vCol = col;
}
