"""Bake four authored miniature worlds into OBJ/MTL, with near/far meshes.
All exterior details use a local tangent frame and rotate with the parent planet.
No downloaded meshes; run python3 tools/build_planets.py to regenerate.
"""
from asset_mesh import *
root=Path(__file__).resolve().parents[1]/'assets/models'
palette={
 'surface_garden':(.24,.55,.31),'surface_runner':(.98,.73,.17),'surface_mario':(.24,.55,.31),
 'paint_grass':(.32,.91,.06),'paint_moss':(.64,1.,.12),'paint_earth':(.48,.30,.19),
 'paint_sand':(.83,.68,.42),'paint_rock':(.36,.39,.43),'paint_ocean':(.02,.72,.91),
 'paint_cream':(.94,.85,.63),'paint_red':(.76,.20,.15),'paint_bark':(.54,.23,.06),
 'paint_leaf':(.12,.66,.08),'paint_leaf_light':(.55,.96,.06),'paint_white':(.96,.95,.85),
 'paint_blue':(.13,.35,.63),'paint_yellow':(.98,.73,.17),'paint_black':(.07,.09,.13),
 'city_wall':(.21,.19,.34),'terrain_rock':(.48,.40,.30),'paint_purple':(.21,.19,.34),'paint_roof':(.50,.14,.23),'paint_pipe':(.15,.65,.24),
 'paint_brick':(.66,.32,.17),'paint_grey':(.42,.49,.54),'metal':(.55,.64,.68),
 'checker_cliff':(.68,.30,.08),'water_blue':(.02,.72,.91),'paint_gold':(1.,.70,.03),
 'glow_cyan':(.29,.89,.94),'glow_pink':(.99,.30,.55),'glow_gold':(1.,.79,.24),
}
def radial(n,kind):
 x,y,z=n
 wav=math.sin(x*5+z*2)*math.cos(y*4-z*3)+.35*math.sin(z*11+x*3)
 return 4.92+(.20 if kind!='dojo' else .08)*wav+.15*max(0,y)
def globe(m,kind):
 rows=16 if m.low else 40;sides=32 if m.low else 80;start=len(m.v)
 def point(a,b):
  n=(math.sin(a)*math.cos(b),math.cos(a),math.sin(a)*math.sin(b));return mul(n,radial(n,kind))
 for j in range(rows+1):
  a=j*math.pi/rows
  for k in range(sides):
   b=k*math.tau/sides;p=point(a,b)
   na=cross(sub(point(a+.0005,b),point(a-.0005,b)),sub(point(a,b+.0005),point(a,b-.0005)))
   if dot(na,p)<0:na=mul(na,-1)
   m.vertex(p,na if dot(na,na)>1e-14 else norm(p))
 for j in range(rows):
  for k in range(sides):
   a=start+j*sides+k;b=start+j*sides+(k+1)%sides;c=a+sides;d=b+sides
   p=norm(m.v[a]);x,y,z=p;v=math.sin(x*5+z*2)*math.cos(y*4-z*3)
   if kind in ('dojo','race'):mat='surface_garden'
   else:mat='surface_'+kind
   if j>0:m.tri(a,b,c,mat)
   if j<rows-1:m.tri(b,d,c,mat)
def placements(count):
 for k in range(count):
  y=1-2*(k+.5)/count;a=k*2.399963;r=math.sqrt(1-y*y)
  yield k,(r*math.cos(a),y,r*math.sin(a))
def mushroom(m,c,scale=1):
 x,y,z=c;m.cylinder((x,y,z),.18*scale,.7*scale,'paint_cream',.14*scale)
 m.sphere((x,y+.77*scale,z),(.65*scale,.30*scale,.65*scale),'paint_red')
 for k in range(5):
  a=k*math.tau/5;m.sphere((x+.36*math.cos(a)*scale,y+.98*scale,z+.36*math.sin(a)*scale),(.11*scale,.035*scale,.11*scale),'paint_cream',4,8)
def tree(m,scale=1):
 m.cylinder((0,0,0),.24*scale,1.6*scale,'paint_bark',.14*scale)
 for x,y,z,s in [(0,1.9,0,.95),(-.5,1.6,.2,.68),(.5,1.7,-.2,.72),(0,2.45,.1,.65)]:m.sphere((x*scale,y*scale,z*scale),(s*scale,s*.8*scale,s*scale),'paint_leaf_light' if y>2 else 'paint_leaf')
