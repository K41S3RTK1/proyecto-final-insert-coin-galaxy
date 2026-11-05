"""Small offline OBJ modeler. Geometry and normals are baked into portable files."""
from pathlib import Path
from contextlib import contextmanager
import math

def add(a,b): return tuple(x+y for x,y in zip(a,b))
def mul(a,k): return tuple(x*k for x in a)
def sub(a,b): return add(a,mul(b,-1))
def dot(a,b): return sum(x*y for x,y in zip(a,b))
def cross(a,b): return (a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0])
def norm(a): return mul(a,1/max(math.sqrt(dot(a,a)),1e-12))
def write_obj(path, vertices, faces, materials, normals=None, groups=None):
    path=Path(path);path.parent.mkdir(parents=True,exist_ok=True)
    if normals is None:
        normals=[(0.,0.,0.) for _ in vertices]
        for _,ids in faces:
            n=cross(sub(vertices[ids[1]],vertices[ids[0]]),sub(vertices[ids[2]],vertices[ids[0]]))
            for i in ids:normals[i]=add(normals[i],n)
        normals=[norm(n) if dot(n,n)>1e-18 else (0.,1.,0.) for n in normals]
    with path.open('w') as f:
        f.write('# Original editable model / Insert Coin Galaxy\nmtllib '+path.with_suffix('.mtl').name+'\n')
        for v in vertices:f.write('v %.6f %.6f %.6f\n'%v)
        for n in normals:f.write('vn %.6f %.6f %.6f\n'%n)
        previous=None;prevgroup=None
        for k,(mat,ids) in enumerate(faces):
            if dot(cross(sub(vertices[ids[1]],vertices[ids[0]]),sub(vertices[ids[2]],vertices[ids[0]])),cross(sub(vertices[ids[1]],vertices[ids[0]]),sub(vertices[ids[2]],vertices[ids[0]])))<1e-18:continue
            group=groups[k] if groups else 'hull'
            if group!=prevgroup:f.write('g '+group+'\n');prevgroup=group
            if mat!=previous:f.write('usemtl '+mat+'\n');previous=mat
            f.write('f '+' '.join(f'{i+1}//{i+1}' for i in ids)+'\n')
    with path.with_suffix('.mtl').open('w') as f:
        for mat,c in materials.items():
            f.write('newmtl %s\nKd %.4f %.4f %.4f\n'%(mat,*c))
            f.write('Ks 0.25 0.25 0.25\nNs 48\n')
            if mat.startswith('glow_') or mat in ('engine','redlight'):f.write('Ke %.4f %.4f %.4f\n'%c)
    print(path.name,len(faces),'triangles')
