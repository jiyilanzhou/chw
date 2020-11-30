
/*
0. CentOS8
    a. 查看版本
        [chw@localhost ~]# cat /etc/redhat-release
        CentOS Linux release 8.2.2004 (Core)
    b. CentOS 切换 root 账户
        [chw@localhost ~]$ su root
        Password:
        [root@localhost chw]#

1. 修改 IP
    a. 修改 eth0 网卡  # 旧版本参考
       [root@centos ~]# vim /etc/sysconfig/network-scripts/ifcfg-eth0
        DEVICE=eth0
        ONBOOT=yes
        BOOTPROTO=static
        IPADDR=192.168.68.30
        NETMASK=255.255.255.0
        GATEWAY=192.168.68.1
    b.  修改 ens33 网卡  # 新版本
         # 查看网卡名
         [root@localhost ~]# nmcli d         # 尽量掌握 nmcli 命令的使用
             DEVICE      TYPE      STATE         CONNECTION
             virbr0      bridge    connected     virbr0
             ens33       ethernet  disconnected  --
         # 编辑网上配置
         [root@localhost ~]# vim /etc/sysconfig/network-scripts/ifcfg-ens33
         [root@localhost ~]# cat /etc/sysconfig/network-scripts/ifcfg-ens33
             TYPE=Ethernet
             PROXY_METHOD=none
             BROWSER_ONLY=no
             #BOOTPROTO=dhcp
             BOOTPROTO=static
             DEFROUTE=yes
             IPV4_FAILURE_FATAL=no
             IPV6INIT=yes
             IPV6_AUTOCONF=yes
             IPV6_DEFROUTE=yes
             IPV6_FAILURE_FATAL=no
             IPV6_ADDR_GEN_MODE=stable-privacy
             NAME=ens33
             UUID=c2e252ae-9c4e-4602-8d71-9ebc75fba2e8
             DEVICE=ens33
             #ONBOOT=no
             ONBOOT=yes

             IPADDR=192.168.68.6
             NETMASK=255.255.255.0
             GATEWAY=192.168.68.1
             DNS1=223.5.5.5
             PREFIX=24
        # 重启网卡
        [root@localhost ~]# ifup ens33
            Connection successfully activated ...

    c. (系统移植[即移动动网卡]时) CentOs 将 eth1 换为 eth0 :      # 旧版本参考
	   (0). 打开" /etc/udev/rules.d/70-persistent-net.rules "文件，将 eth0 的 Mac 地址换为 eth1 的。
	        即是将 eth0 的" ... ATTR{address}=="00:0c:29:8f:89:97" ... "更换成为" eth1 "的相应地址
	        " ... ATTR{address}=="00:0c:29:50:bd:17" ... "后将 eth1 相关信息(即"# PCI device ... "
	        注释及其后信息")删除
	   (1). 编辑 eth0 信息 " vim /etc/sysconfig/network-scripts/ifcfg-eth0 "
	   (2). 重启网卡" service network restart "(再不行就重启 reboot )即可

2. 修改主机名
    # 查看
    [root@localhost ~]# hostname        // 或使用" hostnamectl "命令查看
      localhost.localdomain
    # 修改
    [root@localhost ~]# hostnamectl set-hostname centos
    # 再查看
    [root@localhost ~]# hostnamectl
       Static hostname: centos

3. CentOS 系统更换软件安装源 yum
    a. 备份原镜像文件以免出错后可恢复
        [root@centos ~]# ll /etc/yum.repos.d/
            -rw-r--r--. 1 root root 1991 May 19  2016 CentOS-Base.repo
        # 查看
        [root@centos ~]# cat /etc/yum.repos.d/CentOS-Base.repo
           # CentOS-Base.repo
           [BaseOS]
           name=CentOS-$releasever - Base
           mirrorlist=http://mirrorlist.centos.org/?release=$releasever&arch=$basearch&repo=BaseOS&infra=$infra
           #baseurl=http://mirror.centos.org/$contentdir/$releasever/BaseOS/$basearch/os/
           gpgcheck=1
           enabled=1
           gpgkey=file:///etc/pki/rpm-gpg/RPM-GPG-KEY-centosofficial
        # 备份(重命名)
        [root@centos ~]# mv /etc/yum.repos.d/CentOS-Base.repo /etc/yum.repos.d/CentOS-Base.repo.backup
        [root@centos ~]# ll /etc/yum.repos.d/
           -rw-r--r--. 1 root root  712 Jun  3 09:02 CentOS-Base.repo.backup
    b. 下载新的 CentOS-Base.repo 到 /etc/yum.repos.d/
        # 选用 curl 下载并重命名(亦可使用" wget "下载)
        [root@centos ~]# curl -o /etc/yum.repos.d/CentOS-Base.repo https://mirrors.aliyun.com/repo/Centos-8.repo
        [root@centos ~]# ll /etc/yum.repos.d/
        -rw-r--r--. 1 root root 2595 Aug  5 04:34 CentOS-Base.repo
        -rw-r--r--. 1 root root  712 Jun  3 09:02 CentOS-Base.repo.backup
        # 查看
        [root@centos ~]# cat /etc/yum.repos.d/CentOS-Base.repo
           # CentOS-Base.repo
           [base]
           name=CentOS-$releasever - Base - mirrors.aliyun.com
           failovermethod=priority
           baseurl=https://mirrors.aliyun.com/centos/$releasever/BaseOS/$basearch/os/
                   http://mirrors.aliyuncs.com/centos/$releasever/BaseOS/$basearch/os/
                   # 凡文件内含" mirrors.cloud.aliyuncs.com "修正后无" cloud "关键词
                   http://mirrors.cloud.aliyuncs.com/centos/$releasever/BaseOS/$basearch/os/
           gpgcheck=1
           gpgkey=https://mirrors.aliyun.com/centos/RPM-GPG-KEY-CentOS-Official
    c. 修正" /etc/yum.repos.d/CentOS-Base.repo "文件
        [root@centos ~]# sed -i -e '/mirrors.cloud.aliyuncs.com/d' -e '/mirrors.aliyuncs.com/d' /etc/yum.repos.d/CentOS-Base.repo
        [root@centos ~]# cat /etc/yum.repos.d/CentOS-Base.repo
        # CentOS-Base.repo
        [base]
        name=CentOS-$releasever - Base - mirrors.aliyun.com
        failovermethod=priority
        baseurl=https://mirrors.aliyun.com/centos/$releasever/BaseOS/$basearch/os/
        gpgcheck=1
        gpgkey=https://mirrors.aliyun.com/centos/RPM-GPG-KEY-CentOS-Official
    d. 运行 yum makecache 生成缓存
        [root@centos ~]# yum clean all
        [root@centos ~]# yum makecache
    f. 更新系统
        [root@centos ~]# yum -y update

 */

