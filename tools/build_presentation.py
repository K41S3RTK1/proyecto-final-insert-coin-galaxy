"""Edit captured gameplay into a presentation and generate README media.
Requires FFmpeg, ffprobe and Pillow. No generated gameplay or external music.
Run tools/record-delivery.sh first for an up-to-date HD capture.
"""
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont
import json, subprocess, sys

ROOT=Path(__file__).resolve().parents[1]
BUILD=ROOT/'output/presentation-build'
MEDIA=ROOT/'docs/media'
SOURCE=ROOT/'output/green-hill-silent.mp4'
FONT=ROOT/'assets/fonts/Nunito.ttf'
W,H=1920,1080
NAVY='#07182d'; CYAN='#57edd7'; WHITE='#edf8ff'; GOLD='#ffdb4b'
BUILD.mkdir(parents=True,exist_ok=True); MEDIA.mkdir(parents=True,exist_ok=True)

def run(*args):
    subprocess.run([str(x) for x in args],check=True)
def ff(*args):run('ffmpeg','-hide_banner','-loglevel','error','-y',*args)
def font(size):return ImageFont.truetype(str(FONT),size)
def text(draw,xy,value,size=36,color=WHITE):draw.text(xy,value,font=font(size),fill=color)
def card(name,title,subtitle,kicker='INSERT COIN / GALAXY',number=None,cover=False):
    # Original typography and vector shapes, rendered as a transparent video layer.
    im=Image.new('RGBA',(W,H));d=ImageDraw.Draw(im)
    if cover:
        for x in range(W):
            alpha=round(255*(1-min(1,max(0,x-800)/850)))
            d.line((x,0,x,H),fill=(7,24,45,alpha))
        d.rectangle((0,0,W,120),fill=(7,24,45,255))
        text(d,(86,38),'DANIEL SANDOVAL  /  GRÁFICAS POR COMPUTADORA',30,CYAN)
        text(d,(84,246),'INSERT COIN',104)
        text(d,(80,354),'GALAXY',164,GOLD)
        d.rounded_rectangle((88,562,520,566),radius=2,fill=CYAN)
        text(d,(88,606),title,44)
        text(d,(88,672),subtitle,30,'#c4dbe9')
        d.rounded_rectangle((88,790,691,863),radius=36,fill=GOLD)
        text(d,(120,806),'5 MUNDOS  ·  UNA AVENTURA',31,NAVY)
        text(d,(88,964),'CAPTURAS DEL JUEGO  /  EDICIÓN HD',27,'#b9d2e4')
    else:
        d.rectangle((0,0,W,135),fill=NAVY)
        d.rectangle((0,900,W,H),fill=NAVY)
        d.rectangle((68,900,1852,903),fill=CYAN)
        text(d,(70,27),kicker,24,CYAN)
        text(d,(68,63),title,43)
        if number:text(d,(1670,55),number,48,GOLD)
        text(d,(70,928),subtitle,32)
        text(d,(70,1005),'MOTOR REAL  ·  CONTROLES AUTOMÁTICOS  ·  GPU 1080p',23,'#9db9ce')
        text(d,(1480,1005),'DANIEL SANDOVAL',23,'#9db9ce')
    path=BUILD/f'{name}.png';im.save(path);return path

segments=[
    ('intro',14,4,'Un universo para jugar.','Exploración, velocidad y cinco desafíos.',None,True),
    ('system',0,6,'Cinco destinos. Una galaxia.','Planetas OBJ que orbitan y giran en tiempo real.','01',False),
    ('flight',8,12,'Toma el control de la nave.','Vuelo libre, cámara de seguimiento y asistencia orbital.','02',False),
    ('models',31,8,'Mundos con identidad propia.','Palmeras, arquitectura, ferrocarriles y terrenos sobre superficies curvas.','03',False),
    ('garden',47,9,'The Inkwell Garden','Muévete, dispara y aprende los patrones del jefe.','04',False),
    ('dojo',71,9,'Neon Dojo','Lee el ataque. Bloquea, esquiva y responde.','05',False),
    ('runner',89,10,'Banana Rush','Cambia de carril, salta y deslízate entre los trenes.','06',False),
    ('mario',108,12,'Mushroom Kingdom','Un recorrido lateral de saltos, bloques y plataformas.','07',False),
    ('selection',136,3,'Sonic Grand Prix','Elige a Sonic, Tails o Knuckles.','08',False),
    ('race',142,32,'Velocidad sin perder el control.','Anillos, rivales, Spin Dash y un loop de 360°.','08',False),
]
enc=['-c:v','libx264','-preset','fast','-crf','19','-pix_fmt','yuv420p','-r','30','-video_track_timescale','15360','-an']
clips=[]; timeline=[];events=[];elapsed=0.
source_events=[(float(t),cue) for t,cue in (line.split('\t') for line in SOURCE.with_suffix('.events.tsv').read_text().splitlines())]
for name,start,duration,title,subtitle,num,cover in segments:
    print('Editing',name,flush=True)
    overlay=card(name,title,subtitle,number=num,cover=cover)
    out=BUILD/f'{name}.mp4'
    if cover:
        ff('-loop','1','-i',ROOT/'output/normal-map-gpu-on.png','-loop','1','-i',overlay,'-filter_complex',
           f'[0:v]crop=960:1080:480:0,pad=1920:1080:960:0:color=0x07182d[v];[v][1:v]overlay=0:0,fade=t=in:st=0:d=0.18,fade=t=out:st={duration-.18}:d=0.18,format=yuv420p[out]',
           '-map','[out]','-t',duration,*enc,out)
    else:
        viewport = 'crop=1920:900:0:0,scale=1632:765,pad=1920:1080:144:135:color=0x07182d,' if name in ('mario','race') else ''
        ff('-ss',start,'-i',SOURCE,'-loop','1','-i',overlay,'-filter_complex',
           f'[0:v]{viewport}setpts=PTS-STARTPTS[v];[v][1:v]overlay=0:0,fade=t=in:st=0:d=0.18,fade=t=out:st={duration-.18}:d=0.18,format=yuv420p[out]',
           '-map','[out]','-t',duration,*enc,out)
    clips.append(out);timeline.append({'start':elapsed,'duration':duration,'title':title,**({'source_image':'output/normal-map-gpu-on.png'} if cover else {'source_start':start})})
    if not cover:events += [(elapsed+t-start,cue) for t,cue in source_events if start<=t<start+duration]
    elapsed+=duration
