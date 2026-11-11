#version 330
layout(location=0) in vec4 positionClip;
layout(location=1) in vec3 positionLocal;
layout(location=2) in vec3 normalWorld;
layout(location=3) in vec4 tint;
layout(location=4) in vec3 positionWorld;
layout(location=5) in float materialStyle;
layout(location=6) in vec3 tangentWorld;
out vec3 tangent;
out vec3 localPos;
out vec3 worldPos;
out vec3 normal;
out vec4 color;
flat out int style;
void main() {
    gl_Position=positionClip;
    localPos=positionLocal; worldPos=positionWorld; normal=normalWorld;
    tangent=tangentWorld;
    color=tint; style=int(materialStyle+0.5);
}
