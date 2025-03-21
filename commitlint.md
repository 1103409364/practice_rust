# Git commit 规范

AngularJS 的 Commit 规范

```xml
<type>(<scope>): <subject>
<空行>
<body>
<空行>
<footer>
```

上面是一次 Commit 后 Message 格式规范，分成标题，内容详情，结尾三个部分，各有各的用处，没有多余项。

## 头部

即首行，是可以直接在页面中预览的部分，入上面图中所示，一共有三个部分`<type>`，`<scope>`，`<subject>`，含义分别如下

### Type

- **feat**：新功能（feature）
- **fix**：修补 bug
- **docs**：文档（documentation）
- **style**： 格式（不影响代码运行的变动）
- **refactor**：重构（即不是新增功能，也不是修改 bug 的代码变动）
- **test**：增加测试
- **revert**：回滚
- **config**：构建过程或辅助工具的变动
- **chore**：其他改动

### Scope

用来说明本次 Commit 影响的范围，即简要说明修改会涉及的部分。这个本来是选填项，但从 AngularJS 实际项目中可以看出基本上也成了必填项了。

### Subject

用来简要描述本次改动，概述就好了，因为后面还会在 Body 里给出具体信息。并且最好遵循下面三条:

- 以动词开头，使用第一人称现在时，比如 change，而不是 changed 或 changes
- 首字母不要大写
- 结尾不用句号 (.)

### Body

`<body>` 里的内容是对上面 subject 里内容的展开，在此做更加详尽的描述，内容里应该包含修改动机和修改前后的对比。`</subject>`

### Footer

Footer 里的主要放置**不兼容变更**和**Issue 关闭**的信息，参考下面两个例子

![img](https://upload-images.jianshu.io/upload_images/1966717-d12361273625cc5d?imageMogr2/auto-orient/strip|imageView2/2/w/640/format/webp)

![img](https://upload-images.jianshu.io/upload_images/1966717-14381765ab9efcf3?imageMogr2/auto-orient/strip|imageView2/2/w/640/format/webp)

### Revert

此外如果需要撤销之前的 Commit，那么本次 Commit Message 中必须以`revert：`开头，后面紧跟前面描述的 Header 部分，格式不变。并且，Body 部分的格式也是固定的，必须要记录撤销前 Commit 的 SHA 值。

ATTENTION:

1. 注意 type 和内容之间的空格；
2. 请不要使用--no-verify 参数忽略掉提交信息
