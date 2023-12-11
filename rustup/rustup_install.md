# 1.设置环境变量
变量名：`RUSTUP_DIST_SERVER`  
变量值：`https://mirrors.tuna.tsinghua.edu.cn/rustup`  

变量名：`CARGO_HOME`  
变量值：`{自定义Rust安装文件夹}/.cargo` 注意这是个需要自己创建的文件夹  

变量名：`RUSTUP_HOME`  
变量值：`{自定义Rust安装文件夹}/.rustup` 注意这是个需要自己创建的文件夹  

然后在系统环境变量的`Path`变量中添加如下两行：
`%CARGO_HOME%`  
`%RUSTUP_HOME%`  

# 2.运行rustup-init.exe
输入`1`并回车，选择默认安装方式。  

# 3.配置cargo config
在`{自定义Rust安装文件夹}/.cargo/`下新建config文件，并键入如下内容:
```
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"

# 替换成你偏好的镜像源
replace-with = 'sjtu'

# 清华大学
[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

# 中国科学技术大学
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"

# 上海交通大学
[source.sjtu]
registry = "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index"

# rustcc社区
[source.rustcc]
registry = "git://crates.rustcc.cn/crates.io-index"
```