def ring(m,c,r=.48,tube=.095,mat='paint_gold'):
 sides=20 if m.low else 40;tubes=6 if m.low else 10;start=len(m.v)
 for j in range(sides):
  a=j*math.tau/sides
  for k in range(tubes):
   b=k*math.tau/tubes;n=(math.cos(a)*math.cos(b),math.sin(a)*math.cos(b),math.sin(b))
   m.vertex(add(c,add((r*math.cos(a),r*math.sin(a),0),mul(n,tube))),n)
 for j in range(sides):
  for k in range(tubes):
   a=start+j*tubes+k;b=start+((j+1)%sides)*tubes+k;cc=start+j*tubes+(k+1)%tubes;d=start+((j+1)%sides)*tubes+(k+1)%tubes
   m.tri(a,b,cc,mat);m.tri(b,d,cc,mat)
def palm(m,scale=1):
 for k in range(8):
  y=k*.26*scale;x=.06*k*k/8*scale
  m.cylinder((x,y,0),(.14-k*.006)*scale,.27*scale,'paint_bark')
  m.cylinder((x,y,0),(.15-k*.006)*scale,.055*scale,'paint_sand')
 top=(.37*scale,2.15*scale,0)
 for k in range(7):
  a=k*math.tau/7;along=(math.cos(a),0,math.sin(a));side=(-math.sin(a),0,math.cos(a))
  mid=add(top,add(mul(along,.85*scale),(0,.38*scale,0)));tip=add(top,add(mul(along,1.75*scale),(0,-.28*scale,0)))
  pts=[top,add(mid,mul(side,.32*scale)),tip,add(mid,mul(side,-.32*scale))]
  m.polygon(pts,'paint_leaf_light' if k%2 else 'paint_grass');m.polygon(list(reversed(pts)),'paint_leaf')
 for x in [-.14,.12]:m.sphere(add(top,(x*scale,-.14*scale,.1*scale)),(.18*scale,)*3,'paint_bark',4,8)
def hill_details(m,kind):
 for k,n in placements(13):
  if n[1]<-.65:continue
  with m.surface(n,radial(n,kind)-.1,f'green_hill_island_{k}'):
   m.box((0,.14,0),(1.5,.6,1.2),'checker_cliff',.10)
   m.box((0,.46,0),(1.6,.12,1.3),'paint_grass',.10)
   if k%2==0:palm(m,.85)
   else:
    for x in [-.45,0,.45]:ring(m,(x,.95,0),.19,.045)
 # Sculpted loop, checker terrain with a bright grass running edge.
 with m.surface((-.78,.5,.6),5.05,'green_hill_loop'):
  ring(m,(0,1.3,0),1.25,.23,'checker_cliff')
  ring(m,(0,1.3,.17),1.25,.085,'paint_grass')
  for x in [-.6,0,.6]:ring(m,(x,2.9,.03),.20,.045)
 with m.surface((.70,.42,.65),5.,'turquoise_cascade'):
  m.box((0,.3,0),(1.6,.8,1.4),'checker_cliff')
  m.box((0,.73,0),(1.7,.1,1.5),'paint_grass')
  m.box((0,.78,.35),(.75,.05,.8),'water_blue',0)
  m.box((0,-.15,.76),(.75,1.85,.025),'water_blue',0)
  for x in [-.28,0,.28]:m.box((x,-.15,.78),(.035,1.8,.025),'glow_cyan',0)
