#!/bin/sh -e
mydir="${0%/*}"
cd $mydir
docker build -f Dockerfile_docker_run -t localhost/docker_run:latest
