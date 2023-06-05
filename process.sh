#!/bin/sh

v=`tail -1 ips.txt`
a="$(cut -d'.' -f1 <<<"$v")"
echo "$(($a * 100 / 256))"%
