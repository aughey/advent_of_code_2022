day=$1

if [ "$day" = "" ]; then
    echo "Usage: $? DAY"
    exit 1
fi

if [ -d $day ]; then
    echo "Day $day already exists"
    exit 2
fi

cp -r template $day
for file in `find $day -type f`; do
    sed -i "s/aoc/aoc$day/g" $file
done