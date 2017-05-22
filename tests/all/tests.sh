for f in `ls *.txt | sed 's/.txt//'`; do touch $f.json && cat $f.txt | cargo run | diff $f.json -; done
