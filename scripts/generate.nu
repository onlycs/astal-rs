cd gir/gir-ffi
cargo install --path .
cd ../..

ls sys
| get name
| each { |folder|
    cd $folder
    rm Cargo.*
    gir -o .
}
| ignore

ls crates
| where name != "crates/gir-error"
| get name
| each { |folder|
    cd $folder
    gir -o .
}
| ignore
