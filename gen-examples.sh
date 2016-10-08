#!/bin/bash

set -e

main() {
    rm -rf src/examples

    mkdir src/examples

    echo '// Auto-generated. Do not modify!' > src/examples/mod.rs
    echo '//! Examples' >> src/examples/mod.rs
    echo >> src/examples/mod.rs

    local infile= name= outfile=
    for example in $(ls examples); do
        case $example in
            _* | *.rs.bk)
                continue
                ;;
        esac

        name=${example//-/_}
        name=${name%.*}
        outfile=src/examples/$name.rs
        infile=examples/$example

        echo "// Auto-generated. Do not modify this file! Instead modify $infile" > $outfile
        cat "$infile" | grep '//!' >> $outfile
        echo '//!' >> $outfile
        echo '//! ``` rust,no_run' >> $outfile
        cat "$infile" | grep -v '//!' | (
            IFS=''

            skip_whitespace=true
            while read line; do
                if [ -z $line ]; then
                    if [ $skip_whitespace = false ]; then
                        echo "//!"
                    fi

                    continue
                else
                    skip_whitespace=false
                fi

                echo "//! $line"
            done
        ) >> $outfile
        echo '//! ```' >> $outfile

        chmod 444 $outfile

        echo "pub mod $name;" >> src/examples/mod.rs
    done

    chmod 444 src/examples/mod.rs

    # Sanity check
    cargo doc
}

main

