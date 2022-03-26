#!/bin/bash

file="$1"

cat $file | while read -r line
do
  echo $line | sed 's/\(.*\) = \(.*\),/\2 => OpCode::\1,/g'
done