/*
0.  Ubuntu20.04
    # 查看版本
    root@ubuntu:~# lsb_release -c
      Codename:       focal
      // ubuntu20.04 对应的即是 focal

1. 设置 root 用户登录系统(Ubuntu20.04 默认不支持 root 用户登录)
    a. 为 root 设置初始密码 / 修改 root 用户密码
       (0). 以创建的用户登录系统，打开终端输入命令" sudo passwd root "
            输入设置及再次确认 root 用户密码。
       (1). 或者" sudo -i "输入当前用户密码进入 root 用户
            最后使用" passwd root "重置 root 密码
    b. 修改" /usr/share/lightdm/lightdm.conf.d/50-ubuntu.conf "文件内容
       (0). 终端输入" sudo gedit /usr/share/lightdm/lightdm.conf.d/50-ubuntu.conf "
            chw@ubuntu:~/Desktop$ sudo gedit /usr/share/lightdm/lightdm.conf.d/50-ubuntu.conf
       (1). 打开文件并在末尾追加(切勿删除原文本任何内容)如下两行内容完成后保存并关闭
                greeter-show-manual-login=true
                all-guest=false
            # 修改后的文件内容为:
            chw@ubuntu:~/Desktop$ cat /usr/share/lightdm/lightdm.conf.d/50-ubuntu.conf
            [Seat:*]
            user-session=ubuntu
            greeter-show-manual-login=true
            all-guest=false
            chw@ubuntu:~/Desk
    c. 修改" /etc/pam.d/gdm-autologin "及"  /etc/pam.d/gdm-password "两个文件
        (0). chw@ubuntu:~/Desktop$ sudo gedit /etc/pam.d/gdm-autologin
             # 注释" auth required pam_succeed_if.so user != root quiet_success "(第三行)
             #auth	required	pam_succeed_if.so user != root quiet_success
        (1). chw@ubuntu:~/Desktop$ sudo gedit /etc/pam.d/gdm-password
             注释" auth required pam_succeed_if.so user != root quiet_success "(第三行)
             #auth	required	pam_succeed_if.so user != root quiet_success
    d. 修改" /root/.profile "文件(终端输入" sudo gedit /root/.profile ")
       chw@ubuntu:~/Desktop$ sudo gedit /root/.profile
       # 打开文件将最后一行改为:
         tty -s && mesg n || true
       # 修改后的内容为：
        chw@ubuntu:~/Desktop$ sudo cat /root/.profile
        # ~/.profile: executed by Bourne-compatible login shells.

        if [ "$BASH" ]; then
          if [ -f ~/.bashrc ]; then
            . ~/.bashrc
          fi
        fi

        #mesg n 2> /dev/null || true
        tty -s && mesg n || true
    // 注：完成保存并关闭，重启后选择" 未列出 "输入账号 root 和密码即可

2. 配置网络及修改主机名
   a. (绝不推荐)网卡切勿修改(未解决修改后能连接上网的问题)
	  root@ubuntu:~# cp /etc/netplan/01-network-manager-all.yaml /etc/netplan/01-network-manager-all.yaml.bak
	  root@ubuntu:~# ls /etc/netplan/
	  01-network-manager-all.yaml  01-network-manager-all.yaml.bak
	  	root@ubuntu:~# vi /etc/netplan/01-network-manager-all.yaml
	  # 复制粘贴以下网卡信息至" 01-network-manager-all.yaml "
	  network:
	    ethernets:
	  	ens33:     #配置的网卡的名称
	  	  addresses: [192.168.209.68/24]    #配置的静态ip地址和掩码
	  	  dhcp4: no    #关闭DHCP，如果需要打开DHCP则写yes
	  	  optional: true
	  	  gateway4: 192.168.209.1    #网关地址
	  	  nameservers:
	  		 addresses: [192.168.209.1,114.114.114.114]    #DNS服务器地址，多个DNS服务器地址需要用英文逗号分隔开
	    version: 2
	    renderer: networkd    #指定后端采用systemd-networkd或者Network Manager，可不填写则默认使用systemd-workd
	  # 使网卡生效
	  root@ubuntu:~# sudo netplan apply
   b. (推荐)可于界面修改网络配置
      # 配置: 参考图"0_网络配置.png"及"1_Ubuntu 网络配置.png"
      # 重启网卡：
      root@ubuntu:~# service network-manager restart

   c. 修改主机名及远程连接 Ubuntu
	   vim /etc/hostname		// 重启 reboot 生效

3. 配置远程连接
   a. 安装" openssh-server "
        chw@ubuntu:~/Desktop$ sudo apt-get install openssh-server
           E: Could not get lock /var/lib/dpkg/lock-frontend. It is held by process 6915 (unattended-upgr)
           N: Be aware that removing the lock file is not a solution and may break your system.
           E: Unable to acquire the dpkg frontend lock (/var/lib/dpkg/lock-frontend), is another process using it?
           # 原因分析：无论是 CentOS 还是 Ubuntu，当使用 yum update 或 yum install xxx，
                      还是 apt update 或 apt install xxx (即更新或安装)时出现某种中断则
                      出现此提示(原因可能是有另一个程序正在运行导致资源被锁[资源被锁的可能
                      源于上一次运行安装或更新时没有正常完成]进而出现此状况)
           # 解决方案 : 直接" kill 掉 yum 或 apt 相应进程 "即可
                        CentOS : " kill -9 yum进程PID "
                        Ubuntu : " kill -9 apt进程PID "  // 如" kill -9 6915"
        chw@ubuntu:~/Desktop$ sudo kill -9 6915
        chw@ubuntu:~/Desktop$ sudo apt-get install openssh-server #安装" openssh-server "
    b. Ubuntu 终端设置
        chw@ubuntu:~/Desktop$ sudo vim /etc/ssh/sshd_config
        #找到" #PermitRootLogin prohibit-password "(34行)并在其下追加" PermitRootLogin yes "
    c. 重启 ssh 服务
        chw@ubuntu:~/Desktop$ service ssh restart
    d. 确认 ssh-server 正常工作
        chw@ubuntu:~/Desktop$ netstat -tlp
           Proto Recv-Q Send-Q Local Address           Foreign Address         State       PID/Program name
           tcp        0      0 localhost:domain        0.0.0.0:*               LISTEN      -
           tcp        0      0 0.0.0.0:ssh             0.0.0.0:*               LISTEN      -
           tcp        0      0 localhost:ipp           0.0.0.0:*               LISTEN      -
           tcp6       0      0 [::]:ssh                [::]:*                  LISTEN      -
           tcp6       0      0 ip6-localhost:ipp       [::]:*                  LISTEN
    e. 查看 IP
        chw@ubuntu:~/Desktop$ ifconfig
        # 注：如果没有 ifconfig 则安装" sudo apt-get install net-tools "即可
        # 执行安装命令"apt install net-tools "使得" ifconfig "命令可用
	    # 执行安装命令"apt install vim "使得" vim "命令可用
    f. 远程配置" ssh "连接即可

4. Ubuntu20.04 更改国内镜像源(以下修改命令非 root 用户前置" sudo ")
    a. 查看
       root@ubuntu:~# ll /etc/apt/
          -rw-rw-r--   1 root root  3156 7月  30 13:10 sources.list
          drwxr-xr-x   2 root root  4096 4月   9 18:21 sources.list.d/
       root@ubuntu:~# cat /etc/apt/sources.list
         deb http://cn.archive.ubuntu.com/ubuntu/ focal main restricted
         # deb-src http://cn.archive.ubuntu.com/ubuntu/ focal main restricted
         deb http://cn.archive.ubuntu.com/ubuntu/ focal-updates main restricted
         # deb-src http://cn.archive.ubuntu.com/ubuntu/ focal-updates main restricted
         deb http://cn.archive.ubuntu.com/ubuntu/ focal universe
         # deb-src http://cn.archive.ubuntu.com/ubuntu/ focal universe
         deb http://cn.archive.ubuntu.com/ubuntu/ focal-updates universe
         # deb-src http://cn.archive.ubuntu.com/ubuntu/ focal-updates universe
         deb http://cn.archive.ubuntu.com/ubuntu/ focal multiverse
         # deb-src http://cn.archive.ubuntu.com/ubuntu/ focal multiverse
         deb http://cn.archive.ubuntu.com/ubuntu/ focal-updates multiverse
         # deb-src http://cn.archive.ubuntu.com/ubuntu/ focal-updates multiverse
         deb http://cn.archive.ubuntu.com/ubuntu/ focal-backports main restricted universe multiverse
         # deb-src http://cn.archive.ubuntu.com/ubuntu/ focal-backports main restricted universe multiverse
         # deb http://archive.canonical.com/ubuntu focal partner
         # deb-src http://archive.canonical.com/ubuntu focal partner
         deb http://security.ubuntu.com/ubuntu focal-security main restricted
         # deb-src http://security.ubuntu.com/ubuntu focal-security main restricted
         deb http://security.ubuntu.com/ubuntu focal-security universe
         # deb-src http://security.ubuntu.com/ubuntu focal-security universe
         deb http://security.ubuntu.com/ubuntu focal-security multiverse
         # deb-src http://security.ubuntu.com/ubuntu focal-security multiverse
    b. 备份
       root@ubuntu:~# cp -v /etc/apt/sources.list /etc/apt/sources.list.backup
       root@ubuntu:~# ll /etc/apt
         -rw-rw-r--   1 root root  3156 7月  30 13:10 sources.list
         -rw-r--r--   1 root root  3156 8月   4 05:36 sources.list.backup
    c. 更改镜像源
        # " gedit "仅用于 Ubuntu 界面下 Terminal 执行(弹出窗口)
        # root@ubuntu:~# gedit /etc/apt/sources.list
        root@ubuntu:~# cat /etc/apt/sources.list
          deb http://mirrors.aliyun.com/ubuntu/ focal main restricted universe multiverse
          deb http://mirrors.aliyun.com/ubuntu/ focal-security main restricted universe multiverse
          deb http://mirrors.aliyun.com/ubuntu/ focal-updates main restricted universe multiverse
          deb http://mirrors.aliyun.com/ubuntu/ focal-proposed main restricted universe multiverse
          deb http://mirrors.aliyun.com/ubuntu/ focal-backports main restricted universe multiverse
          deb-src http://mirrors.aliyun.com/ubuntu/ focal main restricted universe multiverse
          deb-src http://mirrors.aliyun.com/ubuntu/ focal-security main restricted universe multiverse
          deb-src http://mirrors.aliyun.com/ubuntu/ focal-updates main restricted universe multiverse
          deb-src http://mirrors.aliyun.com/ubuntu/ focal-proposed main restricted universe multiverse
          deb-src http://mirrors.aliyun.com/ubuntu/ focal-backports main restricted universe multiverse
    d. 更新软件源及其列表
       root@ubuntu:~# apt update        # 执行过程中可看出从配置的镜像源下载
       root@ubuntu:~# apt upgrade

 */


