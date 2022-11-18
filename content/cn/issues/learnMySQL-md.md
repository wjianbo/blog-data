---
title: SQL 学习笔记
date: 2020-01-13 19:59:37
tags: [SQL]
---
2020年1月13日学習内容
[关系数据库概述 - 廖雪峰的官方网站](https://www.liaoxuefeng.com/wiki/1177760294764384/1179613436834240)
## 安装 mySQL
从 MySQL 官方网站下载最新的 MySQL Community Server 版本：
https://dev.mysql.com/downloads/mysql/
## 运行 MySQL
Mac：
```
cd /usr/local/mysql/bin
./mysql -u root -p
```
mysql的可执行命令在/usr/local/mysql/bin目录下，而这个目录不在普通用户的环境路径下，因此每次执行时都需要输入完整路径，比较麻烦。因此需要将这个路径添加到环境变量中。首先我们需要创建一个 .bash_profile 文件（不要掉了文件名前面的 . 号），命令如下：
```
touch .bash_profile
```
创建完成后，用编辑器打开这个文件，添加下面的内容：
```
export PATH=/usr/local/mysql/bin:$PATH
```
保存后退出

## 准备数据
下载 [SQL 脚本](https://raw.githubusercontent.com/michaelliao/learn-sql/master/mysql/init-test-data.sql)，在命令行运行：
```
$ mysql -u root -p < init-test-data.sql
```
就可以自动创建 test 数据库，并且在 test 数据库下创建 students 表和
classes 表，以及必要的初始化数据。
## 使用数据库
```
mysql> USE test;
```

## 基本查询
```
SELECT * FROM <表名>
```
## 条件查询
```
SELECT * FROM <表名> WHERE <条件表达式>
SELECT * FROM <表名> WHERE <条件1> AND/OR <条件2>
SELECT * FROM <表名> WHERE NOT <条件>
SELECT * FROM <表名> WHERE score BETWEEN 60 AND 90
```
要组合三个或者更多的条件，就需要用小括号()表示如何进行条件运算。
如果不加括号，条件运算按照 NOT、AND、OR 的优先级进行。
## 投影查询
```
SELECT 列1, 列2, 列3 FROM …
SELECT 列1 别名1, 列2 别名2, 列3 别名3 FROM …
```
## 排序
```
SELECT id, name, gender, score FROM students ORDER BY score (DESC);
```
## 分页查询
```
LIMIT <M> OFFSET <N>
```
在 MySQL 中，
```
LIMIT 15 OFFSET 30
```
还可以简写成
```
LIMIT 30, 15
```
## 聚合查询
```
SELECT COUNT(*) num FROM <表名>
```
除了 COUNT() 函数外，SQL 还提供了如下聚合函数：
* SUM
计算某一列的合计值，该列必须为数值类型
* AVG
计算某一列的平均值，该列必须为数值类型
* MAX
计算某一列的最大值
* MIN
计算某一列的最小值

通过聚合查询获得总页数：
```
 SELECT CEILING(COUNT(*) / 3) FROM students;
```
### 分组
```
SELECT class_id, COUNT(*) num FROM students GROUP BY class_id;
SELECT class_id, gender, COUNT(*) num FROM students GROUP BY class_id, gender;
```
## 多表查询
```
SELECT
    s.id sid,
    s.name,
    s.gender,
    s.score,
    c.id cid,
    c.name cname
FROM students s, classes c
WHERE s.gender = 'M' AND c.id = 1;
```
## 连接查询
```
SELECT s.id, s.name, s.class_id, c.name class_name, s.gender, s.score
FROM students s
INNER JOIN classes c
ON s.class_id = c.id;
```
注意 INNER JOIN 查询的写法是：
* 先确定主表，仍然使用 FROM <表1> 的语法；
* 再确定需要连接的表，使用 INNER JOIN <表2> 的语法；
* 然后确定连接条件，使用 ON <条件…>，这里的条件是 s.class_id = c.id，表示 students 表的 class_id 列与 classes 表的 id 列相同的行需要连接；
* 可选：加上 WHERE 子句、ORDER BY 等子句。

INNER JOIN、RIGHT OUTER JOIN、LEFT OUTER JOIN、FULL OUTER JOIN 的区别：
* INNER JOIN 只返回同时存在于两张表的行数据，由于 students 表的 class_id 包含 1，2，3，classes 表的 id 包含 1，2，3，4，所以，INNER JOIN 根据条件 s.class_id = c.id 返回的结果集仅包含 1，2，3。
* RIGHT OUTER JOIN 返回右表都存在的行。如果某一行仅在右表存在，那么结果集就会以 NULL 填充剩下的字段。
* LEFT OUTER JOIN 则返回左表都存在的行。
* FULL OUTER JOIN 会把两张表的所有记录全部选择出来，并且，自动把对方不存在的列填充为 NULL。

---
参考：
[如何在 Macbook 上安装 MySQL-百度经验](https://jingyan.baidu.com/article/fa4125ac0e3c2928ac709204.html)
[bash - .bash_profile: bad assignment - Stack Overflow](https://stackoverflow.com/questions/51819285/bash-profile-bad-assignment)