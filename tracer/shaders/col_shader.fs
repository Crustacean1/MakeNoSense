#version 460 core

in vec3 vCol;
out vec4 fCol;

void main(){
    fCol = vec4(vCol,0);
}

