import re
inp=open('/home/user/Shamble-Frontlines/.checkpoints/v2-decision-drafts/inputs/STOP-ANSWERS-mustafa-2026-11-03.md').read()
n=lambda s:re.sub(r'\s+',' ',s)
I=n(inp)
qs=["then the look proof test once its models exist","The preview hosting check may run alongside any of them.",
"scripting comes back only when a designed ability can't be built from data.","judge after 15 minutes of play.",
"yes, the asset research's export and slot-naming part runs before the look proof test. Route: Blender (Spline's skins and named slots are unproven).",
"yes, the asset research's export and slot-naming part runs before the look proof test. Route: Blender (Spline's skins and named slots are unproven). I make the test models with AI help in Blender; blockouts with a skin, named colour slots and LOOK_V2's proportions are enough."]
D=n(open('/home/user/Shamble-Frontlines/docs/SPIKE_PLAN.md').read())
for q in qs: print(q in I, ('"'+q+'"') in D, q[:60])
print(D.count('"'))
