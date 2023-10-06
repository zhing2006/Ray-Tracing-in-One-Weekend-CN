<p align="right">
   <a href="./README.md">English</a> | <strong>中文</strong>
</p>

一周末光线追踪系列图书
====================================================================================================

| ![一周末光线追踪][cover1] | ![下一周光线追踪][cover2] | ![余生中的光线追踪][cover3] |
|:----------------------------:|:---------------------------:|:-----------------------------------:|
|   [一周末完成][book1]    |   [下一周][book2]    |   [余生中的光线追踪][book3]    |


获取图书
------------------
《一周末光线追踪》系列图书现在可以直接从网上免费获取。

我们目前在同一个项目中托管了旧版本v3.2.3和v4.0.0-alpha.1版本。旧版本v3是为了正在阅读该系列的读者提供连续性。对于新读者或刚开始阅读其中一本书的读者，我们强烈建议您跳到v4版本。

### 版本 3.2.3
  - [《一周末光线追踪》][web1-v3]
  - [《光线追踪：下一周》][web2-v3]
  - [《光线追踪：你的余生》][web3-v3]

### 版本 4.0.0-alpha.1

  - [《一周末光线追踪》][web1]
  - [《光线追踪：下一周》][web2]
  - [《光线追踪：你的余生》][web3]

这些书籍已经适配了屏幕和打印。如果需要打印副本或创建 PDF 版本，查看[PRINTING_CN.md][]获取更多信息。


项目状态
---------------
哇！我们在洛杉矶的SIGGRAPH大会上取得了巨大的成功，包括为对该书系感兴趣的人举办了一次birds-of-a-feather聚会。大约有50人参加了。我不会在这里重述所有的内容，但有三个主要的事项是每个人都应该知道的。

**首先，v4.0.0-alpha.1已经发布。** 第一本书基本完成，我们现在正在专注于第二和第三本书。您可以在常规位置找到它：https://raytracing.github.io。我们暂时保留了v3.2.3的最终版本在仓库中，所以您可以直接获取最新版本并参考两个版本。

