rm target 2>/dev/null
bstow source target > /dev/null
find target | diff - ./expected-target.txt

rm target 2>/dev/null
bstow ./source target > /dev/null
find target | diff - ./expected-target.txt


rm target 2>/dev/null
bstow source ./target > /dev/null
find target | diff - ./expected-target.txt


rm target 2>/dev/null
bstow ./source ./target > /dev/null
find target | diff - ./expected-target.txt


rm target 2>/dev/null
bstow "$(pwd)/source" ./target > /dev/null
find target | diff - ./expected-target.txt


rm target 2>/dev/null
bstow "$(pwd)/source" target > /dev/null
find target | diff - ./expected-target.txt


rm target 2>/dev/null
bstow source "$(pwd)/target" > /dev/null
find target | diff - ./expected-target.txt


rm target 2>/dev/null
bstow ./source "$(pwd)/target" > /dev/null
find target | diff - ./expected-target.txt


rm target 2>/dev/null
bstow "$(pwd)/source" "$(pwd)/target" > /dev/null
find target | diff - ./expected-target.txt

