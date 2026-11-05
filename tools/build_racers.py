"""Authored Sonic, Tails and Knuckles fan models, built from the supplied references.
Smooth Wavefront OBJ/MTL parts with explicit pivots for in-engine animation.
No extracted commercial meshes or textures. Run with Python 3.
"""
from asset_mesh import *
root=Path(__file__).resolve().parents[1]/'assets/models'
colors={'paint_blue':(.025,.19,.91),'paint_orange':(1.,.60,.025),'paint_red':(.86,.025,.045),
 'paint_skin':(1.,.72,.40),'paint_white':(.98,.99,1.),'paint_black':(.018,.025,.045),
 'paint_green':(.12,.68,.16),'paint_purple':(.47,.12,.66),'paint_iris_blue':(.02,.57,.89),
 'paint_gold':(1.,.72,.04),'paint_sole':(.67,.74,.81),'paint_darkred':(.48,.015,.025),
 'paint_ear':(1.,.79,.58),'paint_brown':(.31,.12,.04)}

def export(m,path):
 write_obj(path,m.v,[(mat.replace('paint_','hero_'),ids) for mat,ids in m.f],{k.replace('paint_','hero_'):v for k,v in colors.items()},m.n,m.groups)
def ball(m,c,s,mat):m.sphere(c,s,mat,16,28)
def tube(m,points,radii,mat,sides=16):
 """Smooth tapered swept surface, suitable for quills, fingers and tails."""
 start=len(m.v)
 for j,p in enumerate(points):
  tangent=norm(sub(points[min(j+1,len(points)-1)],points[max(0,j-1)]))
  right=norm(cross(tangent,(0,1,0) if abs(tangent[1])<.9 else (1,0,0)));up=cross(right,tangent)
  slope=(radii[min(j+1,len(points)-1)]-radii[max(0,j-1)])/max(.001,math.sqrt(dot(sub(points[min(j+1,len(points)-1)],points[max(0,j-1)]),sub(points[min(j+1,len(points)-1)],points[max(0,j-1)]))))
  for k in range(sides):
   a=k*math.tau/sides;n=add(mul(right,math.cos(a)),mul(up,math.sin(a)))
   m.vertex(add(p,mul(n,max(.001,radii[j]))),sub(n,mul(tangent,slope)))
 for j in range(len(points)-1):
  for k in range(sides):
   a=start+j*sides+k;b=start+j*sides+(k+1)%sides;c=a+sides;d=b+sides
   m.tri(a,b,c,mat);m.tri(b,d,c,mat)
def spike(m,a,b,c,r,mat):
 pts=[];rs=[]
 for j in range(9):
  t=j/8;pts.append(add(add(mul(a,(1-t)**2),mul(b,2*t*(1-t))),mul(c,t*t)));rs.append(r*(1-t)**.7)
 tube(m,pts,rs,mat)
def ear(m,side,kind):
 base=(side*.44,2.64,.03);tip=(side*(.72 if kind=='tails' else .60),3.23 if kind=='tails' else 3.05,-.015)
 mat='paint_orange' if kind=='tails' else 'paint_blue'
 # Thick ears have front/back faces and a smaller inset patch.
 points=[(side*.20,2.60,.16),(side*.72,2.61,.12),tip]
 m.polygon(points,mat);m.polygon(list(reversed(points)),mat)
 back=[(x,y,z-.16) for x,y,z in points]
 for k in range(3):m.polygon([points[k],back[k],back[(k+1)%3],points[(k+1)%3]],mat)
 ins=[(side*.33,2.66,.18),(side*.62,2.68,.15),(tip[0]*.93,tip[1]-.17,tip[2]+.055)]
 m.polygon(ins,'paint_white' if kind=='tails' else 'paint_ear');m.polygon(list(reversed(ins)),'paint_ear')
