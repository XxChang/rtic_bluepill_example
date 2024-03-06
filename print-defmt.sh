#! /usr/bin/env bash

set -ep

TARGET=${1}

echo "flashing target"
probe-rs download --chip STM32F103C8 ${TARGET}

echo "reset target"
probe-rs reset --chip STM32F103C8

echo "Attaching defmt-print.."
socat /dev/ttyUSB0,b9600,raw,echo=0 STDOUT | defmt-print -e $1
