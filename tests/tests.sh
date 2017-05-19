for f in `ls *.txt | sed 's/.txt$//'`; do cat $f.txt | cargo run | (cd all; touch $f.json && diff $f.json -) || exit 1; done
