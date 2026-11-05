#version 330
in vec2 fragTexCoord;
in vec4 fragColor;
uniform sampler2D texture0;
uniform vec2 texel;
out vec4 finalColor;
void main() {
    vec2 uv=fragTexCoord;
    vec3 c=texture(texture0,uv).rgb;
    vec3 nw=texture(texture0,uv+vec2(-1.,-1.)*texel).rgb;
    vec3 ne=texture(texture0,uv+vec2(1.,-1.)*texel).rgb;
    vec3 sw=texture(texture0,uv+vec2(-1.,1.)*texel).rgb;
    vec3 se=texture(texture0,uv+vec2(1.,1.)*texel).rgb;
    vec3 l=vec3(.299,.587,.114);
    float lc=dot(c,l), a=dot(nw,l), b=dot(ne,l), d=dot(sw,l), e=dot(se,l);
    float lo=min(lc,min(min(a,b),min(d,e))), hi=max(lc,max(max(a,b),max(d,e)));
    if(hi-lo<max(.035,hi*.1)) { finalColor=vec4(c,1.)*fragColor; return; }
    vec2 dir=vec2(-((a+b)-(d+e)),(a+d)-(b+e));
    float reduce=max((a+b+d+e)*.03125,.0078125);
    dir=clamp(dir/(min(abs(dir.x),abs(dir.y))+reduce),vec2(-8.),vec2(8.))*texel;
    vec3 inner=.5*(texture(texture0,uv-dir/6.).rgb+texture(texture0,uv+dir/6.).rgb);
    vec3 wide=inner*.5+.25*(texture(texture0,uv-dir*.5).rgb+texture(texture0,uv+dir*.5).rgb);
    float lum=dot(wide,l);
    finalColor=vec4(lum<lo||lum>hi?inner:wide,1.)*fragColor;
}
