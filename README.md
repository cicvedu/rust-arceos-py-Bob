本阶段，需要完成 ArceOS 的相关实验步骤如下：

#### **一、实验环境配置**

实验需要的环境和ArceOS系统编译环境一致，具体可以在[实验环境配置 - ArceOS Tutorial Book (rcore-os.cn)](http://rcore-os.cn/arceos-tutorial-book/ch01-00.html)查看并配置。

#### **二、领取作业**

当完成实验环境配置后，点击老师分发的 GitHub Classroom 邀请链接，将在 `cicvedu` 组织下自动生成一个名为 `rust-arceos-[your_name]` 的项目。这个项目将成为你进行第二阶段实验的工作环境。

然后，在终端中执行以下命令，将项目克隆到本地：

```shell
git clone [repository_url]
```

将 `[repository_url]` 替换为`rust-arceos-[your_name]`项目的 URL。执行命令后，Git 将下载项目的代码到当前目录。

#### **三、切换到实验目录**

在终端中，使用 `cd` 命令来切换至实验目录：

```shell
cd /path/to/rust-arceos-[your_name]/exercises/cos_arceos/
```

将 `/path/to/` 替换为你实际的项目文件夹的路径。

#### **四、具体实验实现与验证**

根据每个实验的描述，在实验目录下修改或添加代码文件，实现所需功能，并执行`./verify aN`命令进行验证。

例如，你要进行 `a0` 实验：

1. 首先参考**实验目录对照表**找到a0对应的目录，按照main.rs的要求，去创建或修改相应的文件。

2. 修改完毕，执行以下命令进行验证（确保当前目录是 `/rust-arceos-[your_name]/exercises/cos_arceos/`）：

   ```
   ./verify a0
   ```

3. 观察输出，如果最后显示 `[ArceOS Tutorial]: A0 okay!` 的类似字符串，说明实验实现成功。

其他实验的实现与验证方式类似。

**实验目录对照表**

| 实验名称 | 对应目录                                                  |
| -------- | --------------------------------------------------------- |
| a0       | rust-arceos-[your_name]/exercises/cos_arceos/exercises/a0 |
| a1       | rust-arceos-[your_name]/exercises/cos_arceos/exercises/a1 |
| a2       | rust-arceos-[your_name]/exercises/cos_arcess/exercises/a2 |
| a3       | rust-arceos-[your_name]/exercises/cos_arcess/exercises/a3 |
| a4       | rust-arceos-[your_name]/exercises/cos_arcess/exercises/a4 |

#### **五、提交更改**

当你确认实验在本地验证通过后，可以将修改的内容提交到远程仓库。

1. 首先，使用以下命令添加更改的文件到暂存区：

   ```
   `git add [file1][file2]...`
   ```

2. 执行git commit命令进行提交并添加相应的提交消息：

   ```
   git commit -m "[your commit about experiment a0]"
   ```

3. 执行git status命令确认提交内容完整：

   ```
   git status -s
   ```

4. 最后，使用以下命令将更改推送到远程仓库：

   ```
   git push
   ```

#### **六、查看排行**

你可以访问 [Grading](https://cicvedu.github.io/rust-os-ranking/) 来查看自己的得分情况。请注意，排行榜的更新频率约为 20 分钟，所以提交后可能需要等待一段时间才能刷新并查看自己的得分。

希望这些步骤能够帮助你顺利进行 ArceOS 相关的 Rust 实验！

Enjoy yourself！


