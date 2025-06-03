```sh
..####....####....####...######..######...........####...#####...######.
.##..##..##......##..##....##......##............##..##..##..##....##...
.######...####...##........##......##............######..#####.....##...
.##..##......##..##..##....##......##............##..##..##..##....##...
.##..##...####....####...######..######..######..##..##..##..##....##...
```

## ASCII ART GENERATOR

`ASCII_Art_Generator` 是一个用 Rust 语言编写的小工具，可将图像（包括 GIF 动图）转换为 ASCII Art 字符画。它支持多种输出格式，如文本文件（`.txt`）、JSON 文件（`.json`）、HTML 文件（`.html`）和图像文件（目前支持`.jpg` 、`.jpeg`和`.png`），同时还提供了彩色输出、自定义字符集和伽马校正等功能。  



### 特性

- **多种输出格式**：支持直接在终端输出，也可保存为文本文件、JSON文件、HTML文件和图像。
- **彩色输出**：通过 ANSI 转义序列实现彩色 ASCII Art。
- **自定义字符集**：允许用户指定自定义字符集，使用自定义字符集进行生成。
- **GIF 动图支持**：能够将 GIF 动图转换为 ASCII Art并播放。（注：现在已支持导出为`.gif`文件，导出操作耗时较长，请耐心等待）
- **伽马校正**：可以调整伽马校正因子，优化图像亮度。



### 安装

确保你已经安装了 Rust 开发环境。

克隆仓库到本地：

```sh
git clone https://github.com/Limpid-8818/ascii_art_generator.git
cd ascii_art_generator
```

使用 `Cargo` 构建项目：

```sh
cargo build --release
```



### 使用方法

#### 基本用法

将图像转换为 ASCII 艺术并保存为文本文件：

```sh
./target/release/ASCII_Art_Generator -i input.jpg -o output.txt
```

保存后在终端中查看：

```sh
cat output.txt
```

#### 命令行参数

- `-i, --input <FILE>`：输入图像文件路径，必填项。
- `-o, --output <FILE>`：输出文件路径，支持 `.txt`（默认）、`.json` 、`.html` 等扩展名。
- `-w, --width <WIDTH>`：输出 ASCII Art的宽度，默认为 80。
- `-t, --height <HEIGHT>`：输出 ASCII Art的高度，默认为根据图像比例自动计算。
- `-g, --gamma <GAMMA>`：伽马校正因子，默认为 1.0。
- `-c, --color`：启用彩色输出。
- `-v, --invert`：反转字符集。
- `--charset <CHARSET>`：使用的字符集，可选值为 `default`、`simple`、`block` 或 `pixel`，默认为 `default`。
- `--custom-charset <CHARSET>`：自定义字符集，使用此选项时 `--charset` 将被忽略。

#### 示例

- **彩色输出**：

```sh
./target/release/ASCII_Art_Generator -i input.jpg -o output.html -c
```

- **自定义字符集**：

```sh
./target/release/ASCII_Art_Generator -i input.jpg -o output.txt --custom-charset abcdefg
```

- **播放 GIF 动图**：

```sh
./target/release/ASCII_Art_Generator -i input.gif
```



### 贡献

欢迎对本项目进行贡献！如果你发现了 bug 或者有新的功能建议，请提交 issue 或者 pull request。

