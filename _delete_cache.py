import os, json

cache_path = os.path.expanduser("~/.bridge-ai/cache/model-directory.json")
if os.path.exists(cache_path):
    os.remove(cache_path)
    print(f"Deleted: {cache_path}")
else:
    print("File not found, will be created on next launch")