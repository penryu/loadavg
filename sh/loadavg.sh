#!/bin/sh

sysname=$(uname -s)

case $sysname in
  *BSD | Darwin )
    sysctl vm.loadavg | cut -d' ' -f3,4,5
    ;;
  Linux)
    cut -d' ' -f1,2,3 < /proc/loadavg
    ;;
  *)
    echo "Unknown system:" "$sysname"
esac
