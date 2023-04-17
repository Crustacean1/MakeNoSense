#version 430 core

in vec2 vTex;
out vec4 fCol;

uniform sampler2D texture0;

void main(){
    fCol = texture(texture0, vTex);
}