/*
Windows 下安装 Rust

0. 卸载 rust
    a. 命令行" win + R -> cmd "：输入" rustup self uninstall "
    b. 删除" C:\Users\Administrator\ "下" .cargo "及" .rustup "配置目录

1. 安装依赖
    a. 方式 1 ：安装官网推荐的" C++ Build Tools "
    b. 方式 2 ：下载" https://sourceforge.net/projects/mingw-w64/files/ "
                网站下的" MinGW-W64 GCC-8.1.0 "之" x86_64-posix-seh "。待
                解压后配置于 path (图" 2_Rust 安装预备.png ")
                // 注：方式 2 未能解决 cc 依赖的问题
    c. 方式 3 ： 使用 WinBuilds      // 能解决 cc 依赖的问题

2. 配置
    a. 配置环境变量(创建目录并配置)
       CARGO_HOME = E:\System\Cargo
       RUSTUP_HOME = E:\System\Rustup
    b. 设置环境变量
       RUSTUP_DIST_SERVER = https://mirrors.ustc.edu.cn/rust-static
       RUSTUP_UPDATE_ROOT = https://mirrors.ustc.edu.cn/rust-static/rustup

3. 安装 rust
    a. 官网下载" rustup-init.exe "双击运行
    b. 选择第 2 个选项(因其后使用的调试器是 GDB )即自定义安装如下：
          Current installation options:
             default host triple: x86_64-pc-windows-msvc
               default toolchain: stable (default)
                         profile: default
            modify PATH variable: yes
          1) Proceed with installation (default)
          2) Customize installation
          3) Cancel installation
          >2                            // 输入
    c. default host triple 修改为 x86_64-pc-windows-gnu (因其后使用的调试器是 GDB )
          Default host triple?
          x86_64-pc-windows-gnu            // 输入
    d. default toolchain 我们选择 stable (稳定版)
          Default toolchain? (stable/beta/nightly/none)
          stable
    e. Profile 选择 default
          Profile (which tools and data to install)? (minimal/default/complete)
          default
    f. 是否修改 PATH 变量 : 选择 Y
          Modify PATH variable? (y/n)
          y
    g. 再次确认选择 1 开始下载并安装
    h. 设置 cargo 国内镜像:在" C:\Users\Administrator\.cargo "新建 config 并配置
        [source.crates-io]
        registry = "https://github.com/rust-lang/crates.io-index"
        replace-with = 'ustc'
        [source.ustc]
        registry = "git://mirrors.ustc.edu.cn/crates.io-index"

4. 开发依赖
    a. 先更新并升级到" nightly "版本
        rustup self update         // 更新 rustup 本身
        rustup update              // 更新 工具链
        rustup default nightly     // 升级到 nightly 版本
    b. 安装 RLS (Rust Language Server)
       rustup component add rls rust-analysis rust-src
    c. 安装 cargo-expand
       cargo install cargo-expand       // 依赖 gcc
       cargo expand --help              // 验证
    d. 安装 Racer 插件 : 代码补全
       (0). # Racer 是 Rust 代码补全库，很多编辑器都需要安装它(虽部分 IDE 默认
               包含代码补全功能但其基于自实现 AST 而非基于 Racer)
               rustup toolchain add nightly
               cargo +nightly install racer
             # 另一方案：从来源安装
               下载 racer :" git clone https://github.com/racer-rust/racer.git "
               进入 racer 构建 ：" $ cargo +nightly build --release "
               $ racer --version
                 racer 2.1.36
        (1). # 代码补全需要源代码 : 可用 Rustup 下载
                rustup component add rust-src
        (2). # 设置环境变量
                RUST_SRC_PATH = E:\System\Rustup\toolchains\nightly-x86_64-pc-windows-gnu\lib\rustlib\src\rust\src
    e. cargo 插件
        (0). clippy 分析源代码,检查代码中的 Code Smell 可用 rustup 工具安装
             rustup component add clippy
        (1). rustfmt 统一代码风格(开发中推荐使用)，可通过 rustup 工具来安装
             rustup component add rustfmt
        (2). cargo fix :自动修复编译器中有警告的代码(1.29版本后 cargo 自带)

 */


