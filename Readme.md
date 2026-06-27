[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-22041afd0340ce965d47ae6ef1cefeee28c7c493a6346c4f15d667ab976d596c.svg)](https://classroom.github.com/a/5pSShGos)
# 需求2： 支持倒车和加速功能

Executor组件增加执⾏倒车和加速指令：
**B:  倒车指令，接收到该指令，车进入倒车状态，该状态下：**
M：在当前朝向上后退一格，朝向不变。注：比如朝向为N时收到M指令，y坐标减1，朝向保持N。
L：右转90度，位置不变
R：左转90度，位置不变

**F:  加速指令，接收到该指令，车进入加速状态，该状态下：**
M：前进2格（不能跳跃，只能一格一格前进）
L：先前进1格，然后左转90度
R：先前进1格，然后右转90度

**B和F两个状态可以叠加，叠加状态下：**
M：倒退2格（不能跳跃，只能一格一格后退）
L：先倒退一格，然后右转90度
R：先倒退一格，然后左转90度

再接收一次B或者F指令，对应的状态取消