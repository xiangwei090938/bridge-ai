import sys, json
sys.stdout.reconfigure(encoding='utf-8')

cache_path = r'C:\Users\Administrator\.bridge-ai\cache\model-directory.json'
try:
    with open(cache_path, 'r', encoding='utf-8') as f:
        data = json.load(f)
    providers = data.get('providers', [])
    print(f'Total providers: {len(providers)}')
    for p in providers:
        print(f'  - {p["name"]}')
except Exception as e:
    print(f'Error: {e}')