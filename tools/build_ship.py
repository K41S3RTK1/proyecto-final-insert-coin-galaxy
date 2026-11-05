"""Original disc freighter inspired by the student's previous Millennium Falcon.
Triangulated hull, forked bow, offset cockpit, paneling and engine arc.
No external meshes. Run Python 3 to rebuild OBJ/MTL.
"""
from pathlib import Path
import math
from asset_mesh import write_obj, cross, sub, dot
root=Path(__file__).resolve().parents[1]/'assets'
vertices=[];faces=[]
def face(points,mat):
    n=cross(sub(points[1],points[0]),sub(points[2],points[0]));c=tuple(sum(p[i] for p in points)/len(points) for i in range(3))
    radius=math.hypot(c[0],c[2])
    if 2.70<radius<2.86 and abs(n[1])<math.sqrt(dot(n,n))*.15 and n[0]*c[0]+n[2]*c[2]<0:points=list(reversed(points))
    start=len(vertices);vertices.extend(points)
    for k in range(1,len(points)-1):faces.append((mat,(start,start+k,start+k+1)))
def box(c,s,mat):
    x,y,z=c;a,b,d=[v/2 for v in s]
    p=[(x-a,y-b,z-d),(x+a,y-b,z-d),(x+a,y+b,z-d),(x-a,y+b,z-d),(x-a,y-b,z+d),(x+a,y-b,z+d),(x+a,y+b,z+d),(x-a,y+b,z+d)]
    for ids in [(0,3,2,1),(4,5,6,7),(0,4,7,3),(1,2,6,5),(3,7,6,2),(0,1,5,4)]:face([p[i] for i in ids],mat)
def ellipsoid(c,s,mat,rings=8,sides=16):
    start=len(vertices)
    for r in range(rings+1):
        a=math.pi*r/rings
        for k in range(sides+1):
            b=math.tau*k/sides;u=(math.sin(a)*math.cos(b),math.cos(a),math.sin(a)*math.sin(b))
            vertices.append(tuple(c[i]+s[i]*u[i] for i in range(3)))
    for r in range(rings):
        for k in range(sides):
            a=start+r*(sides+1)+k;b=a+sides+1
            faces.extend([(mat,(a,a+1,b)),(mat,(a+1,b+1,b))])
def extrusion(poly,lo,hi,mat):
    face([(x,hi,z) for x,z in reversed(poly)],mat)
    face([(x,lo,z) for x,z in poly],'dark')
    for i,(x,z) in enumerate(poly):
        xx,zz=poly[(i+1)%len(poly)]
        face([(x,lo,z),(xx,lo,zz),(xx,hi,zz),(x,hi,z)],'edge')
# A low, tapered saucer with separately colored radial armor panels.
slices=96
rings=[(0.,.64),(.65,.62),(1.15,.40),(2.30,.27),(2.66,.16),(2.80,.08)]
for ring in range(len(rings)-1):
    ra,ya=rings[ring];rb,yb=rings[ring+1]
    for k in range(slices):
        a=math.tau*k/slices;b=math.tau*(k+1)/slices
        mat='hull' if (k+ring*3)%7 else 'panel'
        if ring==2 and k in [34,35,49,50]:mat='red'
        face([(ra*math.cos(a),ya,ra*math.sin(a)),(ra*math.cos(b),ya,ra*math.sin(b)),(rb*math.cos(b),yb,rb*math.sin(b)),(rb*math.cos(a),yb,rb*math.sin(a))],mat)
        face([(ra*math.cos(a),-ya*.65-.15,ra*math.sin(a)),(rb*math.cos(a),-yb*.65-.15,rb*math.sin(a)),(rb*math.cos(b),-yb*.65-.15,rb*math.sin(b)),(ra*math.cos(b),-ya*.65-.15,ra*math.sin(b))],'belly')
for k in range(slices):
    a=math.tau*k/slices;b=math.tau*(k+1)/slices
    def point(angle,y,r=2.8):return (r*math.cos(angle),y,r*math.sin(angle))
    face([point(a,-.21),point(b,-.21),point(b,.08),point(a,.08)],'edge')
    if 3<=k<=28:
        face([point(a,-.15,2.815),point(b,-.15,2.815),point(b,.01,2.815),point(a,.01,2.815)],'engine')
# Long twin mandibles with a visible central slot.
for side in [-1,1]:
    poly=[(side*.36,-1.4),(side*1.45,-1.2),(side*1.36,-3.9),(side*.42,-4.1)]
    # Keep polygon winding consistent on both sides.
    if side>0:poly.reverse()
    extrusion(poly,-.23,.22,'hull')
    box((side*.82,.265,-2.9),(.38,.06,1.3),'panel')
    box((side*.82,.305,-3.0),(.16,.035,.55),'red')
    for z in [-1.8,-2.1,-2.4,-3.5]:box((side*.87,.29,z),(.70,.03,.035),'seam')
