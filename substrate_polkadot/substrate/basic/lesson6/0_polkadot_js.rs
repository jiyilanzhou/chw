
// extrinsic[eksˈtrɪnsɪk]adj.外在的,外来的
/*
0. Polkadot Js App 与 Polkadot Js Api
    a. App 是官方与 Substrate 互动的前端应用
    b. Api 是与 Substrate 互动的官方 JS API

1. 在 Ubuntu 上安装 Yarn
    (参考" https://yarn.bootcss.com/docs/install/#debian-stable ")
    a. 方式1：通过 Debian 软件包系统信息库安装 Yarn。首先需要配置存储库
        curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | sudo apt-key add -
        echo "deb https://dl.yarnpkg.com/debian/ stable main" | sudo tee /etc/apt/sources.list.d/yarn.list
    b. 方式2：在线安装
        sudo apt update && sudo apt install yarn
        // 如果使用 nvm 则可通过" sudo apt update && sudo apt install --no-install-recommends yarn "避免 node 安装
        //  nvm全名node.js version management，顾名思义是一个nodejs的版本管理工具。通过它可以安装和切换不同版本的 nodejs
    c. 校验
        yarn --version

2. Polkadot Js App 安装及运行
    a. 查看 yarn 版本
        root@ubuntu:~# yarn --version
            1.22.4
    b. 下载 " https://github.com/polkadot-js/apps "并进入 apps 查看 yarn 版本
        root@ubuntu:~/project/apps# yarn --version
            2.1.1       // 与系统安装的" 1.22.4 " yarn 版本不一致
        // (版本不一致)可注释掉" ../apps/.yarnrc.yml "文件内的" yarnPath "行即：
            root@ubuntu:~/project/apps# cat .yarnrc.yml
                // ...
                plugins:
                  - path: .yarn/plugins/@yarnpkg/plugin-interactive-tools.js
                    spec: "@yarnpkg/plugin-interactive-tools"
                    #yarnPath: .yarn/releases/yarn-2.1.1.js
            root@ubuntu:~/project/apps# yarn --version
                1.22.4
     c. (保持 apps 项目与系统 yarn 版本一致)安装
        root@ubuntu:~/project/apps# yarn install
        # 安装完毕启动
        root@ubuntu:~/project/apps# yarn start

    // 注 : 当然亦可直接使用网络版本 " https://polkadot.js.org/apps/#/explorer "
            (单位换算:" 1 unit = 1 * 10^12 pico ")

3. Polkadot Js Api
    官方文档" https://polkadot.js.org/api/ "
    a. 在需要使用 JS Api 相应的项目中添加 JS Api 库
        root@ubuntu:~/project/substrate-node-template# yarn add @polkadot/api
        root@ubuntu:~/project/substrate-node-template# yarn add @polkadot/keyring
        # 如果是在浏覧器内与前端交互，最好还添加
        root@ubuntu:~/project/substrate-node-template# yarn add @polkadot/extension-dapp
        root@ubuntu:~/project/substrate-node-template# yarn add @polkadot/ui-keyring
        root@ubuntu:~/project/substrate-node-template# yarn add @polkadot/ui-settings
    b. 在浏览器安装扩展插件
        (0). root@ubuntu:~# git clone https://github.com/polkadot-js/extension.git
        (1). FireFox -> 工具栏右侧下拉菜单 -> Add-ons -> Manage Your Extensions
                     -> Debug Add-ons -> Load Temporary Add-on...
                     -> 打开" /root/extension/packages/extension/manifest.json "
             (加载失败)失败信息
             There was an error during the temporary add-on installation.
             Error details
             Extension is invalid
             Reading manifest: Property "version" is required
            解决方案：可参阅" manifest.json "同目录下" package.json "的版本信息,而
                     后在" manifest.json "追加" version": "0.33.0-beta.6", "即可
    c. 启动结点
        root@ubuntu:~/project/substrate-node-template# ./target/release/node-template --dev

4. 网页前端
    Extrinsics : 一般交易
    Sudo : 超级用户权限
    Chain state : 链状态
    JavaScript 调试

5. 网页 JavaScript 调试
    a. Developer -> Javascript -> F12 (切换至浏览器调试模式)
    b. 在 Javascript 代码框粘贴如下代码后点击"运行"按钮
        debugger;
        const blockNum = 12;
        const blockHash = await api.rpc.chain.getBlockHash(blockNum);
        console.log(blockHash);
    c. 在 F12 调试模式下操作


*/
