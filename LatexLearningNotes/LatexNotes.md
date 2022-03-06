### Latex Learning Notes

---

#### Small backgound

在大一大二的时候我有接触过Latex，但并没有系统地学习Latex并且用Latex写过什么东西（一般都是用markdown）。

我是出于两个动机决定系统地学习Latex的，第一是为了以后写论文的需要：听说一般一些会议会给一个Latex的模板，然后写论文的时候按这个模板来写，并且听学长说Latex用熟了以后写论文会很爽，这让我跃跃欲试。第二个原因是我这学期上的DDA2020课程的作业老师给的作业sample是Latex，TA也说这门课的作业用Latex写会更好（《给更高分》），所以乘此系统地学习下如何使用Latex。

---

#### Hello WORLD

+ 我用的是Overleaf, 一个线上的latex编辑器，在创建一个Project后就可以开始第一份latex文档了。

```Latex
\documentclass{article}
\begin{document}
Hello world! 
\end{document}
```

+ 这里documentcalss{article}中的参数可以替换为book, report。决定了整个文档的类型。

---

#### Preamble介绍

+ 写在Latex最前面的，我的直观理解是类似调用库。

+ 比如：

  ```Latex
  \documentclass[12pt, letterpaper]{article}
  \usepackage[utf8]{inputenc}
  ```

  + 12pt: 字体大小（默认10，还可以是9，11，12）
  + letterpaper页面的大小（还可以是a4paper, legalpaper）
  + utf8:编码方式

---

#### 标题，作者，日期

```latex
\documentclass[12pt, letterpaper, twoside]{article}
\usepackage[utf8]{inputenc}

\title{My First document}
\author{Peiran Qin \thanks{the tutorials from overleaf.com}}
\date{1st March 2022}

\begin{document}
\maketitle

Hello world 
\end{document}
```

+ twoside: 就是双面的意思，像一本书的字可以印在纸的前后面
+ 记得在下面\maketitle, 这样title才会显示出来
+ 效果：

![image-20220301114626960](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220301114626960.png)

---

#### 备注

Latex的comments用%

```latex
% This line here is a comment. It will not be printed in the document
```

---

#### 加粗，斜体，下划线

```latex
% First way of adding bold, italic, under line
Latex is \textbf{fun}.
coding is \underline{tedious}.
coding plus Latex is \textbf{\textit{wow}}.

% Adding emphasize
\textbf{I am very \emph{hungry} now}
```

+ \emph的意思是强调，一般来说默认是等同于加斜体

![image-20220301115408421](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220301115408421.png)

---

#### 插入图片

+ 加入Preamble:

  ```latex
  \usepackage{graphicx}
  \graphicspath{{images/}}
  ```

  + images是指同级下的一个文件夹，要插入的图片存在里面，我在里面放了一个名为dictionary.JPG的图片

  ```latex
  \includegrahttps{dictionary.JPG}
  ```

![image-20220301115953943](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220301115953943.png)

+ 给图片标注，调整图片大小，文字中引用图片的标注

	```latex
	% Adding images
	\begin{figure}[h]
    \centering
    \includegraphics[width=0.25\textwidth]{dictionary.JPG}
    \caption{a picture of youdao dictionary}
    \label{fig:dic}
	\end{figure}
	As you can see in the figure \ref{fig:dic}, this is a picture of a electronic dictionary.
	```
	
	+ `\caption{a picture of youdao dictionary}`: 在图片下方给个标注
	+ `\label{fig:dic}`: 给图片打个标签，方便在后文引用
	+ `\ref{fig:dic}`: 通过标签引用图片
	+ 效果：
	
	![image-20220301121152132](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220301121152132.png)

---

#### 加入有序/无序列表

```latex
% creating unordered lists
\begin{itemize}
    \item DDA2020
    \item CSC3180
    \item CSC4001
    \item GEB2504
\end{itemize}
% creating ordered lists
\begin{enumerate}
    \item assignments
    \itme projects
    \item pre
    \item exams
\end{enumerate}
```

效果：

![image-20220301121542323](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220301121542323.png)

---

### 数学公式

+ 夹杂在文字间的数学公式：

  ```latex
  formulation: $E = mc^2 $, raised by Albert.
  ```

+ 单独在行外的数学公式：

  ```latex
  \usepackage{amsmath}
  ```

  ```latex
  \begin{equation}
  \begin{aligned}
      f(x) &= x_1 + x_2^2\\
      &= x_1 + x_3\\
      g(x) &= x_{100} + 2
  \end{aligned}
  \end{equation}
  ```

  + `&=`: 保证等号是对其的

  效果：

  ![image-20220301123132550](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220301123132550.png)

