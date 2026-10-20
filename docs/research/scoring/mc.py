import random, importlib.util, sys, io, contextlib
spec = importlib.util.spec_from_file_location("score", __import__("os").path.join(__import__("os").path.dirname(__file__), "score.py"))
m = importlib.util.module_from_spec(spec)
with contextlib.redirect_stdout(io.StringIO()):
    sys.argv=["x","none"]; spec.loader.exec_module(m)
random.seed(7)
N=20000
w=m.W["Balanced"]
top1={}; top3={}
base={oid:[(int(t[:-1]),t[-1]) for t in s.split()] for oid,_,s in m.O}
for _ in range(N):
    tot={}
    for oid,vals in base.items():
        v=[min(5,max(1,n+random.choice((-1,0,1)))) if t=="J" else n for n,t in vals]
        tot[oid]=sum(a*b for a,b in zip(w,v))/5
    r=sorted(tot,key=lambda k:-tot[k])
    top1[r[0]]=top1.get(r[0],0)+1
    for k in r[:3]: top3[k]=top3.get(k,0)+1
print("Perturb every J score by -1/0/+1 uniformly, Balanced weights, N=",N)
for k in sorted(top3,key=lambda k:-top3[k])[:8]:
    print(f"{k}: top1 {100*top1.get(k,0)/N:.0f}%  top3 {100*top3[k]/N:.0f}%")
