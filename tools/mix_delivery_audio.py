"""Original arcade backing track + game events recorded by --record (no third-party music)."""
from pathlib import Path
import array, math, subprocess, sys, wave, json

video=Path(sys.argv[1])
output=Path(sys.argv[2])
root=Path(__file__).resolve().parents[1]
info=json.loads(subprocess.check_output(['ffprobe','-v','error','-show_entries','format=duration','-of','json',str(video)]))
duration=float(info['format']['duration']); rate=22050; total=round(duration*rate)
audio=array.array('f',[0.])*total
# A new pentatonic phrase, with a four-chord bass and soft pulse lead.
notes=[76,79,83,79,81,76,74,71, 74,78,81,78,79,74,71,69,
       72,76,79,83,81,79,76,72, 74,78,81,86,83,81,78,74]
bass=[52,50,48,55]; beat=60/144
for k in range(math.ceil(duration/(beat/2))):
    start=round(k*beat/2*rate); length=min(round(beat/2*rate),total-start)
    freq=440*2**((notes[k%len(notes)]-69)/12)
    low=440*2**((bass[(k//8)%4]-69)/12)
    for j in range(length):
        t=j/rate; env=min(1.,t/.012)*max(0.,1-t/(beat/2))**.4
        # Smooth harmonics keep the melody gentle beneath game cues.
        lead=(math.sin(math.tau*freq*t)+.25*math.sin(math.tau*freq*2*t))*env*.055
        bottom=math.sin(math.tau*low*t)*math.exp(-t*5)*.07
        tick=math.sin(math.tau*(160-60*t)*t)*math.exp(-t*65)*(.06 if k%2==0 else .02)
        audio[start+j]=lead+bottom+tick
cache={}
for row in video.with_suffix('.events.tsv').read_text().splitlines():
    stamp,name=row.split('\t');start=round(float(stamp)*rate)
    if name not in cache:
        with wave.open(str(root/'assets/audio'/f'{name}.wav'),'rb') as f:
            assert f.getnchannels()==1 and f.getsampwidth()==2 and f.getframerate()==rate
            samples=array.array('h',f.readframes(f.getnframes()))
            if sys.byteorder!='little':samples.byteswap()
            cache[name]=samples
    gain=.32 if name=='shot' else .70
    for j,sample in enumerate(cache[name]):
        if start+j>=total:break
        audio[start+j]+=sample/32768*gain
# Fade in/out and gentle limiting prevent clicks and clipped effects.
pcm=array.array('h')
for i,v in enumerate(audio):
    fade=min(1.,i/(rate*.8),(total-i)/(rate*2.))
    pcm.append(int(math.tanh(v*1.25)*fade*29000))
if sys.byteorder!='little':pcm.byteswap()
audio_path=video.with_suffix('.mix.wav')
with wave.open(str(audio_path),'wb') as f:
    f.setnchannels(1);f.setsampwidth(2);f.setframerate(rate);f.writeframes(pcm.tobytes())
subprocess.run(['ffmpeg','-hide_banner','-loglevel','error','-y','-i',str(video),'-i',str(audio_path),'-c:v','copy','-c:a','aac','-b:a','160k','-ar','44100','-ac','2','-shortest','-movflags','+faststart',str(output)],check=True)
print(f'Video final: {output} ({duration:.1f} s, audio con eventos del juego + musica original)')
