*题目来自《Rust 程序设计语言》第 8 章 常见集合*

## 要求
将字符串转换为 Pig Latin [^*] ，也就是每一个单词的第一个辅音字母被移动到单词的结尾并增加「ay」，所以「first」会变成「irst-fay」。元音字母开头的单词则在结尾增加「hay」（「apple」 会变成「apple-hay」）。

[^*]: **Pig Latin**（或译儿童黑话、猪式拉丁话）是一种英语语言游戏，形式是在英语上加上一点规则使发音改变。据说是由在德国的英国战俘发明来瞒混德军守卫的。Pig Latin 于 1950 年代和 1960 年代在英国利物浦达到颠峰，各种年纪和职业的人都有使用。Pig Latin 多半被儿童用来瞒着大人秘密沟通，有时则只是说着好玩。虽然是起源于英语的游戏，但是规则适用很多其他语言。——维基百科

## 代码实现
```
fn main() {
    println!("{}", pig_latin(&String::from("first"))); 
    println!("{}", pig_latin(&String::from("apple"))); 
}


fn pig_latin(s: &String) -> String {
    let v = vec!["a", "e", "i", "o", "u"];
    let h = &s[0..1];
    let e = &s[1..];
    
    match v.contains(&h) {
        true => format!("{}-{}", s, "hay"),
        false => format!("{}-{}{}", e, h, "ay"),
    }
}
```