# Offset cockpit and corridor: same recognizable asymmetry as the old project.
box((2.34,.18,-.72),(1.85,.42,.62),'hull')
ellipsoid((3.12,.20,-1.30),(.43,.36,.93),'hull',16,32)
ellipsoid((3.12,.23,-1.83),(.37,.28,.43),'glass',16,32)
for x in [2.98,3.12,3.26]:box((x,.495,-1.75),(.035,.035,.52),'dark')
box((3.12,.46,-1.70),(.66,.055,.06),'hull')
# Raised gun turret, paired barrels and radar dish.
ellipsoid((0.,.65,-.15),(.46,.19,.46),'panel',6,16)
box((0.,.85,-.2),(.50,.18,.58),'hull')
for x in [-.13,.13]:box((x,.88,-.88),(.075,.075,1.02),'dark')
box((-.95,.65,-.55),(.12,.7,.12),'dark')
ellipsoid((-.95,1.02,-.58),(.48,.39,.09),'panel',14,24)
ellipsoid((-.95,1.02,-.69),(.36,.28,.025),'hull',6,12)
# Six cooling vents and fine machinery over the aft deck.
for x,z in [(-1.35,.7),(-.65,1.1),(.15,1.2),(.95,.9),(-.6,1.8),(.45,1.85)]:
    ellipsoid((x,.35,z),(.30,.07,.30),'dark',4,12)
    for dx in [-.15,-.05,.05,.15]:box((x+dx,.43,z),(.035,.04,.43),'panel')
for k in range(34):
    a=math.tau*k/34;r=1.9 if k%2 else 2.35
    box((r*math.cos(a),.31,r*math.sin(a)),(.24,.07,.14),'panel' if k%3 else 'seam')
# Small navigation lamps and layered rim details.
for side in [-1,1]:box((side*2.65,.15,.15),(.10,.08,.18),'redlight' if side<0 else 'engine')
# Raised segmented armor follows the saucer profile; gaps expose the dark substrate.
for k in range(48):
    a=math.tau*(k+.12)/48;b=math.tau*(k+.88)/48
    for inner,outer,yi,yo in [(1.22,1.84,.40,.335),(1.9,2.55,.33,.22)]:
        face([(inner*math.cos(a),yi,inner*math.sin(a)),(inner*math.cos(b),yi,inner*math.sin(b)),(outer*math.cos(b),yo,outer*math.sin(b)),(outer*math.cos(a),yo,outer*math.sin(a))], 'hull' if k%7 else 'panel')
# Recessed rear exhaust, radiator fins, rim service slots and tiny navigation lamps.
for k in range(96):
    a=math.tau*(k+.16)/96;b=math.tau*(k+.84)/96
    def rim(angle,y,r): return (r*math.cos(angle),y,r*math.sin(angle))
    if 6<k<43:
        face([rim(a,-.185,2.825),rim(b,-.185,2.825),rim(b,.04,2.825),rim(a,.04,2.825)],'dark')
        face([rim(a,-.13,2.832),rim(b,-.13,2.832),rim(b,-.015,2.832),rim(a,-.015,2.832)],'engine')
    else:
        face([rim(a,-.17,2.81),rim(b,-.17,2.81),rim(b,.015,2.81),rim(a,.015,2.81)],'panel' if k%3 else 'dark')
# Docking collars on both sides, bolts and landing-gear bay panels under the hull.
for side in [-1,1]:
    ellipsoid((side*2.65,-.02,0.),(.28,.24,.48),'panel',8,20)
    ellipsoid((side*2.87,-.02,0.),(.055,.16,.33),'dark',8,20)
    for z in [-.21,0,.21]:box((side*2.91,-.02,z),(.04,.23,.035),'hull')
    box((side*1.15,-.52,.55),(.55,.08,.9),'panel')
    box((side*.73,.32,-3.78),(.48,.04,.10),'dark')
    for z in [-2.5,-2.9,-3.3]:box((side*1.05,.26,z),(.06,.035,.05),'engine')
for x,z in [(-1.35,.7),(-.65,1.1),(.15,1.2),(.95,.9),(-.6,1.8),(.45,1.85)]:
    for k in range(12):
        a=k*math.tau/12
        ellipsoid((x+.28*math.cos(a),.445,z+.28*math.sin(a)),(.025,.018,.025),'hull',3,6)
materials={'hull':(.06,.27,.92),'panel':(.87,.96,1.),'belly':(.08,.16,.46),'edge':(.05,.12,.34),'dark':(.025,.06,.16),'seam':(.08,.17,.38),'red':(1.,.66,.02),'glass':(.04,.78,.95),'engine':(1.,.84,.18),'redlight':(1.,.24,.10)}
write_obj(root/'arcade-ship.obj',vertices,faces,materials)
