# os-new-tutorial
## 完成情况
- 目前已完成前七章，大纲里要求的内容也基本完成。
- 目前的文件系统使用了rcore的sfs，故而第六章完成相对简单，未来会尝试实现自己的文件系统。
- 已完成的代码感觉逻辑还有些混乱，代码风格不太好，有些复制的代码直接拿过来，注释格式也比较乱，又有英文又有中文。
- 内存管理部分的代码有的也是使用了rcore tutorial里的代码，具体的细节理解还不到位，主要体现在rust语言的不熟练。
- 目前代码还存在很多问题，比如有些测试无法通过，有些bug定位很困难，影响不大，就没有处理，和别人的代码相比，自己的
还比较乱，整体设计和细节处理还不太好，未来改进。
## 感想
- 前七章内容大概花费了1个月的时间，和之前的rcore实验相比，明显吃力了很多，说明之前还是对写操作系统的细节不够了解，rust
语言的熟练程度还不够。
- 以语言来说，有时候会遇到有思路，知道该怎么写，但是使用rust语言就无从下手的感觉，不知道应该怎么通过编译，后续会通过
完成CS110L课程和继续学习一些rust语言的书籍和博客来提升rust语言的编程能力。
- 以大纲本身难度来讲，如果完全独立完成的话，还是有些困难的，因为以前做的lab相当于是填空，而新大纲相当于解答题，而且有些
部分描述的内容不够详细，具体实现细节需要思考很久。而以前的lab已经为你搭建好了代码框架，相对容易。
- 在改bug方面，新大纲由于自己搭建平台和框架，所以会遇到很多莫名其妙的问题，无法解决，定位也十分困难。感觉所有时间里面，有
一大半时间都是在找bug，修改bug，这也是新大纲比以前的lab困难的地方。
- 感觉在开始实验之前，先完成一个专门的有针对性的rust语言教程的练习比较好，比如相对底层的一些操作，各种库的使用，比如bitflag，
Arc，spin，lazystatic等，这样在做实验的时候会更加得心应手。
## 参考资料
- [吴一凡同学的 rCore-Tutorial-Book 第三版！](https://rcore-os.github.io/rCore-Tutorial-Book-v3/)
- [吴一凡同学的新大纲代码](https://github.com/rcore-os/rCore-Tutorial-v3/)
- [rcore toturial V3实验笔记](https://rcore-os.github.io/rCore-Tutorial-deploy/)
- [rcore toturial v3实验代码](https://github.com/rcore-os/rCore-Tutorial/)
- [rcore代码](https://github.com/rcore-os/rCore/)
- [wfly1998同学的2020日报](https://github.com/wfly1998/DailySchedule_2020/)
- [yunwei37同学的日报](https://github.com/yunwei37/os-summer-of-code-daily/)