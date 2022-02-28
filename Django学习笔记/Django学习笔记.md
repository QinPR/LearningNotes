### Django学习笔记

以下为Windows环境

---

#### 0. 下载Django

```bash
pip install Django
```

---

#### 1. Hello World：在VScode上创建并运行一个Django项目

+ 创建一个文件夹，并在VScode中打开文件夹，在VSC中打开terminal

+ 创建一个项目：

  ```bash
  django-admin startproject SW_Project
  cd SW_Project
  ```

  创建一个app:

  ```bash
  python manage.py startapp Our_project
  cd Our_project
  mkdir templates    //在项目建立一个新的文件夹
  ```

  会看到这样的项目结构：

  ![image-20220223205022225](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220223205022225.png)

  ![image-20220225161945453](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220225161945453.png)

  (图源：见水印)

+ 项目运行：

  ```bash
  python manage.py runserver
  ```

  ![image-20220223190801558](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220223190801558.png)

  若运行成功会返回一个url（我这里返回的是http://127.0.0.1:8000），证明这个项目在本地的服务器上跑起来了，打开这个即可看到页面。

---

#### 3. 建一个简单的网站

（1）配置一下setting

​		setting.py中加入刚刚创的app来告诉框架加入了这样一个新的app

![image-20220223205620018](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220223205620018.png)

(2) 上手

+ 首先在/Our_Project/templates中创建一个html文件作为网页要展示的页面

  ![image-20220223210701837](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220223210701837.png)

  接着在/Our_project/views中写一个函数，这个函数会调用这个html文件：

  ![image-20220223210852342](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220223210852342.png)

  + 这里的参数request代表从前端发送过来的请求（之后会用到）

  这个函数又会被url调用，所以去/SW_Project/urls加上，**这样当我们在浏览器中输入对应的url，就会自动调用index这个函数，接着这个函数又会返回index.html给网页**

  ![image-20220223211233886](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220223211233886.png)

  （记得import views）

  最后，在浏览器中输入对应的url（localhost:8000/index）就能看到效果:

  ![image-20220223211344030](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220223211344030.png)

---

#### 4. 与前端交互

刚刚做的仅仅只是后端在自娱自乐，并没有和前端的交互，一般情况下前端会发送数据（比如输入用户名和密码）给后端，后端做出处理后再返回结果给前端。

这里，简单地做一个登陆页面来展示后端如何处理前端数据并与前端交互。

(1) 登陆页面：

+ 修改刚刚的Our_project/templates/index.html

  ![image-20220223220736032](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220223220736032.png)

  这里前端会以stuid和passcode两个变量得到用户输入的两个变量，然后把url变为localhost:8000/login，然后用POST的方式发送给后端。

+ 后端的处理从url开始，先在/SW_Project/urls.py添加localhost:8000/login和views.py中函数的连接：

  ![image-20220223221057500](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220223221057500.png)

  前端的请求将会由views.py中的login函数处理，所以接下来写login函数。

+ ![image-20220223221241468](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220223221241468.png)

  用request.POST['XXX']得到前端发送过来的请求中的变量。判断密码是否为12345，如果是则返回登陆成功的主页面main_page.html，并带上变量'stuid'(变量值为id)。

+ 接着在templates/下写个main_page.html的页面：

  ![image-20220223221531412](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220223221531412.png)

  + 注意，这里用{{变量名}}的方法接收后端发送回来的变量。

+ 整个流程可以概括为：

  index.html中接受用户输入，并改变url，发送数据 **->** urls.py将对应url映射到views.py的login函数上 **->** login函数做出处理，返回新的页面main_page.html,并传回变量stuid **->** main_page.html接受stuid并打印

+ 最终效果是：

  ![image-20220223222009418](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220223222009418.png)

​	点提交后跳转到：

​		![image-20220223222023951](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220223222023951.png)

---

#### 5. 与数据库交互

在这里演示如何跟连接MySQL并进行数据库操作

+ 先在mysql中创建一个空的schema

+ 首先在settings.py中将DATABASES改成要连接的数据库
  + ‘NAME’为schema的名称
  + ‘USER’, 'PASSWORD'对应数据库的用户密码
  + ‘HOST’, 'PORT'是数据库的ip和端口号，如果是本地的数据库就是127.0.0.1(mysql数据库默认用3306端口)

![image-20220223225629419](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220223225629419.png)

+ 在Our_project/_init__.py中加东西：

  ![image-20220223230047606](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220223230047606.png)

+ 接着最重要的一步：在/Our_projects/models.py中加入想加入数据库的表，比如我想加一个students的表，表里包括id和密码：

  ![image-20220223230224313](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220223230224313.png)

+ 接着我们将这个表migrate到mysql数据库中

  ```bash
  python manage.py makemigration
  python manage.py migrate
  ```

  将会看到mysql中原本sw_db的tables里原本是空的，现在多了Our_project_students这个table：

  ![image-20220223230543872](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220223230543872.png)

+ 成功与数据库建立连接并加入了表，接下来进行一个简单的往表里加一条信息的操作，基于上面views.py里修改一下，让它在登陆的时候，将id和passcode加入到数据库student的表中：

  + 先把表import进来：

  ![image-20220223230828330](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220223230828330.png)

  + 接着在login函数中加一行create的代码：

  ![image-20220223230913010](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220223230913010.png)

+ 效果是，当用户输入id，passcode登陆后，可以在数据库中看到多了一个条目：

  ![image-20220223231026687](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220223231026687.png)

