#!/bin/bash
cd /home/svn
svnadmin create Test
htpasswd -c conf/passwd user user
cd / 