# Side-by-side comparison uses actual renderer exports of the same scene.
print('Editing normal map comparison',flush=True)
overlay=card('normals','La luz también cuenta la historia.','Normal mapping: misma geometría, una superficie con más detalle.',number='09')
im=Image.open(overlay);d=ImageDraw.Draw(im)
text(d,(300,220),'SIN NORMAL MAPPING',28,GOLD);text(d,(1280,220),'CON NORMAL MAPPING',28,CYAN)
im.save(overlay)
out=BUILD/'normals.mp4';duration=6
ff('-loop','1','-i',ROOT/'output/normal-map-gpu-off.png','-loop','1','-i',ROOT/'output/normal-map-gpu-on.png','-loop','1','-i',overlay,
   '-filter_complex','[0:v]scale=960:540[a];[1:v]scale=960:540[b];[a][b]hstack,pad=1920:1080:0:270:color=0x07182d[v];[v][2:v]overlay=0:0,fade=t=in:st=0:d=0.18,fade=t=out:st=5.82:d=0.18,format=yuv420p[out]',
   '-map','[out]','-t',duration,*enc,out)
clips.append(out);timeline.append({'start':elapsed,'duration':duration,'title':'Normal mapping / comparación'});elapsed+=duration
# Closing card keeps the CPU evidence visible as a separate, uninterrupted recording.
outro=Image.new('RGBA',(W,H),NAVY);d=ImageDraw.Draw(outro)
for r,a in [(400,35),(320,55),(240,90)]:d.ellipse((1460-r,540-r,1460+r,540+r),outline=(87,237,215,a),width=3)
text(d,(120,164),'INSERT COIN / GALAXY',38,CYAN)
text(d,(116,268),'Hecho para explorar.',94)
text(d,(120,420),'Daniel Sandoval',57,GOLD)
text(d,(120,515),'Proyecto final de Gráficas por Computadora',34)
text(d,(120,663),'RUST  /  OBJ + MTL  /  TECLADO + DUALSENSE',28,CYAN)
text(d,(120,794),'Modo CPU propio + edición visual GPU',32)
text(d,(120,858),'Video CPU continuo e instrucciones disponibles en el repositorio.',27,'#b9d2e4')
text(d,(120,969),'github.com/K41S3RTK1/proyecto-final-insert-coin-galaxy',23,'#b9d2e4')
p=BUILD/'outro.png';outro.save(p);out=BUILD/'outro.mp4';duration=5
ff('-loop','1','-i',p,'-vf','fade=t=in:st=0:d=0.25,fade=t=out:st=4.4:d=0.6','-t',duration,*enc,out)
clips.append(out);timeline.append({'start':elapsed,'duration':duration,'title':'Daniel Sandoval / entrega'});elapsed+=duration
manifest=BUILD/'concat.txt';manifest.write_text(''.join(f"file '{p.name}'\n" for p in clips))
silent=BUILD/'presentation-silent.mp4'
ff('-f','concat','-safe','0','-i',manifest,'-c','copy','-movflags','+faststart',silent)
silent.with_suffix('.events.tsv').write_text(''.join(f'{t:.6f}\t{cue}\n' for t,cue in events))
run(sys.executable,ROOT/'tools/mix_delivery_audio.py',silent,ROOT/'output/Presentacion_Galaxy.mp4')
# Consistent listening level with true-peak limiting; preserve the encoded picture.
normalized=BUILD/'presentation-normalized.mp4'
ff('-i',ROOT/'output/Presentacion_Galaxy.mp4','-c:v','copy','-af','loudnorm=I=-18:TP=-1.5:LRA=9','-c:a','aac','-b:a','160k','-ar','44100','-ac','2','-movflags','+faststart',normalized)
normalized.replace(ROOT/'output/Presentacion_Galaxy.mp4')
(MEDIA/'chapters.json').write_text(json.dumps({'seconds':elapsed,'chapters':timeline},ensure_ascii=False,indent=2)+'\n')
# All README stills and animation are extracted from the real game/video.
ff('-ss',1.5,'-i',BUILD/'intro.mp4','-frames:v','1','-q:v','2',MEDIA/'cover.jpg')
for name,stamp in [('solar',15),('garden',51),('dojo',76),('runner',94),('mario',115),('race',159)]:
    ff('-ss',stamp,'-i',SOURCE,'-frames:v','1','-vf','crop=1920:765:0:135,scale=960:-2','-q:v','3',MEDIA/f'{name}.jpg')
ff('-ss',11,'-t',7,'-i',ROOT/'output/Presentacion_Galaxy.mp4','-filter_complex',
   'fps=9,scale=640:-1:flags=lanczos,split[a][b];[a]palettegen=max_colors=128[p];[b][p]paletteuse=dither=bayer',MEDIA/'flight.gif')
print(f'Presentation complete: {elapsed:.1f} seconds',flush=True)
