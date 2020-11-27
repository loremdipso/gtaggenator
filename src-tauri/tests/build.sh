#!/bin/bash

#go build -o taggenator ./src
#cd src && go build -o ../taggenator -mod=vendor
cd ../ && go build -o ./tests/taggenator .
