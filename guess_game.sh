#!/bin/bash          
# 猜字小游戏

echo "欢迎来到猜字小游戏！"
echo "请输入一个1到100的数字："

# 生成随机数
random_num=$((1 + RANDOM % 100))

# 计数器
count=0

while true
do
    read guess_num
    ((count++))
    if ! [[ $guess_num =~ ^[0-9]+$ ]] ; then
        # 判断是否为数字，非数字则提醒并重新输入
        echo "请输入一个1到100的数字："
    elif (( $guess_num == $random_num )) ; then
        # 猜对了，输出结果，并退出
        echo "恭喜你猜对了！共猜了$count 次。"
        exit 0
    elif (( $guess_num > $random_num )) ; then
        # 猜大了，提醒并重新输入
        echo "猜大了！请再次输入："
    else
        # 猜小了，提醒并重新输入
        echo "猜小了！请再次输入："
    fi
done