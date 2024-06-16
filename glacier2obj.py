import os
import bpy
import json
import mathutils
from mathutils import Euler
import math

path_to_obj_dir = os.path.join('D:\\workspace\\glacier2obj\\objects\\OBJ\\')
file_list = sorted(os.listdir(path_to_obj_dir))
obj_list = [item for item in file_list if item.endswith('.obj')]
path_to_prims_json = os.path.join('D:\\workspace\\glacier2obj\\prims.json');
f = open (path_to_prims_json, "r")
data = json.loads(f.read())
print("Printing hashes and transforms")
transforms = {}
for hash_and_entity in data['entities']:
    hash = hash_and_entity['primHash']
    entity = hash_and_entity['entity']
    transform_json = entity['transform']
    properties = entity['properties']
    scale = properties['m_PrimitiveScale']
    print(hash + ":")
    transform = {}
    transform["position"] = transform_json["position"]
    transform["rotate"] = transform_json["rotation"]
    transform["scale"] = scale["data"]
    print(json.dumps(transform))
    
    if (hash not in transforms):
        transforms[hash] = []
    transforms[hash].append(transform)
        
f.close()

for item in obj_list:
    path_to_file = os.path.join(path_to_obj_dir, item)
    hash = item[:-4]
    if hash not in transforms:
        continue
    for i in range(0, len(transforms[hash])):
        bpy.ops.import_scene.obj(filepath = path_to_file, use_split_objects=False, use_split_groups=False)
        transform = transforms[hash][i]
        p = transform["position"]
        r = transform["rotate"]
        s = transform["scale"]
        r["roll"] = math.pi * 2 - r["roll"]
        bpy.ops.transform.resize(value=(s["x"], s["y"], s["z"]), orient_type="LOCAL")
        bpy.ops.transform.rotate(value=r["yaw"], orient_axis='X', orient_type="LOCAL")
        bpy.ops.transform.rotate(value=r["pitch"], orient_axis='Y', orient_type="LOCAL")
        bpy.ops.transform.rotate(value=r["roll"], orient_axis='Z', orient_type="LOCAL")
        bpy.ops.transform.translate(value=(p["x"], p["y"], p["z"]), orient_type="LOCAL")
