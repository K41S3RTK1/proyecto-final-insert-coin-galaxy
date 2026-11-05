#version 330
in vec3 localPos; in vec3 worldPos; in vec3 normal; in vec4 color;
flat in int style;
uniform vec3 eye;
uniform float time;
uniform vec3 fogColor;
uniform float fogAmount;
out vec4 finalColor;
void main() {
    vec3 n=normalize(normal), v=normalize(eye-worldPos), light=normalize(vec3(-0.5,0.8,0.6));
    float nl=dot(n,light), facing=abs(dot(n,v));
    vec3 base=color.rgb;
    if(style==2) {
        float bands=0.5+0.5*sin(localPos.y*24.+time*1.6+sin(localPos.x*9.)*2.);
        finalColor=vec4(1.,(157.+bands*65.)/255.,(56.+bands*58.)/255.,1.); return;
    }
    if(style==4) { finalColor=vec4(base,1.); return; }
    if(style==1) {
        float q=sin(localPos.x*9.+sin(localPos.z*7.))+cos(localPos.y*11.+localPos.z*3.)*0.65;
        base=mix(vec3(.70,.47,.35),vec3(.83,.70,.47),smoothstep(-.5,-.3,q));
        base=mix(base,vec3(.41,.59,.50),smoothstep(.2,.4,q));
    }
    if(style==6) {
        float stripe=sin(worldPos.x*.3+sin(worldPos.z*.18)*1.4)*.5+.5;
        base=mix(vec3(.19,.76,.09),vec3(.36,.87,.07),stripe);
    }
    if(style==8) {
        vec3 p=localPos;
        float h=abs(n.x)>abs(n.z)?p.z:p.x;
        vec2 uv=fract(vec2(h*2.5,p.y*3.));
        float win=step(.22,uv.x)*step(uv.x,.5)*step(.28,uv.y)*step(uv.y,.57);
        base=mix(vec3(.17,.17,.27),vec3(.42,.30,.5),smoothstep(.4,.5,p.y));
        base=mix(base,vec3(.29,.76,.84),win*step(abs(n.y),.85));
    }
    if(style>=9 && style<=11) {
        vec3 p=normalize(localPos);
        float q=sin(p.x*5.+p.z*2.)*cos(p.y*4.-p.z*3.);
        float coast=-.18+.16*sin(p.x*6.+p.z*4.);
        float check=mod(floor(atan(p.z,p.x)*5.092958)+floor(asin(clamp(p.y,-1.,1.))*7.639437),2.);
        vec3 earth=mix(vec3(126.,52.,16.),vec3(205.,111.,37.),step(1.,check))/255.;
        vec3 grass=mix(vec3(55.,193.,22.),vec3(112.,228.,18.),smoothstep(.22,.28,q))/255.;
        vec3 water=vec3(10.,201.,224.)/255.;
        float wave=sin(p.x*75.+p.z*40.+time*2.);
        water=mix(water,vec3(.6,1.,1.),smoothstep(.96,1.,wave)*.35);
        vec3 land=mix(water,vec3(1.,.87,.42),smoothstep(-.48,-.44,q));
        land=mix(land,grass,smoothstep(-.40,-.36,q));
        base=mix(earth,vec3(.60,.97,.11),smoothstep(coast-.008,coast+.008,p.y));
        base=mix(base,land,smoothstep(coast+.05,coast+.065,p.y));
    }
    if(style==12) {
        float check=mod(floor(localPos.x*2.)+floor(localPos.y*2.)+floor(localPos.z*2.),2.);
        base=mix(vec3(126.,52.,16.),vec3(205.,111.,37.),step(1.,check))/255.;
    }
    if(style==13) {
        float wave=sin(worldPos.y*13.+worldPos.x*5.-time*4.);
        base=mix(vec3(.04,.78,.88),vec3(.56,.98,1.),smoothstep(.8,.95,wave));
    }
    float diffuse=.72+.16*smoothstep(-.12,-.06,nl)+.12*smoothstep(.47,.53,nl);
    if(style==14) diffuse=.67+.33*max(nl,0.);
    float shine=pow(max(dot(n,normalize(v+light)),0.),style==3?36.:50.);
    vec3 lit=base*diffuse + vec3(1.,.94,.82)*shine*(style==3?.38:.06);
    lit+=vec3(.06,.10,.15)*base*(1.-max(n.y,0.));
    if(style==0 || style==1) lit=mix(vec3(.025,.08,.20),lit,smoothstep(.025,.09,facing));
    if(style==7) {
        lit=mix(vec3(.08,.25,.38),vec3(.43,.77,.88),.25+abs(n.y)*.45);
        lit+=pow(1.-facing,3.)*.25+shine*.5;
    }
    if(style==14) lit+=base*pow(1.-facing,3.)*.10;
    float fog=(1.-exp(-length(worldPos-eye)*.006))*fogAmount;
    lit=mix(lit,fogColor,clamp(fog,0.,.65));
    finalColor=vec4(clamp(lit,0.,1.),1.);
}