---

#### 新的段落

+ 加入新段落不等于加入新的行，因为段落间会有更大的空格：

  ```latex
  \usepackage{parskip}
  ```

  ```latex
  % adding new lines and new paragraph
  This is the paragraph one, it mainly talk about something meanless to show an example about the difference between making a new line and a new para.
  newline without starting a new para
  
  
  This is a new para
  ```

  + 直接回车换行
  + 通过加两个空行来起新段落

---

#### 摘要

```latex
\begin{document}
\begin{abstract}
    This is a very short abstract with no meaningful contents to just show the function of abstract.
\end{abstract}
\end{document}
```

效果：

![image-20220301132836947](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220301132836947.png)

---

#### Sections功能

```latex
\section{Introduction}
This is the Introduction section. In this section, the passage will briefly talk about the basic idea of our work.

\section{Literature review}
In this part, we will introduce some important work conducted previously.

\subsection{Beer's work}
Beer conduct the project in Chile but fail in the end.

\subsection{Albert's work}
Albert succeeded to be a great scientist in the world.
```

效果：

![image-20220301151922732](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220301151922732.png)

---

#### 表格

```latex
% creating tables
\begin{center}
    \begin{tabular}{c c c}
    a & b & c \\
    d & e & f
    \end{tabular}
\end{center}
```

+ `c`: 代表居中，还可以是r 或 l

效果：

![image-20220301152434240](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220301152434240.png)

+ 加上边界线 \hline

```latex
% creating tables
\begin{center}
    \begin{tabular}{|c| c |c|}
    \hline
    a & b & c \\
    \hline
    d & e & f \\
    \hline
    \end{tabular}
\end{center}
```

效果：

![image-20220301152712712](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220301152712712.png)

---

#### 目录

```latex
\begin{document}
% making title appear
\maketitle

% table of contents
\tableofcontents
```

+ `\tableofcontents`: 加在一开头，latex就会根据下文中的section，subsection做出目录。

![image-20220301153022082](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220301153022082.png)

---

#### 双/多栏

```latex
\documentclass[twocolumn, 12pt, letterpaper, twoside]{article}
```

![image-20220301153347933](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220301153347933.png)

---

#### 代码总结

```latex
\documentclass[twocolumn, 12pt, letterpaper, twoside]{article}
\usepackage[utf8]{inputenc}
\usepackage{graphicx}
\usepackage{amsmath}
\usepackage{parskip}
\graphicspath{{images/}}

% adding title
\title{My First document}
\author{Peiran Qin \thanks{the tutorials from overleaf.com}}
\date{1st March 2022}

\begin{document}
% making title appear
\maketitle

% table of contents
\tableofcontents

% abstract
\begin{abstract}
    This is a very short abstract with no meaningful contents to just show the function of abstract.
\end{abstract}

Hello world.

% adding new lines and new paragraph
This is the paragraph one, it mainly talk about something meanless to show an example about the difference between making a new line and a new para.
newline without starting a new para


This is a new para

% First way of adding bold, italic, under line
Latex is \textbf{fun}.
coding is \underline{tedious}.
coding plus Latex is \textbf{\textit{wow}}.

% Adding emphasize
\textbf{I am very \emph{hungry} now}

% Adding images
\begin{figure}[h]
    \centering
    \includegraphics[width=0.25\textwidth]{dictionary.JPG}
    \caption{a picture of youdao dictionary}
    \label{fig:dic}
\end{figure}
As you can see in the figure \ref{fig:dic}, this is a picture of a electronic dictionary.

% creating unordered lists
\begin{itemize}
    \item DDA2020
    \item CSC3180
\end{itemize}
% creating ordered lists
\begin{enumerate}
    \item assignments
    \item exams
\end{enumerate}

% inline mode
There is a famous formulation: $E = mc^2 $, raised by Albert.
% outline mode
There are some formulations:
\begin{equation}
\begin{aligned}
    f(x) &= x_1 + x_2^2//
    &= x_1 + x_3//
    g(x) &= x_{100} + 2
\end{aligned}
\end{equation}


\section{Introduction}
This is the Introduction section. In this section, the passage will briefly talk about the basic idea of our work.

\section{Literature review}
In this part, we will introduce some important work conducted previously.

\subsection{Beer's work}
Beer conduct the project in Chile but fail in the end.
\paragraph{first para}
I wonder if this paragraph could work?

\subsection{Albert's work}
Albert succeeded to be a great scientist in the world.

% creating tables
\begin{center}
    \begin{tabular}{|c| c |c|}
    \hline
    a & b & c \\
    \hline
    d & e & f \\
    \hline
    \end{tabular}
\end{center}

\end{document}

```

