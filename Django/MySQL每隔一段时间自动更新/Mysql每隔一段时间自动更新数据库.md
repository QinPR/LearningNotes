### 使用MySQL每隔一段时间自动更新数据库

---

+ 动机：做软工作业的时候，我们的推荐系统需要根据blog的热度给用户推荐blogs。这就需要我们实时更新热度，这个需要数据库每隔一段时间自动完成。

SQL代码：

```sql
DROP EVENT IF EXISTS refresh_hot; 

CREATE EVENT refresh_hot 
ON SCHEDULE EVERY 1 day STARTS DATE_ADD(DATE(CURDATE() + 1),  INTERVAL 0 HOUR)
ON COMPLETION PRESERVE

DO
  UPDATE `CSC4001`.`Our_project_blog_questions` SET `hot` = `hot`/2;
```

+ 这样就可以create出一个EVENT让数据库在每天0点自动将`hot`减半。`ON COMPLETION PRESERVE`的意思是让它永久执行。
+ 参考：https://www.jianshu.com/p/91773f8375eb

