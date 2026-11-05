"""Original synthesized effects, 22050 Hz mono PCM. No external recordings."""
from pathlib import Path
import math, wave, struct, random
root=Path(__file__).resolve().parents[1]/'assets/audio';root.mkdir(parents=True,exist_ok=True)
rate=22050
specs={'coin':(.13,1100,1700),'jump':(.20,260,650),'shot':(.12,920,250),'hit':(.15,170,60),'hurt':(.35,120,45),'win':(1.1,440,880),'lose':(.8,330,90),'portal':(.9,160,1000),'count':(.11,660,660),'warning':(.23,380,520),'phase':(.6,180,700),'block':(.16,950,380),'dodge':(.16,300,100),'punch':(.13,190,65),'engine':(1.,55,55)}
for name,(duration,start,end) in specs.items():
    rng=random.Random(2026);samples=[];phase=0.
    for index in range(round(duration*rate)):
        t=index/rate;u=t/duration;freq=start+(end-start)*u
        if name=='win':freq=[440,554.37,659.25,880][min(3,int(u*4))]
        phase+=math.tau*freq/rate
        envelope=min(1.,t/.008)*min(1.,(duration-t)/.035)*(1.-u*.55)
        signal=math.sin(phase)*.70+math.sin(phase*2)*.18
        if name in ('hit','hurt','punch','dodge'):signal=signal*.45+(rng.random()*2-1)*.45
        if name=='engine':signal=math.sin(math.tau*55*t)*.6+math.sin(math.tau*110*t)*.2;envelope=.28
        samples.append(round(max(-1.,min(1.,signal*envelope*.48))*32767))
    with wave.open(str(root/(name+'.wav')),'wb') as f:f.setparams((1,2,rate,0,'NONE','not compressed'));f.writeframes(struct.pack('<'+'h'*len(samples),*samples))
    assert max(abs(s) for s in samples)>100
print(f'{len(specs)} original WAV effects generated')
