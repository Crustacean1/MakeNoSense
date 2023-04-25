#version 420 core

in vec4 vCol;
out vec4 fCol;
uniform vec4 ufCol;

void main(){
    fCol = ufCol;
}

