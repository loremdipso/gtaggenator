#!/bin/bash

filewatcher '../**/*.go' 'printf "\ec" && ./test.sh'
#filewatcher '**/*' 'printf "\ec" && cd test && go run ../src/'
