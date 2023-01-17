
# 需求

grep 命令用于查找文件里符合条件的字符串。如果发现某个文件的内容符合所指定的字符串，grep 命令会把含有字符串的那一行显示出；若不指定任何文件名称，或是所给予的文件名为  -，grep 命令会从标准输入设备读取数据。

首先是最简单的，给定一个字符串以及一个文件，打印出文件中所有包含该字符串的行：

```rust
$ rgrep Hello a.txt
55: Hello world. This is an exmaple text
```

然后放宽限制，允许用户提供一个正则表达式，来查找文件中所有包含该字符串的行：

```rust
$ rgrep Hel[^\\s]+ a.txt
55: Hello world. This is an exmaple text
89: Help me! I need assistant!
```

如果这个也可以实现，那进一步放宽限制，允许用户提供一个正则表达式，来查找满足文件通配符的所有文件（你可以使用 globset 或者 glob 来处理通配符），比如：

```rust

$ rgrep Hel[^\\s]+ a*.txt
a.txt 
    55:1 Hello world. This is an exmaple text
    89:1 Help me! I need assistant!
    5:6  Use `Help` to get help.
abc.txt:
    100:1 Hello Tyr!
```

推荐:

- 使用 [clap](https://github.com/clap-rs/clap)
- 正则使用 [regex](https://github.com/rust-lang/regex)
- 文件可以 [std::fs](https://doc.rust-lang.org/std/fs/index.html) 或 [tokio::fs](https://docs.rs/tokio/1.12.0/tokio/fs/index.html), 你可以顺序对所有满足通配符的文件进行处理，也可以用 [rayon](https://docs.rs/rayon/1.5.1/rayon/) 或者 [tokio](https://docs.rs/tokio/1.12.0/tokio) 来并行处理.
- 对于输出的结果，最好能把匹配的文字用不同颜色展示。
