import sys
sys.stdout.reconfigure(encoding='utf-8')

with open('src/pages/Models.tsx', 'r', encoding='utf-8') as f:
    content = f.read()

# Verify translations
checks = ['已配置模型', '添加模型', '模型供应商', '云端', '模型:', '来源:', '延迟:', '未测试', '删除', '编辑']
for c in checks:
    if c in content:
        print(f'OK: "{c}"')
    else:
        print(f'MISSING: "{c}"')