def garden(m):
 for k,n in placements(18):
  with m.surface(n,radial(n,'garden')-.04,f'garden_detail_{k}'):
   if k%3==0:palm(m,.95)
   elif k%3==1:mushroom(m,(0,0,0),.75)
   else:m.sphere((0,.12,0),(.48,.30,.35),'terrain_rock',5,10)
 with m.surface((0,1,.1),5.,'great_tree'):
  palm(m,1.55)
  for side in [-1,1]:mushroom(m,(side*.9,0,.5),.55)
 # Floral relief on the front: intentional landmark and recognizable garden identity.
 with m.surface((0,.23,1),4.94,'flower_monument'):
  m.cylinder((0,0,0),.22,.8,'paint_bark')
  for k in range(10):
   a=k*math.tau/10;m.sphere((math.cos(a)*1.15,.5,math.sin(a)*1.15),(.48,.22,.48),'paint_red')
  m.sphere((0,.72,0),(.95,.18,.95),'paint_cream')
  for side in [-1,1]:
   m.sphere((side*.30,.91,-.18),(.20,.05,.29),'paint_white')
   m.sphere((side*.30,.96,-.18),(.085,.025,.17),'paint_black')
  m.sphere((0,.90,.42),(.38,.04,.18),'paint_black')
  for x in [-.22,0,.22]:m.sphere((x,.94,.36),(.08,.025,.07),'paint_cream',4,8)
def dojo(m):
 m.group='orbital_neon_belt';m.torus((0,0,0),5.06,.055,'glow_pink')
 for k,n in placements(24):
  if n[1]>.82:continue
  with m.surface(n,4.94,f'city_tower_{k}'):
   h=.65+(k%5)*.25;m.box((0,h/2,0),(.55,h,.55),'paint_purple')
   m.box((0,h+.04,0),(.65,.09,.65),'metal')
   for y in range(1,int(h/.20)):
    for x in [-.15,.15]:m.box((x,y*.20,.282),(.08,.06,.025),'glow_cyan',0)
 with m.surface((0,1,.08),5.,'neon_pagoda'):
  m.box((0,.18,0),(2.9,.36,2.4),'paint_grey')
  for level in range(3):
   w=2.4-level*.45;y=.36+level*.7
   m.box((0,y+.24,0),(w*.7,.48,w*.6),'paint_cream')
   m.box((0,y+.53,0),(w,.16,w*.9),'paint_roof')
   m.box((0,y+.65,0),(w*.7,.09,w*.65),'paint_roof')
   for side in [-1,1]:m.box((side*w*.46,y+.62,0),(.13,.26,w*.9),'paint_roof')
  m.cylinder((0,2.4,0),.07,.6,'glow_gold',.02)
 with m.surface((0,.2,1),5.,'dojo_gate'):
  for side in [-1,1]:m.box((side*.95,.75,0),(.18,1.5,.22),'paint_roof')
  m.box((0,1.52,0),(2.5,.18,.36),'paint_roof');m.box((0,1.20,0),(2.15,.12,.25),'glow_pink')
def runner(m):
 m.group='railway';m.torus((0,0,0),5.01,.25,'paint_black')
 for y in [-.19,.19]:m.torus((0,y,0),5.20,.035,'metal')
 for k in range(40 if not m.low else 20):
  a=k*math.tau/(40 if not m.low else 20);n=(math.cos(a),0,math.sin(a))
  with m.surface(n,5.19,'railway_sleepers'):m.box((0,0,0),(.07,.05,.55),'paint_grey',0)
 for k,n in placements(16):
  if abs(n[1])<.3 or n[1]>.85:continue
  with m.surface(n,radial(n,'runner'),f'city_{k}'):
   h=.7+(k%3)*.35;m.box((0,h/2,0),(.65,h,.65),'paint_blue' if k%2 else 'paint_cream')
   m.box((0,h,0),(.75,.10,.75),'paint_red')
   for dx in [-.17,.17]:m.box((dx,h*.6,.34),(.12,.20,.025),'paint_black',0)
 with m.surface((0,1,.12),5.,'minion_tower'):
  m.sphere((0,1.,0),(.85,1.2,.72),'paint_yellow')
  m.sphere((0,.35,.04),(.85,.55,.72),'paint_blue')
  for x in [-.36,.36]:
   m.sphere((x,1.32,.69),(.36,.38,.16),'metal');m.sphere((x,1.32,.82),(.27,.28,.08),'paint_white')
   m.sphere((x,1.32,.89),(.11,.14,.035),'paint_bark');m.sphere((x,1.32,.92),(.055,.09,.02),'paint_black')
  m.sphere((0,.91,.71),(.25,.08,.06),'paint_black')
 with m.surface((0,.06,1),5.15,'subway_train'):
  m.box((0,.35,0),(1.7,.7,.7),'paint_blue');m.box((0,.75,0),(1.65,.12,.7),'paint_yellow')
  for x in [-.55,0,.55]:m.box((x,.48,.36),(.35,.28,.03),'paint_black')
  for x in [-.7,.7]:m.sphere((x,.15,.32),(.12,.12,.08),'metal',4,8)
