

# 安装
dependence
需要先安装rust
```bash

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# To configure your current shell, run:
source $HOME/.cargo/env
git clone https://github.com/lihe2019/crash-tools.git
cargo build --release
```



# 使用
```bash
cd target/release
./crash-tools p 59400
./crash-tools 6 "{5112356, 9377040, 3281321984, 2867013992}"
./crash-tools 4 402657452
```
port 2280
ipv6 2402:4e00:1015:8f00:0:95c3:6829:e3aa
ipv4: 172.16.0.24
