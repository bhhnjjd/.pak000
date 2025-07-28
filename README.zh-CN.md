# pak000 仓库

本仓库包含一个用 Rust 编写的 WebAssembly 格式 `.pak` 文件解析器，以及一个简单的 JavaScript 前端。

## 常见问题与修复

* **过时的 `typed-arena` 依赖** – 已在 `Cargo.toml` 中删除。
* **未使用的导入** – 清理了 `lib.rs`和`security.rs`中的无效导入。
* **损坏的测试** – 删除了引用不存在并释放错误的测试。
* **npm 包版本错误** – `webpack-bundle-analyzer` 使用 `^4.10.2`。
* **缺少入口文件** – 添加了 `src/index.js`以便 webpack 构建。
* **JSX 标签未闭合** – 修复 `HexEditor.jsx`中的按钮标签闭合问题。
* **缺少换行** – 为一些文件添加缩进末换行。

## 构建方法

```bash
# 构建 Rust 解析器
cd pak_parser
cargo build --release

# 安装前端依赖并编译
cd ../pak_editor
npm install
npm run build
```

运行测试：

```bash
cd pak_parser
cargo test
```

## 安全注意

目前解析器只做了基础校验，如需在生产环境使用，请考虑添加更完善的边界检查以及与 WebAssembly 交互的安全之处理。
