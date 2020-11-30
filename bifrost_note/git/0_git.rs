
/*
Git 常用命令
    git log 查看日志(历史提交记录等)
    git status 查看状态(红色表示文件修改)

0. 本地代码同步到远程仓库
    a. 在 github 上新建创建  new repository
    b. 本地仓库与远程仓库关联
        git remote add origin https://github.com/username/lib_name.git
        # remote 代表要操作的远程库(未指明则默认为" master ")
        # add 代表在本地添加一个从 github 上获取的远程库并取名为 origin (名称任意)
        // 相当于以键值对方式为" https://github.com/username/lib_name.git "取个别名
        即" alias "键对应" https://github.com/username/lib_name.git "值"
        // 具体可查看" .git/config "文件"
    c. 提交
        # 首次提交
        git push -u origin master
        // git push origin master
        # 本地修改文件后需再次 git add 及 git commit 才能 git push 到远程
        git add file
        git commit -m "comment"     // 提交到本地仓库
        // add 及 commit 可简化为一步:  git commit -a -m "commit" 但 add 的是所有已 modify 的文件
        git push alias master   // 再次提交到远程服务器

1. 项目分支：
    # 查看远程仓库分支(图" 0_远程分支.png ")
       浏览器-> github 项目地址 -> 点击 branches 即可查看所有分支
    # 查看本地分支 git branch
        git branch    (列举分支[绿色代表当前分支])
    # 创建本地分支 git branch branch_name
        git branch branch_name

2. git 创建分支命令
    a. # 创建分支
        git branch branch_name
    b. # 切换到分支
        git checkout branch_name        // 在新分支编辑并提交到本地仓库
    c. # 将 branch_name 提交到远程仓库
        git push origin branch_name     // 此时远程分支新增了 branch_name 分支
    d. # 删除本地分支(不能在本分支内删除自身分支:一般切换到master删除其余相应分支)
        git branch -d branch_name
        // 小写" d "代表安全删除(若分支内文件不同于或多于master则给出提示)
        // 大写" D " 代表强制删除
        git branch -D branch_name
       # 删除远程分支(注意 origin 后的空格不可省略)
        git push origin :branch_name
        // 因本地分支已删除即 push 空分支(即是删除远程相应分支)
        // 没有" : "即" git push origin branch_name "为上传分支
        // 有了" : "为即" git push origin :branch_name "为上传空分支(即删除)

问题：当远程分支发生更新时，再推送本地分支到远程则报错。(解决方案：先同步远程代码)

3. 同步远程代码(事先已拉取过)
    git pull origin master      // 合并式拉取(直接无脑合并：不推荐使用)

 */

/*
4. 若远程文件更新则无法 push (解决方案有两个:pull 及 fetch )
    a. pull : 直接拉取远程文件并合并(产生冲突亦需手动解决：不推荐使用)
              git pull = git fetch + git merge
    b. fetch : 从远程获取更新、比对差异(不会自动合并)

5. 使用 fetch 基本步骤
    a. 从远程分支 branch_name 获取更新
        git fetch origin branch_name
    b. 比对本地库和远程库(事先提交本地编辑后的代码即 commit -a -m "local_update" )
        git diff local_branch origin/branch_name
    c. 合并(有冲突则需进行手动解决)
        (0). 无冲突直接合并
              git merge origin/branch_name
        (1). 冲突解决
             # 选择基板
                " git checkout --ours 文件名  " // 使用本地版本
                或" git checkout --theirs 文件名  "//  使用远程版本
             # 重新 add 和 commit 后合并
                git commit -a -m "commit"
             # 合并
                git merge origin/branch_name
    d. 提交到远程
        git push origin/branch_name

*/
/*
# bifrost
Administrator@CHW MINGW64 /e/project/bifrost (chw)
$ git fetch origin develop
remote: Enumerating objects: 20, done.
remote: Counting objects: 100% (20/20), done.
remote: Compressing objects: 100% (2/2), done.
remote: Total 97 (delta 18), reused 19 (delta 18), pack-reused 77
Unpacking objects: 100% (97/97), done.
From https://github.com/bifrost-finance/bifrost
 * branch            develop    -> FETCH_HEAD
   e17ed7d..ae0f974  develop    -> origin/develop

# 查看哪些文件发生冲突
  git ls-files -s


 */

/*
git config user.name chw683
git config user.email 326756656@qq.com

# CLion 使用 git 快捷键   Alt + `
    1. commit  // 因 IDEA 将 add 与 commit 合并故无需 add

# git 的忽略文件(被忽略的文件不会被提交到仓库)
    a. " .gitignore "设置忽略的文件或目录
         在相应项目下编辑" .gitignore "文件
    b. 如" .gitignore "设置忽略" ./idea target/ "等,其内容如下
        .idea/
        target/
    c. " Alt + ` "再次" commit "则" .gitignore "内配置的文件或目录不再出现

 */

/*
VCS -> Get from Version Control (即" git clone ")
	配置 URL:" https://github.com/bifrost-finance/bifrost.git "

查看" annotation "条件：拉取项目前登录 github
	可查看" annotation "

尽量避免冲突策略：
    提交前更新
    常提交(完成小功能即提交)

 */