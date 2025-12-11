CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"
echo "Using CARGO_HOME=$CARGO_HOME"

mkdir -p "$CARGO_HOME"

cat <<EOF > "$CARGO_HOME/config.toml"
[source.crates-io]
replace-with = 'mirror'

[source.mirror]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"
EOF

cargo build --release
