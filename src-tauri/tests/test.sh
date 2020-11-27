#!/bin/bash

printf "\ec"
#./build.sh && cd tmp && time ../taggenator open <<< "-some other tag, yolo baggins-"
#./build.sh && cd tmp && time ../taggenator open -sort unseen
#./build.sh && cd tmp && time ../taggenator open
#cp ./tmp/data.db.old ./tmp/data.db

#./build.sh && cd tmp && time ../taggenator <<< ""
#./build.sh && cd tmp && time ../taggenator dump_tags
cp ./tmp/data.db.old ./tmp/data.db
#./build.sh && cd tmp && time ../taggenator open <<< "tmp"
#./build.sh && cd tmp && time ../taggenator open unseen  <<< "tmp"
#./build.sh && cd tmp && time ../taggenator help -v <<< "tmp"
./build.sh && cd tmp && time ../taggenator apply_tags --tag --threads 20 silly <<< "tmp"
#./build.sh && cd tmp && time ../taggenator apply_tags --tag silly <<< "tmp"
#./build.sh && cd tmp && time ../taggenator apply_tags --tag silly <<< "tmp"
#./build.sh && cd tmp && time ../taggenator dump
#./build.sh && cd tmp && time ../taggenator dump search 7005 <<< ""

#./build.sh && cd tmp && time ../taggenator dump_tags
#./build.sh && cd tmp && time ../taggenator open search oopsie <<< ""

cd ..
cp ./taggenator ~/Downloads/Shared/
#./build.sh && cd tmp && time ../taggenator open_read_only
#./build.sh && cd tmp && time ../taggenator open search "temp" <<< ""
#./build.sh && cd tmp && time ../taggenator move -destination "/tmp" -sort search 004 <<< "ls"
#./build.sh && cd tmp && time ../taggenator open <<< "pre"
#./build.sh && cd tmp && time ../taggenator help
#./build.sh && cd tmp && time ../taggenator open <<< "y"
#./build.sh && cd tmp && time ../taggenator open <<< "y"
#./build.sh && cd tmp && time ../taggenator open
#./build.sh && cd tmp && time ../taggenator open <<< ""
#./build.sh && cd tmp && time ../taggenator open <<< "some tag, some other tag, some third tag, some fourth tag, some fifth tag, some sixth tax"
#./build.sh && cd tmp && time ../taggenator open <<< "< 5"
#./build.sh && cd tmp && time ../taggenator open <<< "rename:applesauce.txt"
#./build.sh && cd tmp && time ../taggenator open <<< "rename:AppleSauce.txt"
#./build.sh && cd tmp && time ../taggenator open -sort unseen -sort reverse <<< "rename:AppleSauce.txt"
#./build.sh && cd tmp && time ../taggenator open -sort unseen <<< "a"
#./build.sh && cd tmp && time ../taggenator dump_tags
#./build.sh && cd tmp && time ../taggenator help
#./build.sh && cd tmp && time ../taggenator open -sort search tagA tagB

# Multiple process tmp (failed)
#go build -o taggenator ./src/ && cd tmp
#../taggenator A &
#../taggenator B &