def core(kind):
 m=Mesh();mat={'sonic':'paint_blue','tails':'paint_orange','knuckles':'paint_red'}[kind]
 m.group='torso';ball(m,(0,1.29,0),(.34,.49,.29),mat)
 if kind!='knuckles':ball(m,(0,1.32,.265),(.25,.34,.055),'paint_white' if kind=='tails' else 'paint_skin')
 else:
  # Knuckles' white crescent collar.
  for x in range(-5,6):
   xx=x*.043;ball(m,(xx,1.54-.13*(1-(xx/.25)**2),.275),(.065,.045,.045),'paint_white')
 m.group='head';ball(m,(0,2.26,0),(.66,.66,.52),mat)
 if kind=='sonic':
  # Six swept-back quills define the silhouette in front, profile and chase views.
  for side in [-1,1]:
   for y,z,tx,ty,tz in [(2.66,-.12,1.00,2.35,-.70),(2.30,-.30,.93,1.75,-.90),(2.01,-.25,.70,1.33,-.73)]:
    spike(m,(side*.38,y,z),(side*.80,y-.08,z-.35),(side*tx,ty,tz),.34,mat)
  spike(m,(0,1.52,-.18),(0,1.2,-.65),(0,.95,-.75),.20,mat)
 elif kind=='knuckles':
  for side in [-1,1]:
   for k in range(3):
    spike(m,(side*(.36+k*.10),2.54-k*.12,-.13-k*.14),(side*(.76+k*.08),1.65,-.12-k*.27),(side*(.64+k*.13),1.07-k*.08,.06-k*.36),.23,mat)
  spike(m,(0,1.05,-.15),(0,.80,-.50),(.20,.62,-.72),.13,mat)
 else:
  for side in [-1,1]:
   for y in [2.03,2.16]:spike(m,(side*.36,y,.28),(side*.65,y-.03,.27),(side*.86,y-.06,.15),.17,'paint_white')
  for x in [-.20,0,.20]:spike(m,(x,2.70,.18),(x+.12,2.91,.43),(x+.20,2.60,.48),.14,mat)
  for side in [-1,1]:spike(m,(side*.16,1.60,.20),(side*.32,1.45,.32),(side*.40,1.33,.25),.15,'paint_white')
 if kind!='knuckles':
  for side in [-1,1]:ear(m,side,kind)
 # Continuous eye mask under the colored brow, wide cheeks and protruding nose.
 m.group='face'
 for side in [-1,1]:
  ball(m,(side*.245,2.35,.445),(.275,.385,.145),'paint_white')
  iris={'sonic':'paint_green','tails':'paint_iris_blue','knuckles':'paint_purple'}[kind]
  ball(m,(side*.21,2.30,.58),(.085,.175,.027),iris)
  ball(m,(side*.21,2.30,.604),(.040,.13,.018),'paint_black')
  ball(m,(side*.21-.024,2.37,.622),(.022,.045,.008),'paint_white')
 if kind=='knuckles':
  for side in [-1,1]:spike(m,(side*.58,2.54,.44),(side*.25,2.54,.62),(side*.05,2.39,.60),.105,mat)
 # V-shaped forehead makes Sonic/Knuckles' eye masks recognisable.
 if kind!='tails':spike(m,(0,2.79,.25),(0,2.59,.52),(0,2.30,.55),.22,mat)
 for side in [-1,1]:ball(m,(side*.22,2.045,.48),(.34,.20,.18),'paint_white' if kind=='tails' else 'paint_skin')
 ball(m,(0,2.13,.684),(.095,.075,.075),'paint_black')
 ball(m,(-.022,2.16,.744),(.026,.017,.012),'paint_white')
 # Small sculpted smile on the right cheek, not a large circular mouth.
 tube(m,[(.18,1.99,.638),(.28,2.00,.64),(.37,2.035,.60)],[.012,.012,.009],'paint_brown',8)
 m.v=[(x,y+.5,z) for x,y,z in m.v]
 return m

def arm(kind,side):
 m=Mesh();mat={'sonic':'paint_skin','tails':'paint_orange','knuckles':'paint_red'}[kind]
 m.group='arm';tube(m,[(0,0,0),(side*.10,-.20,.025),(side*.18,-.46,.06)],[.09,.08,.08],mat)
 ball(m,(side*.18,-.48,.06),(.17,.12,.15),'paint_white')
 m.group='glove';ball(m,(side*.21,-.68,.08),(.22,.23,.18),'paint_white')
 for k in range(3):
  x=side*.21+(k-1)*.085
  ball(m,(x,-.83,.12),(.065,.12,.10),'paint_white')
 ball(m,(side*.02,-.67,.17),(.10,.14,.12),'paint_white')
 if kind=='knuckles':
  for x in [-.09,.09]:spike(m,(side*.21+x,-.65,.20),(side*.21+x,-.61,.38),(side*.21+x,-.57,.51),.085,'paint_white')
 return m

