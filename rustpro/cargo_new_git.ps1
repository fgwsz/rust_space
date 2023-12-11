# 使用`cargo new {project name}`命令创建项目有时会无法生成`{project name}/.gitignore`文件
# 从而导致在`git push`的时候把`{project name}/target`文件夹也同步到git仓库
# 而使用`cargo new {project name} --vcs git`命令创建项目则会稳定生成`{project name}/.gitignore`文件
# 所以本脚本用来代替cargo new来创建项目,旨在简化cargo new --vcs git命令输入操作
$project_name=Read-Host -Prompt "input project name"
cargo new $project_name --vcs git