class Mesh:
    def __init__(self,low=False):
        self.v=[];self.n=[];self.f=[];self.groups=[];self.group='surface';self.low=low
        self.frame=((1,0,0),(0,1,0),(0,0,1),(0,0,0))
    def vector(self,p):
        a,b,c,_=self.frame;return add(add(mul(a,p[0]),mul(b,p[1])),mul(c,p[2]))
    def vertex(self,p,n):
        self.v.append(add(self.vector(p),self.frame[3]));self.n.append(norm(self.vector(n)));return len(self.v)-1
    def tri(self,a,b,c,mat):self.f.append((mat,(a,b,c)));self.groups.append(self.group)
    def polygon(self,points,mat):
        clean=[]
        for p in points:
            if not clean or dot(sub(p,clean[-1]),sub(p,clean[-1]))>1e-18:clean.append(p)
        if len(clean)>1 and dot(sub(clean[0],clean[-1]),sub(clean[0],clean[-1]))<1e-18:clean.pop()
        if len(clean)<3:return
        points=clean
        n=(0,0,0)
        for k in range(1,len(points)-1):
            n=cross(sub(points[k],points[0]),sub(points[k+1],points[0]))
            if dot(n,n)>1e-18:break
        if dot(n,n)<1e-18:return
        n=norm(n)
        ids=[self.vertex(p,n) for p in points]
        for k in range(1,len(ids)-1):self.tri(ids[0],ids[k],ids[k+1],mat)
    @contextmanager
    def surface(self,n,r,group):
        old=self.frame;oldgroup=self.group;n=norm(n)
        right=norm(cross((0,1,0) if abs(n[1])<.95 else (0,0,-1),n));forward=cross(right,n)
        self.frame=(right,n,forward,mul(n,r));self.group=group
        yield
        self.frame=old;self.group=oldgroup
    def sphere(self,c,scale,mat,rings=None,sides=None):
        rings=rings or (6 if self.low else 12);sides=sides or rings*2
        start=len(self.v)
        for j in range(rings+1):
            a=math.pi*j/rings
            for k in range(sides):
                b=math.tau*k/sides;n=(math.sin(a)*math.cos(b),math.cos(a),math.sin(a)*math.sin(b))
                self.vertex(add(c,tuple(n[i]*scale[i] for i in range(3))),tuple(n[i]/scale[i] for i in range(3)))
        for j in range(rings):
            for k in range(sides):
                a=start+j*sides+k;b=start+j*sides+(k+1)%sides;c1=a+sides;d=b+sides
                if j>0:self.tri(a,b,c1,mat)
                if j<rings-1:self.tri(b,d,c1,mat)
    def box(self,c,size,mat,bevel=.06):
        x,y,z=c;a,b,d=[v/2 for v in size];t=min(a,d,bevel)
        poly=[(-a+t,-d),(a-t,-d),(a,-d+t),(a,d-t),(a-t,d),(-a+t,d),(-a,d-t),(-a,-d+t)]
        self.polygon([(x+xx,y+b,z+zz) for xx,zz in reversed(poly)],mat)
        self.polygon([(x+xx,y-b,z+zz) for xx,zz in poly],mat)
        for k,(xx,zz) in enumerate(poly):
            u,v=poly[(k+1)%len(poly)];self.polygon([(x+xx,y-b,z+zz),(x+xx,y+b,z+zz),(x+u,y+b,z+v),(x+u,y-b,z+v)],mat)
    def cylinder(self,c,r,h,mat,top=None):
        x,y,z=c;side=12 if self.low else 32;top=r if top is None else top
        start=len(self.v)
        for j,rad in enumerate([r,top]):
            for k in range(side):
                a=k*math.tau/side;n=(math.cos(a),(r-top)/h,math.sin(a))
                self.vertex((x+rad*math.cos(a),y+j*h,z+rad*math.sin(a)),n)
        for k in range(side):
            a=start+k;b=start+(k+1)%side;self.tri(a,a+side,b,mat);self.tri(b,a+side,b+side,mat)
        for j,rad in enumerate([r,top]):
            pts=[(x+rad*math.cos(k*math.tau/side),y+j*h,z+rad*math.sin(k*math.tau/side)) for k in range(side)]
            self.polygon(list(reversed(pts)) if j else pts,mat)
    def torus(self,c,major,minor,mat):
        n=24 if self.low else 64;tubes=6 if self.low else 10;start=len(self.v)
        for j in range(n):
            a=j*math.tau/n
            for k in range(tubes):
                b=k*math.tau/tubes;nrm=(math.cos(a)*math.cos(b),math.sin(b),math.sin(a)*math.cos(b))
                self.vertex(add(c,add((major*math.cos(a),0,major*math.sin(a)),mul(nrm,minor))),nrm)
        for j in range(n):
            for k in range(tubes):
                a=start+j*tubes+k;b=start+((j+1)%n)*tubes+k;c1=start+j*tubes+(k+1)%tubes;d=start+((j+1)%n)*tubes+(k+1)%tubes
                self.tri(a,c1,b,mat);self.tri(b,c1,d,mat)
    def save(self,path,materials):write_obj(path,self.v,self.f,materials,self.n,self.groups)
