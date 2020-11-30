
/*
0. Windos 安装 llvm
    a. 安装 Visual Studio
    b. 安装 Python
    c. 将 Cmake 追加至环境变量

1. 安装步骤
    a. 下载安装
        git clone https://github.com/llvm/llvm-project.git
        cd llvm-project
        mkdir build
        cd build
        cmake -DLLVM_ENABLE_PROJECTS=clang -G "Visual Studio 15 2017" -A x64 -Thost=x64 ../llvm
    b. Visual Studio 构建
        File -> Open File -> E:\System\llvm-project\build\LLVM.sln
        -> CMakePredefinedTargets -> ALL_BUILD (右击选择 build)
    // 文档参阅" 4_Windows下LLVM环境配置.png "

2. Windows 下 substrate 编译
    a. 安装 nightly 编译链
        rustup update nightly
    b. 对 nightly 编译链添加 wasm 编译target
       rustup target add wasm32-unknown-unknown --toolchain nightly
       export WASM_BUILD_TYPE=release // 建议配置
    c. 进入 substrate 编译
       cargo build --release          // 编译失败(待解决[?])

*/