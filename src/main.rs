/**
hello rust
*/
/**①标准的rust主程序：
    程序命名规范：xxx_xxx.rs
    一些命令：
        查看rust版本：rustc --version 或者-V
        程序编译：rustc xxx.rs
            编译：同目录下生成 xxx.exe 可执行文件
            运行：main 或者 main.exe即可
            备注：rustc只适合简单的编译 大项目使用Cargo
    ②Cargo
    含义：包管理工具
    作用：代码构建、下载库
    一些命令：
        查看Cargo版本：cargo --version(和rust版本保持一致)
        新建工程：cargo new 工程名
        构建工程：cargo build
        构建且运行：cargo run
            备注：之前编译过且代码没有修改的话，运行该命令会等同于执行可执行文件。
        代码检查：cargo check
            备注：速度比cargo build快的多，开发者可以在编写代码时周期性的运行此命令，检查代码。
        发布版本：cargo build --release
            备注：默认会自动话优化你的代码(编译的时间会比较长)。
            编译后的代码在target/release下而不是debug下
    空项目转cargo:
        新建目录、新建toml文件、新建src以及主函数
    建议：
        推荐使用cargo指令来构建运行检查项目而不是rustup
    ③toml
    配置文件：
        package:项目名字、版本，作者信息等
        dependencies:依赖
    ④crate
    含义：rust的包或者库
    ⑤cargo.lock
    来源：运行cargo build后会生成该文件
    作用：负责追踪项目依赖的精确版本；不需要也不必要手动修改文件

*/
fn main() {
    println!("Hello, world!");
}
