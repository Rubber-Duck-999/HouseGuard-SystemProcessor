#!/bin/sh

cd $HOME/Documents/HouseGuard-SystemProcessor

git pull

cargo build

if [ -f target/debug/exeSystemProcessor ];
then
    echo "SYS File found"
    if [ -f $HOME/Documents/Deploy/exeSystemProcessor ];
    then
        echo "SYS old removed"
        rm -f $HOME/Documents/Deploy/exeSystemProcessor
    fi
    cp target/debug/exeSystemProcessor $HOME/Documents/Deploy/exeSystemProcessor
fi
