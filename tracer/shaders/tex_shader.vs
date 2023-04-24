#version 420 core

layout(location = 0) in vec2 pos;
layout(location = 1) in vec2 tex;

out vec2 vTex;

uniform mat3 world;

void main() {
    gl_Position = vec4((world * vec3(pos.x, pos.y , 1.0)),1.0);
    vTex = tex;
}
