#!/bin/sh

cd $HOME/Documents/HouseGuard-SystemProcessor

git pull
success=$?
echo $success

cargo clean

cargo build

if [ -f target/debug/exeSystemProcessor ];
then
    echo "SYS File found"
    if [ -f $HOME/Documents/Temp/exeSystemProcessor ];
    then
        echo "SYS old removed"
        rm -f $HOME/Documents/Temp/exeSystemProcessor
    fi
    cp target/debug/exeSystemProcessor $HOME/Documents/Temp/exeSystemProcessor
fi
