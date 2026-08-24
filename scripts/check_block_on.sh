#!/usr/bin/env bash
# 门禁：block_on 仅允许出现在 sync 门面文件内（blocking.rs）。
# async 路径禁止 block_on（防止在库代码中阻塞 runtime 线程）。
violations=$(grep -rn "block_on" crates/*/src --include="*.rs" | grep -v "blocking.rs" || true)
if [ -n "$violations" ]; then echo "违规 block_on："; echo "$violations"; exit 1; fi
echo "block_on 门禁通过"