def leg(kind,side):
 m=Mesh();mat={'sonic':'paint_blue','tails':'paint_orange','knuckles':'paint_red'}[kind]
 m.group='leg';tube(m,[(0,0,0),(0,-.27,.015),(0,-.52,.01)],[.105,.09,.085],mat)
 ankle='paint_green' if kind=='knuckles' else 'paint_white'
 ball(m,(0,-.51,.025),(.18,.12,.18),ankle)
 m.group='shoe';ball(m,(side*.035,-.71,.19),(.235,.18,.40),'paint_red')
 ball(m,(side*.035,-.825,.19),(.245,.055,.405),'paint_sole')
 if kind=='knuckles':
  ball(m,(side*.035,-.67,.27),(.24,.14,.14),'paint_gold')
  m.box((side*.035,-.54,.22),(.32,.045,.32),'paint_sole')
  for x in [-.09,.09]:
   for z in [.13,.23,.33]:ball(m,(x,-.51,z),(.025,.028,.025),'paint_black')
 else:
  ball(m,(side*.035,-.67,.23),(.24,.15,.10),'paint_white')
  if kind=='tails':ball(m,(side*.035,-.70,.45),(.22,.15,.15),'paint_white')
  else:
   m.box((side*.24,-.67,.20),(.04,.16,.19),'paint_gold',.01)
   m.box((side*.264,-.67,.20),(.01,.08,.11),'paint_white',.005)
 m.v=[(x,y-.5 if y<-.5 else y*2.,z) for x,y,z in m.v]
 return m

def tails_part():
 m=Mesh();m.group='twin_tails'
 for side in [-1,1]:
  pts=[];rs=[]
  for j in range(17):
   t=j/16;pts.append((side*(.12+.72*t),.8*t-.15,-.16-1.35*t));rs.append(.10+.31*math.sin(math.pi*t)**.7 if j<16 else .002)
  # Orange and white sections share the same profile at their boundary.
  tube(m,pts[:12],rs[:12],'paint_orange',20);tube(m,pts[11:],rs[11:],'paint_white',20)
 return m

for kind in ['sonic','tails','knuckles']:
 parts={'core':core(kind),'arm-l':arm(kind,-1),'arm-r':arm(kind,1),'leg-l':leg(kind,-1),'leg-r':leg(kind,1)}
 if kind=='tails':parts['tails']=tails_part()
 for part,m in parts.items():export(m,root/f'{kind}-{part}.obj')
 # Complete standing OBJ for external editing and classroom inspection.
 combined=Mesh()
 for part,m in parts.items():
  offset={'core':(0,0,0),'arm-l':(-.38,2.12,0),'arm-r':(.38,2.12,0),'leg-l':(-.19,1.38,0),'leg-r':(.19,1.38,0),'tails':(0,1.55,-.22)}[part]
  start=len(combined.v);combined.v.extend(add(v,offset) for v in m.v);combined.n.extend(m.n)
  combined.f.extend((mat,tuple(i+start for i in ids)) for mat,ids in m.f);combined.groups.extend(m.groups)
 export(combined,root/f'{kind}.obj')

for kind,mat in [('sonic','paint_blue'),('tails','paint_orange'),('knuckles','paint_red')]:
 m=Mesh();ball(m,(0,0,0),(.85,.85,.85),mat)
 for k in range(8):
  a=k*math.tau/8;spike(m,(math.cos(a)*.58,math.sin(a)*.58,-.25),(math.cos(a)*.8,math.sin(a)*.8,-.6),(math.cos(a)*.75,math.sin(a)*.75,-.95),.23,mat)
 if kind=='tails':ball(m,(0,0,.72),(.55,.55,.16),'paint_white')
 export(m,root/f'{kind}-ball.obj')
