# 常见问题解决

**GOAL.md 使用中的常见问题及解决方案**

---

## 评分脚本问题

### Q: 脚本报错 "Permission denied"

**原因**: 脚本没有执行权限

**解决**:
```bash
chmod +x scripts/score.sh
```

---

### Q: 分数不一致

**原因**: 脚本中有随机元素或外部依赖

**解决**:
1. 检查脚本中是否有 `$RANDOM` 或类似随机元素
2. 确保 `set -uo pipefail` 已启用
3. 确保没有依赖外部网络请求

```bash
# 检查随机元素
grep -n 'RANDOM' scripts/score.sh
```

---

### Q: 脚本超时

**原因**: 某个检查耗时过长

**解决**:
```bash
# 添加超时
TIMEOUT=60
TEST_OUTPUT=$(timeout $TIMEOUT cargo test 2>&1 || true)
```

---

### Q: JSON 输出无效

**原因**: 输出中包含特殊字符或换行符

**解决**:
```bash
# 清理特殊字符
escape_json() {
    echo "$1" | sed 's/\\/\\\\/g; s/"/\\"/g; s/\n//g'
}
```

---

## 迭代日志问题

### Q: 如何分析迭代日志？

**方法**:
```python
import json

with open('iterations.jsonl') as f:
    data = [json.loads(line) for line in f]

total = sum(d['after'] - d['before'] for d in data)
kept = sum(1 for d in data if d['result'] == 'kept')

print(f"Total improvements: {total}")
print(f"Kept: {kept}")
```

---

### Q: 迭代日志格式错误

**原因**: JSON 格式不正确

**解决**:
```bash
# 验证 JSONL 格式
while IFS= read -r line; do
    echo "$line" | python3 -m json.tool > /dev/null && echo "OK" || echo "ERROR"
done < iterations.jsonl
```

---

## GOAL.md 问题

### Q: Agent 不遵守 GOAL.md

**原因**: Agent 没有读取 GOAL.md

**解决**:
1. 确保 GOAL.md 在项目根目录
2. 添加 CLAUDE.md 明确指示 Agent 读取 GOAL.md

```markdown
# CLAUDE.md

## 当前目标

使用 GOAL.md 持续改进代码质量。

运行 `./scripts/score.sh` 查看当前分数。
```

---

### Q: 分数达到上限但还想改进

**原因**: 评分脚本的最大分值设置过低

**解决**:
1. 增加新组件到评分脚本
2. 提高现有组件的分值
3. 添加更细粒度的评分

---

### Q: Action Catalog 中的行动不起作用

**原因**: 行动描述不够具体

**解决**:
1. 明确每个行动的具体执行方法
2. 添加验证步骤确认行动生效
3. 更新 Action Catalog 记录实际效果

---

## Agent 协作问题

### Q: 多个 Agent 同时运行冲突

**原因**: 没有锁机制

**解决**:
```bash
LOCK_FILE=".goal-lock"
if [[ -f "$LOCK_FILE" ]]; then
    echo "Another agent is running"
    exit 1
fi

trap "rm -f $LOCK_FILE" EXIT
touch "$LOCK_FILE"
```

---

### Q: Agent 陷入循环

**原因**: Action Catalog 不完整或停止条件不明确

**解决**:
1. 添加明确的停止条件
2. 扩展 Action Catalog
3. 使用 Supervised 模式

```markdown
## Stopping Conditions

Stop when ANY of:
- Score reaches 100/100
- 10 consecutive iterations with no improvement
- 30 iterations completed
```

---

## 性能问题

### Q: 评分脚本太慢

**原因**: 执行了不必要的操作

**解决**:
1. 使用 `--no-run` 替代完整测试
2. 并行执行独立检查
3. 添加缓存

```bash
# 并行
./check_format.sh &
./check_clippy.sh &
wait
```

---

### Q: 占用太多磁盘空间

**原因**: 迭代日志无限增长

**解决**:
1. 定期归档旧日志
2. 设置最大迭代次数
3. 压缩日志文件

```bash
# 归档
if [[ $(wc -l < iterations.jsonl) -gt 1000 ]]; then
    gzip iterations.jsonl
    mv iterations.jsonl.gz "iterations_$(date +%Y%m).jsonl.gz"
    > iterations.jsonl
fi
```

---

## 集成问题

### Q: 如何与 CI/CD 集成？

**GitHub Actions 示例**:

```yaml
name: GOAL.md Score Check
on: [push, pull_request]

jobs:
  score:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-rust@v1
      - name: Run Score
        run: |
          chmod +x scripts/score.sh
          ./scripts/score.sh --json > score.json
      - name: Check Score
        run: |
          SCORE=$(cat score.json | jq '.total')
          if [[ $SCORE -lt 80 ]]; then
            exit 1
          fi
```

---

## 其他问题

### Q: 如何回滚到某个分数？

**方法**:
```bash
# 1. 找到目标分数对应的 commit
git log --oneline

# 2. 切换到目标 commit
git checkout <commit-hash>

# 3. 验证分数
./scripts/score.sh
```

---

### Q: 如何重置 GOAL.md？

**方法**:
```bash
# 1. 删除迭代日志
> iterations.jsonl

# 2. 重新运行基线
./scripts/score.sh > baseline.txt
```

---

## 获得帮助

如果遇到未列出的问题：

1. 查看 [examples/](../../examples/) 中的完整示例
2. 参考 [template/GOAL.md](../../template/GOAL.md)
3. 查看 [GitHub Issues](https://github.com/gyc567/AutoHarness/issues)
