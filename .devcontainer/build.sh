#!/bin/bash

set -e

MODE=${1:---debug}
TYPE=${2:-linux}
MODE=${MODE/*-/}


build(){
    pwd
    $WORKDIR/entrypoint $1
}

build_arm64(){
    CWD=$(pwd)
    cd $WORKDIR
    $WORKDIR/flutter/ndk_arm64.sh
    cp $WORKDIR/target/aarch64-linux-android/release/liblibrustdesk.so $WORKDIR/flutter/android/app/src/main/jniLibs/arm64-v8a/librustdesk.so
    cd $CWD
}

build_apk(){
    cd $WORKDIR/flutter
    MODE=$1 $WORKDIR/flutter/build_android.sh
    cd $WORKDIR
}

key_gen(){
    if [ ! -f $WORKDIR/flutter/android/key.properties ]
    then
        if [ ! -f $HOME/upload-keystore.jks ]
        then
            echo "Remember the password you enter in keytool!"
            keytool -genkey -v -keystore $HOME/upload-keystore.jks -keyalg RSA -keysize 2048 -validity 10000 -alias upload
        else
            read -r -p "enter the password used to generate $HOME/upload-keystore.jks" password
            echo -e "storePassword=${password}\nkeyPassword=${password}\nkeyAlias=upload\nstoreFile=$HOME/upload-keystore.jks" > $WORKDIR/flutter/android/key.properties
        fi
    fi
}

android_build(){
    if [ ! -d $WORKDIR/flutter/android/app/src/main/jniLibs/arm64-v8a ]
    then
        $WORKDIR/.devcontainer/setup.sh android
    fi
    build_arm64
    case $1 in
        debug)
        build_apk debug
        ;;
        release)
        key_gen
        build_apk release
        ;;
    esac
}

case "$MODE:$TYPE" in
    "debug:linux")
    build
    ;;
    "release:linux")
    build --release
    ;;
    "debug:android")
    android_build debug
    ;;
    "release:android")
    android_build release
    ;;
esac
