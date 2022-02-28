### Django连接服务器上的数据库

---

服务器：Ubuntu-20-8

#### 1. 在服务器上安装MySQL

```bash
sudo apt-get install mysql-server
```

+ 启动MySQL服务器

```bash
sudo systemctl start mysql
```

+ 设置密码

```bash
sudo mysql_secure_installation
```

+ 连接上服务器

```bash
sudo mysql -p
```

+ 创建一个schema试试

```mysql
> Create SCHEMA SW_DB;
> show databases;
```

(其实这个过程中遇到了好几个很奇怪的报错，不太具有代表新，我直接把报错信息在google上搜索然后按着步骤来就解决了)

+ 开放连接mysql的ip权限（即所有外界的主机都可以连接mysql,这步对用Django远程连接数据库很关键）

  + 在服务器终端上输入：

  ```bash
  sudo vi /etc/mysql/mysql.conf.d/mysqld.cnf
  ```

  进去对bind-address进行修改：

  ![image-20220224170246222](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220224170246222.png)

  + 重启mysql

  ```bash
  sudo service mysql restart
  ```

---

#### 2. 在本地的MySQL Workbench上远程连接数据库

如果单用命令行来操作服务器上的数据库那可太糟心了。。为了可视化数据库，我们用本地的MySQL Workbench来远程连接服务器上的数据库。（MySQL Workbench的安装跳过）

+ 先点击MySQL Connections旁边的加号：

![image-20220224152741854](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220224152741854.png)

+ 设置用ssh连接：

![image-20220224152900978](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220224152900978.png)

​	这里175.178.34.84是租的一个服务器的IP地址，可以按上图的配置配好，然后test connection. (这里所有的密码都是Q@@pr294118).

+ 搞定后进去发现，整个数据库都可视化了！

![image-20220224153228740](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220224153228740.png)

---

#### 3. 在Django项目上连接数据库

+ 进入setting.py将DATABASE改成：

![image-20220224170500742](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220224170500742.png)

+ 终端输入：

  ```bash
  python manage.py migrate
  python manage.py runserver
  ```

+ 打开网页，并登陆（如果报错密码就输12345）

+ 用MySQL Workbench连上数据库后会发现，数据库中Our_project_students这个table中加入了刚刚输入的内容：

  ![image-20220224170828097](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220224170828097.png)