**其次，Trevor和我正在努力完成和发布最终的v4.0.0版本。** 我们计划在2023年底完成这项工作。如果您想查看最新的更新并关注我们的进展，我们在`dev`分支上。您还可以浏览我们的发布积压任务来了解我们的计划。我们的相关里程碑包括：

  - [v4.0.0](https://github.com/RayTracing/raytracing.github.io/milestone/16)
  - [v4.0.0-release](https://github.com/RayTracing/raytracing.github.io/milestone/19)

**第三，我们开始思考接下来的方向。** 心中的首要话题包括阴影光线、三角网格几何和并行计算，但还有许多可能的扩展，无论是大还是小。如果您有兴趣做出贡献，请给我们发送电子邮件！您可以在每本书的开头找到我们的联系信息。

如果您想贡献一个PR，请**先阅读我们的[贡献指南][CONTRIBUTING]**。


GitHub讨论
------------------
您对光线追踪代码有一般性问题，对自己的实现有问题，或者想分享一些光线追踪的想法吗？请查看我们的[GitHub讨论][discussions]论坛！


目录结构
-------------------
该存储库的组织结构旨在简单明了：

  - `books/` --
    此文件夹包含三本光线追踪书籍（以HTML格式），以及一些支持材料。

  - `images/` --
    包含所有书籍的图像和插图。也可以用来比较您的结果。

  - `style/` --
    包含书籍和网站的CSS样式。

  - `src/` --
    包含源代码。

  - `src/<book>/` --
    包含每本书的最终源代码。

  - `v3/` --
    v3.2.3版本（2020年12月）的所有内容（具有相同的一般结构）。

  - `v3/common` --
    包含两本或更多v3书籍共用的v3头文件。这也是外部头文件存储的位置。


源代码
-----------
### 目的
该存储库并不意味着作为教程。提供源代码是为了在阅读书籍时与您的工作进行比较。我们强烈建议阅读并跟随书籍，以理解源代码。理想情况下，您将在阅读过程中开发自己的实现，以深入理解光线追踪器的工作原理。

### 下载源代码
这个项目的[GitHub home][]包含与《一个周末实现光线追踪》书系相关的所有源代码和文档。要克隆或下载源代码，请参见项目主页右上方的绿色"Clone or download"按钮。

### 编程语言
本书使用C++编写，并使用了一些C++11的现代特性。选择这种语言和特性是为了广泛为最多的程序员所理解。它并不意味着代表理想（或优化）的C++代码。

### 其他语言的实现
《一个周末实现光线追踪》系列在其他编程语言中有着悠久的实现历史（参见[_其他语言的实现_][implementations]），并且在不同的操作系统上也有着实现。欢迎将您自己的实现添加到列表中！

### 分支
一般来说，所有最新更改的正在进行中的开发可以在`dev`分支中找到，该分支可能包含补丁、次要和主要更改，具体取决于正在进行的发布。我们尽量保持CHANGELOG.md的最新状态，这样您可以轻松浏览每个开发分支中的新内容。我们可能会不时使用其他开发分支，因此请通过查看[CONTRIBUTING][]页面保持最新。

`release`分支包含最新发布（和实时）的资源。这是GitHub页面提供https://raytracing.github.io/的分支。


构建和运行
---------------------
提供源代码的副本供您检查和比较。如果您希望构建提供的源代码，该项目使用CMake。要构建，请转到项目目录的根目录，并运行以下命令以创建每个可执行文件的调试版本：

    $ cmake -B build
    $ cmake --build build

您可以使用`--target <program>`选项指定目标，其中program可以是`inOneWeekend`、`theNextWeek`、`theRestOfYourLife`或任何演示程序。默认情况下（没有`--target`选项），CMake将构建所有目标。

在Windows上，您可以构建`debug`（默认）或`release`（优化版本）。要指定这一点，使用`--config <debug|release>`选项。


### 在Windows上使用CMake GUI
您可以选择在Windows上构建时使用CMake GUI。

1. 在Windows上打开CMake GUI。
2. 对于"Where is the source code:"，设置为复制目录的位置。例如，`C:\Users\Peter\raytracing.github.io`。
3. 在复制目录的位置中添加一个名为"build"的文件夹。例如，`C:\Users\Peter\raytracing.github.io\build`。
4. 对于"Where to build the binaries"，将其设置为新创建的"build"目录。
5. 点击"Configure"。
6. 对于"Specify the generator for this project"，将其设置为您的Visual Studio版本。
7. 点击"Done"。
8. 再次点击"Configure"。
9. 点击"Generate"。
10. 在文件资源管理器中，导航到build目录，并双击新创建的`.sln`项目。
11. 在Visual Studio中进行构建。

如果项目成功克隆和构建，您可以使用操作系统的本机终端将图像简单地打印到文件中。


### 运行程序

在Linux或OSX上，从终端运行如下命令：

    $ build/inOneWeekend > image.ppm

在Windows上，运行如下命令：

    build\debug\inOneWeekend > image.ppm

或者，运行优化版本（如果您使用`--config release`进行构建）：

    build\release\inOneWeekend > image.ppm

生成的PPM文件可以直接作为常规计算机图像查看，如果您的操作系统支持该图像类型。如果您的系统不支持PPM文件，则可以在网上找到PPM文件查看器。我们推荐使用[ImageMagick][]。


更正和贡献
----------------------------
如果您发现错误，有建议的更正，或者想要帮助这个项目，
_**请查阅[CONTRIBUTING][]文档以了解最有效的操作方式。**_



[book1]:           books/RayTracingInOneWeekend_cn.html
[book2]:           books/RayTracingTheNextWeek.html
[book3]:           books/RayTracingTheRestOfYourLife.html
[CONTRIBUTING]:    CONTRIBUTING.md
[cover1]:          images/cover/CoverRTW1-small.jpg
[cover2]:          images/cover/CoverRTW2-small.jpg
[cover3]:          images/cover/CoverRTW3-small.jpg
[discussions]:     https://github.com/RayTracing/raytracing.github.io/discussions/
[GitHub home]:     https://github.com/RayTracing/raytracing.github.io/
[ImageMagick]:     https://imagemagick.org/
[implementations]: https://github.com/RayTracing/raytracing.github.io/wiki/Implementations
[milestone 16]:    https://github.com/RayTracing/raytracing.github.io/milestone/16
[milestone 19]:    https://github.com/RayTracing/raytracing.github.io/milestone/19
[PRINTING_CN.md]:  PRINTING_CN.md
[v3.2.3]:          https://github.com/RayTracing/raytracing.github.io/releases/tag/v3.2.3
[web1]:            https://raytracing.github.io/books/RayTracingInOneWeekend.html
[web1-v3]:         https://raytracing.github.io/books/v3/RayTracingInOneWeekend.html
[web2]:            https://raytracing.github.io/books/RayTracingTheNextWeek.html
[web2-v3]:         https://raytracing.github.io/books/v3/RayTracingTheNextWeek.html
[web3]:            https://raytracing.github.io/books/RayTracingTheRestOfYourLife.html
[web3-v3]:         https://raytracing.github.io/books/v3/RayTracingTheRestOfYourLife.html