def mario(m):
 for k,n in placements(14):
  if n[1]>.7:continue
  with m.surface(n,radial(n,'mario')-.05,f'mushroom_detail_{k}'):
   if k%3==0:
    m.cylinder((0,0,0),.38,.8,'paint_pipe');m.cylinder((0,.68,0),.45,.20,'paint_pipe');m.cylinder((0,.881,0),.34,.012,'paint_black')
   elif k%3==1:mushroom(m,(0,0,0),.65)
   else:m.box((0,.34,0),(.65,.65,.65),'paint_brick')
 with m.surface((0,1,.12),5.,'castle'):
  m.box((0,.7,0),(2.4,1.4,1.5),'paint_cream')
  for x in [-1.,1.]:
   m.cylinder((x,0,0),.46,2.,'paint_cream');m.cylinder((x,2.,0),.64,.9,'paint_red',.02)
   m.sphere((x,1.55,.45),(.12,.22,.025),'paint_black')
  m.box((0,1.65,0),(.85,.7,.9),'paint_cream');m.cylinder((0,2.,0),.65,.8,'paint_red',.02)
  m.sphere((0,.5,.76),(.28,.5,.025),'paint_bark')
  m.cylinder((0,2.65,0),.035,.75,'metal')
  m.polygon([(0,3.4,0),(.65,3.15,0),(0,2.95,0)],'paint_red')
  if not m.low:
   for y in [.25,.6,.95,1.3]:
    for x in [-.8,-.4,0,.4,.8]:m.box((x,y,.76),(.36,.025,.025),'paint_sand',0)
 with m.surface((0,.2,1),5.03,'question_blocks'):
  for k in [-1,0,1]:
   m.box((k*.85,.35,0),(.72,.72,.72),'paint_yellow')
   # Raised question marks on the outward-facing top surfaces.
   for x,z,w,d in [(0,-.16,.22,.07),(.13,-.06,.07,.16),(.06,.04,.15,.07),(0,.12,.07,.10),(0,.26,.07,.06)]:m.box((k*.85+x,.73,z),(w,.04,d),'paint_white',.01)
def race(m):
 m.group='race_circuit';m.torus((0,0,0),5.08,.32,'paint_blue')
 for y in [-.28,.28]:m.torus((0,y,0),5.10,.05,'paint_gold')
 with m.surface((0,1,.12),5.,'finish_line'):
  m.box((0,.12,0),(3.4,.24,2.),'paint_blue')
  for x in [-1.4,1.4]:m.cylinder((x,0,0),.09,2.8,'paint_white')
  for k in range(8):
   for j in range(2):m.box((-1.25+k*.35,2.55+j*.22,0),(.35,.22,.08),'paint_white' if (k+j)%2 else 'paint_black',0)
  ring(m,(0,1.4,.10),.82,.17)
 with m.surface((0,.18,1),5.,'three_heroes'):
  for k,mat in enumerate(['paint_blue','paint_yellow','paint_red']):
   m.sphere(((k-1)*.85,.5,0),(.33,.5,.33),mat)
   ring(m,((k-1)*.85,1.25,0),.24,.055)
for low in [False,True]:
 for kind,fn in [('garden',garden),('dojo',dojo),('runner',runner),('mario',mario),('race',race)]:
  m=Mesh(low);globe(m,kind);fn(m);hill_details(m,kind);m.save(root/f'{kind}{"-low" if low else ""}.obj',palette)

# Reusable scene props use the same authored OBJ pipeline inside the minigames.
m=Mesh();palm(m,1.);m.save(root/'hill-palm.obj',palette)
m=Mesh();ring(m,(0,1.5,0),1.4,.24,'checker_cliff');ring(m,(0,1.5,.19),1.4,.075,'paint_grass');m.save(root/'hill-loop.obj',palette)