/*  Windows 安装 rust-init.ext 提示：
           解决方案" 安装 WinBuilds "

Rust Visual C++ prerequisites

Rust requires the Microsoft C++ build tools for Visual Studio 2013 or
later, but they don't seem to be installed.

The easiest way to acquire the build tools is by installing Microsoft
Visual C++ Build Tools 2019 which provides just the Visual C++ build
tools:

  https://visualstudio.microsoft.com/visual-cpp-build-tools/

Please ensure the Windows 10 SDK and the English language pack components
are included when installing the Visual C++ Build Tools.

Alternately, you can install Visual Studio 2019, Visual Studio 2017,
Visual Studio 2015, or Visual Studio 2013 and during install select
the "C++ tools":

  https://visualstudio.microsoft.com/downloads/

Install the C++ build tools before proceeding.

If you will be targeting the GNU ABI or otherwise know what you are
doing then it is fine to continue installation without the build
tools, but otherwise, install the C++ build tools before proceeding.

Continue? (Y/n)

 */

/*
0. Git 环境(非 root 用户其命令前置" sudo ")
    a. 安装
       (0). CentOS
            [root@centos ~]# yum -y update          // 更新系统
            [root@centos ~]# dnf install git-all   // 亦可使用" yum "安装
       (1). Ubuntu
            root@ubuntu:~# apt-get update          // 安装前须先更新系统
            root@ubuntu:~# apt-get install git-all
    b. 校验
       $ git --version

1. Rust 环境
    a. 安装(Ubuntu20.04 需先执行" apt install curl "安装)
       $ curl https://sh.rustup.rs -sSf | sh
       // 卸载可用" rustup self uninstall "命令
    b. 配置当前 shell
       [root@centos ~]# source $HOME/.cargo/env
    c. 验证
       [root@centos ~]# rustc --version
           rustc 1.45.2 (d3fb005a3 2020-07-31)
       [root@centos ~]# cargo --version
           cargo 1.45.1 (f242df6ed 2020-07-22)

2. 配置 Rust 镜像源及" nightly "版本及创建" project "目录
    a. 将以下命令粘贴至终端执行(即在"$HOME/.cargo/"创建"config"并配置)
        # 中科大源
        tee $HOME/.cargo/config <<-'EOF'
        [source.crates-io]
        registry = "https://github.com/rust-lang/crates.io-index"
        replace-with = 'ustc'
        [source.ustc]
        registry = "git://mirrors.ustc.edu.cn/crates.io-index"
        EOF
        # 注：若所处的环境中不允许使用 git 协议则可将"git"改为"https"即
              registry = "https://mirrors.ustc.edu.cn/crates.io-index"
    b. 查看 config
        [root@centos ~]# cat .cargo/config
           [source.crates-io]
           registry = "https://github.com/rust-lang/crates.io-index"
           replace-with = 'ustc'
           [source.ustc]
           registry = "git://mirrors.ustc.edu.cn/crates.io-index"
    c. 更新
       [root@centos ~]# rustup self update      // 更新 rustup 自身
       [root@centos ~]# rustup update           // 更新工具链 toolchain
    d. 配置" nightly "版本
       root@ubuntu:~# rustup default nightly
    e. 查看
       root@ubuntu:~# cargo --version
          cargo 1.47.0-nightly (aa6872140 2020-07-23)
    f. 统一在" $Home "目录下创建" project "目录(用于存储 rust project)
        root@ubuntu:~# mkdir project
        [root@centos ~]# mkdir project

3. 安装 mysql (不可置于 root 用户目录[权限拒绝])
    a. 不可在 Windows 下解压 后上传至" /system "目录下
        # SecureCRT 使用 Alt + P 快捷键打开上传窗口
        sftp> cd /system     // 切换至指定目录
        sftp>     # 此时将  Windows 上目标文件拖拽至窗口即可
        # 解压
        [root@centos ~]# tar xvJf /system/mysql-8.0.13-linux-glibc2.12-x86_64.tar.xz  -C /system
        # 重命名
        [root@centos ~]# mv /system/mysql-8.0.13-linux-glibc2.12-x86_64 /system/mysql
        [root@centos ~]# ls /system
          mysql
    b. 安装" numactl "及在" /system/mysql "创建" data "目录
        # 安装 numactl
           # CentOS 安装 numactl
             [root@centos ~]# yum -y install numactl
           # Ubuntu 安装 numactl
             root@ubuntu:~# apt install numactl
        # 创建" /system/mysql/data "
        [root@centos ~]# mkdir /system/mysql/data
    c. (综合查询)查看是否存在 mysql 组和 mysql 用户
        [root@centos ~]# cat /etc/group  /etc/passwd  | grep mysql
        // 无显示则说明不存在
    d. 创建 mysql 组及 mysql 用户
        [root@centos ~]# groupadd mysql
        [root@centos ~]# useradd -r -g mysql mysql
    e. 创建修改"mysqld.log"文件及"mysqld.pid"文件权限
        # 创建
        [root@centos ~]# vim /system/mysql/mysqld.log
        [root@centos ~]# vim /system/mysql/mysqld.pid
        # 查看
        [root@centos ~]# ll /system/mysql/ | grep mysqld.*
            -rw-r--r--.  1 root root       0 Aug  5 17:34 mysqld.log
            -rw-r--r--.  1 root root       0 Aug  5 17:34 mysqld.pid
        # 修改
        [root@centos ~]# chmod 666 /system/mysql/mysqld.log /system/mysql/mysqld.pid
        [root@centos ~]# ll /system/mysql/ | grep mysqld.*
            -rw-rw-rw-.  1 root root      0 Aug  4 13:30 mysqld.log
            -rw-rw-rw-.  1 root root      0 Aug  4 13:30 mysqld.pid
    f. 更改 mysql 目录下所有文件及目录所属" 组 / 用户 "( -R : recursive )
        [root@centos ~]# chown -R mysql /system/mysql/
        [root@centos ~]# chgrp -R mysql /system/mysql/
        [root@centos ~]# ll /system/mysql/
            drwxr-xr-x.  2 mysql mysql   4096 Aug  5 13:43 bin
            drwxr-xr-x.  2 mysql mysql      6 Aug  5 13:43 data
            drwxr-xr-x.  6 mysql mysql   4096 Aug  5 13:43 lib
            -rw-rw-rw-.  1 mysql mysql      0 Aug  4 13:30 mysqld.log
            -rw-rw-rw-.  1 mysql mysql      0 Aug  4 13:30 mysqld.pid
            drwxr-xr-x.  2 mysql mysql     90 Aug  5 13:43 support-files
    g. 在" /etc/ "下创建" my.cnf "文件并键入如下内容
        [root@centos ~]# vim /etc/my.cnf
            [client]
            port=3306
            [mysqld]
            character_set_server=utf8
            basedir=/system/mysql
            datadir=/system/mysql/data
            log-error=/system/mysql/mysqld.log
            pid-file=/system/mysql/mysqld.pid
         # 查看
         [root@centos ~]# cat /etc/my.cnf
            [client]
            port=3306
            [mysqld]
            character_set_server=utf8
            basedir=/system/mysql
            datadir=/system/mysql/data
            log-error=/system/mysql/mysqld.log
            pid-file=/system/mysql/mysqld.pid
    h. 配置" $MYSQL_HOME/bin "并执行其内命令(亦可直接进入" mysql/bin "目录执行)
        # 配置 MYSQL_HOME
        [root@centos ~]# vim /etc/profile   // 在文件末尾追加如下内容
            export MYSQL_HOME=/system/mysql
            export PATH=$PATH:$MYSQL_HOME/bin
        # 查看
        [root@centos ~]# echo $PATH
            /root/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/root/bin
        # 重新加载配置文件
        [root@centos ~]# source /etc/profile
        # 再次查看
        [root@centos ~]# echo $PATH
        /root/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/root/bin:/system/mysql/bin
    i. 初始化(完毕后可在" /system/mysql/mysqld.log"日志中查看密码)
        (0). # Ubuntu 初始化时出现错误
               mysqld: error while loading shared libraries: libaio.so.1: cannot open shared object file: No such file or directory
             # 解决方案：先安装" libiao1 "
             root@ubuntu:~# apt-get install libaio1
             # 初始化
             root@ubuntu:~# mysqld  --initialize --user=mysql  --basedir=/system/mysql/  --datadir=/system/mysql/data/
        (1). CentOS 正常初始化
             [root@centos ~]# mysqld  --initialize --user=mysql  --basedir=/system/mysql/  --datadir=/system/mysql/data/
    j. 设置 service 命令启动 mysql
        # 未配置时不可用" service mysql xxx "命令
        [root@centos ~]# service mysql start
            Redirecting to /bin/systemctl start mysql.service
            Failed to start mysql.service: Unit mysql.service not found.
        # 查询 /etc/init.d/ 是否存在 mysql (无则拷贝)
        [root@centos ~]# ll /etc/init.d/ | grep mysql  // 无显示则说明不存在
        # 将" mysql/support-files/mysql.server "拷贝重命名到" /etc/init.d/mysql "并设置运行权限(方
          可用 service 命令启动/停止服务[否则只能用" $MYSQL_HOME/bin/mysqld_safe & "命令来启动服务])
        [root@centos ~]# cp /system/mysql/support-files/mysql.server /etc/init.d/mysql
        [root@centos ~]# ll /etc/init.d/ | grep mysql
            -rwxr-xr-x. 1 root root 10576 Aug  5 17:42 mysql
    K. 修改 mysql 文件中 basedir 及 datadir 相关路径
        [root@centos ~]# vim /etc/init.d/mysql
        basedir=/system/mysql                   // 46 行
        datadir=/system/mysql/data              // 47 行
    k. 启动服务(出错)
       (0). CentOS 错误
            [root@centos ~]# service mysql start
                Starting MySQL...... ERROR! The server quit without updating PID file (/system/mysql/mysqld.pid).
            # 错误解决方案 ：kill -9 {所有mysql pid }
            [root@centos ~]# ps aux | grep mysql
                mysql  2683  0.6  4.9 2162224 396816 ?  Sl   16:51   0:24 /system/mysql/bin...
                mysql  2765  0.6  4.9 2162224 396816 ?  Sl   16:51   0:24 /system/mysql/bin...
            # kill
            [root@centos ~]# kill -9 {2683,2765}
            # 重新启动服务
            [root@centos ~]# service mysql start
                Starting MySQL SUCCESS!             // 启动成功
        (1). Ubuntu 错误
             root@ubuntu:~# service mysql start
                 Failed to start mysql.service: Unit mysql.service not found
             # 解决方案：使得" mysqld.service "可用
             root@ubuntu:~# systemctl enable mysqld.service
                 mysqld.service is not a native service, redirecting to systemd-sysv-install.
                 Executing: /lib/systemd/systemd-sysv-install enable mysqld
             # 启动服务
             root@ubuntu:~# service mysql start
     l. 查看启动状态
        [root@centos ~]# netstat -nltp
            Proto Recv-Q Send-Q Local Address   Foreign Address  State       PID/Program name
            tcp6       0      0 :::3306         :::*             LISTEN      5722/mysqld
            tcp6       0      0 :::33060        :::*             LISTEN      5722/mysqld
     m. 将 mysql 注册为开机启动
        (0). CentOS 使用 chkconfig 注册为开机启动
             # 方式 1 ：未生效(勿操作)
             [root@centos ~]# type mysql           // Display information about command type
                 mysql is /system/mysql/bin/mysql
             [root@centos ~]# chkconfig --add mysql
             [root@centos ~]# chkconfig --list mysql
                 mysql   0:off   1:off   2:on    3:on    4:on    5:on    6:off  // 注册成功
             # 方式 2 ：亦未生效(勿操作)
             # 拷贝并查看当前系统的服务启动和状态
             [root@centos ~]# cp /system/mysql/support-files/mysql.server /etc/init.d/mysqld
             [root@centos ~]# systemctl list-unit-files
                mysql.service                              generated
                mysqld.service                             generated
             # 使其可用
               systemctl enable mysqld
         (1). Ubuntu 使用 update-rc.d 注册为开机启动
              root@ubuntu:~# update-rc.d mysql defaults
              # 查看服务列表
              root@ubuntu:~# sudo service --status-all
                 [ - ]  mysql
     n. (查看密码)登录
        (0). CentOS 登录(错误)
            # 查看密码
            [root@centos ~]# cat /system/mysql/mysqld.log
                ...A temporary password is generated for root@localhost: tohGvRc,T8.L
            # (未携带密码)登录(错误)
            [root@centos ~]# mysql -uroot -p
                mysql: error while loading shared libraries: libtinfo.so.5: cannot open shared object file: No such file or directory
            # (携带密码)登录(错误)
            [root@centos ~]# mysql -uroot -p%w<FGT2fL+U2
                -bash: FGT2fL+U2: No such file or directory     // 解决方案：若为此种情况可手动输入密码
            # 错误解决方案: 建立软连接
            [root@centos ~]# sudo ln -s  /usr/lib64/libtinfo.so.6.1 /usr/lib64/libtinfo.so.5
            # 重新登录
            [root@centos ~]# mysql -uroot -ptohGvRc,T8.L
                mysql: [Warning] Using a password on the command line interface can be insecure.
                Welcome to the MySQL monitor.  Commands end with ; or \g.
                Your MySQL connection id is 8
                Server version: 8.0.13
             mysql>     // 出现此标识则登录成功
         (1). Ubuntu 登录(错误)
            # (未携带密码)登录(错误)
            root@ubuntu:~# mysql -uroot -p
                mysql: error while loading shared libraries: libtinfo.so.5: cannot open shared object file: No such file or directory
            # (携带密码)登录(错误)
            [root@centos ~]# mysql -uroot -p.b;i)ECMj4h:
                -bash: syntax error near unexpected token `)'   // 解决方案：若为此种情况可手动输入密码
            # 错误解决方案: 查看系统是否存在" libtinfo.so.x "文件(存在则拷贝至" /lib/x86_64-linux-gnu/ "
              目录下并更名为" libtinfo.so.5 "；否则从 Windows" D:\Database\Mysql\libtinfo.so.5 "上传)
            # 查看是否存在
              root@ubuntu:~# ll /lib/x86_64-linux-gnu/libtinfo.so.*
                lrwxrwxrwx 1 root root     15 7月  30 12:31 /lib/x86_64-linux-gnu/libtinfo.so.6 -> libtinfo.so.6.2
                -rw-r--r-- 1 root root 192032 2月  26 15:14 /lib/x86_64-linux-gnu/libtinfo.so.6.2
             # 存在则拷贝至" /lib/x86_64-linux-gnu/ "录下并更名为" libtinfo.so.5 "
             root@ubuntu:~# cp /lib/x86_64-linux-gnu/libtinfo.so.6 /lib/x86_64-linux-gnu/libtinfo.so.5
             # 重新登录
             root@ubuntu:~# mysql -uroot -p
             Enter password:
                mysql>     // 出现此标识则登录成功
     o. # 修改密码
         mysql> ALTER USER "root"@"localhost" IDENTIFIED  BY "root";
            Query OK, 0 rows affected (0.04 sec)
         # 将 sql_mode 值去除首元素" ONLY_FULL_GROUP_BY "后追加到" /etc/my.cnf "文件末尾
         mysql> select @@global.sql_mode;
            +-----------------------------------------------------------------------------------------------------------------------+
            | @@global.sql_mode                                                                                                     |
            +-----------------------------------------------------------------------------------------------------------------------+
            | ONLY_FULL_GROUP_BY,STRICT_TRANS_TABLES,NO_ZERO_IN_DATE,NO_ZERO_DATE,ERROR_FOR_DIVISION_BY_ZERO,NO_ENGINE_SUBSTITUTION |
            +-----------------------------------------------------------------------------------------------------------------------+
            1 row in set (0.00 sec)
         mysql> exit
             Bye
     p. 将" sql_mode=STRICT_TRANS_TABLES,NO_ZERO_IN_DATE,NO_ZERO_DATE,ERROR_FOR_DIVISION_BY_ZERO,NO_ENGINE_SUBSTITUTION "
        追加至" /etc/my.cnf "文件末尾
        [root@centos ~]# vim /etc/my.cnf
        [root@centos ~]# cat /etc/my.cnf
            [client]
            port=3306
            [mysqld]
            character_set_server=utf8
            basedir=/system/mysql
            datadir=/system/mysql/data
            log-error=/system/mysql/mysqld.log
            pid-file=/system/mysql/mysqld.pid
            sql_mode=STRICT_TRANS_TABLES,NO_ZERO_IN_DATE,NO_ZERO_DATE,ERROR_FOR_DIVISION_BY_ZERO,NO_ENGINE_SUBSTITUTION
        [root@centos ~]# halt

4. 安装 cargo-expand (用于展开宏)
    a. 须先安装 GCC 依赖
       # 安装一系列软件包包括 gcc、g++ 及 make
       (0).CentOS
           [root@centos ~]# sudo dnf group install "Development Tools"
           # 安装 GNU/Linux 开发的手册 (按需安装)
           [root@centos ~]# sudo dnf install man-pages
       (1). Ubuntu
           root@ubuntu:~# sudo apt install build-essential
           # 具体可参考 ：
             # To install basic development tools, run:
                    sudo apt install build-essential   // 安装基本开发工具
             # Any other tools can be installed with:
                    sudo apt install tool-name          // 安装指定工具
       # 验证
       [root@centos ~]# gcc --version
          gcc (GCC) 8.3.1 20191121 (Red Hat 8.3.1-5)
    b. 安装 cargo-expand
       [root@centos ~]# cargo install cargo-expand
    c. 验证
       [root@centos macro_expand]# cargo expand

5. 配置安装 Racer (自动补全和语法分析工具)     // 使用官方源安装
    # 先切换至官方源
    a. 切换到 nightly 版本
       root@ubuntu:~# rustup toolchain add nightly
    b. 安装
       [root@centos ~]# cargo +nightly install racer
       root@ubuntu:~# racer --version
            racer 2.1.37
    c. 获取源码
       root@ubuntu:~#  rustup component add rust-src
            info: downloading component 'rust-src'
            info: installing component 'rust-src'
            info: Defaulting to 500.0 MiB unpack ram
    d. 更新源码
       root@ubuntu:~# rustup update
    e. 环境设置
       # 在 .bashrc 中添加以下内容
            export RUST_SRC_PATH="$(rustc --print sysroot)/lib/rustlib/src/rust/src"
       root@ubuntu:~# vim .bashrc
       root@ubuntu:~# source .bashrc
       root@ubuntu:~# echo $RUST_SRC_PATH
            /root/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src

注：以上为 Origin 版本安装内容(以下为待加入 Origin 的内容)
6. 配置镜像源(国内推荐"清华源")及安装 Chrome 浏览器
    a. 配置镜像源(国内推荐"清华源")
       Ubuntu Desktop -> Show Applications -> Settings -> About(左侧边栏最下方)
                      -> Software Updates -> Ubuntu Software 选项卡下
                      -> 点击 Download from 栏右侧下拉列表 -> 选择" Other... "
                      -> 对话框中选择 China 并点击其右侧的" Select Best Server "
                      -> 选择推荐的镜像源保存退出->" apt update / apt upgrade "
       参见图"3_Ubuntu有效更换源.png"
   b. 安装 Chrome 浏览器
       # 卸载 Chrome
       root@ubuntu:~# sudo apt-get autoremove google-chrome-stable
       # 删除下载源
       root@ubuntu:~# sudo rm /etc/apt/sources.list.d/google-chrome.list
       安装 Chrome 浏览器
       root@ubuntu:~# wget https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb
       root@ubuntu:~# apt install ./google-chrome-stable_current_amd64.deb
       # 安装后打不开(待解决[?])


 */