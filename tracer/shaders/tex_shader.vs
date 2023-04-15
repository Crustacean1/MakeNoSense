#version 460 core

layout(location = 0) in vec3 pos;
layout(location = 1) in vec2 tex;

out vec2 vTex;

uniform mat4 world;

void main() {
    gl_Position = world * vec4(pos, 1.0);
    vTex = tex;
}
