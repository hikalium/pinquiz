#!/bin/bash
QNUM=$1
echo "## Q${QNUM}"
echo "### これをコンパイル・実行するとどうなる？"
echo '```'
cat examples/q${QNUM}.rs
echo '```'
echo "### こたえ"
echo '```'
echo $ cargo --quiet run --example q${QNUM}
cargo --quiet run --example q${QNUM}
echo '```'
echo "### 解説"
