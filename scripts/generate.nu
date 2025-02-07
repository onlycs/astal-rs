# cd gir/gir-ffi
# cargo install --path .
# cd ../..

ls sys
| get name
| each { |folder|
    cd $folder
	print -e $"Entering ($folder)"
  	rm Cargo.*
    gir -o .
}
| ignore

ls crates
| where name != "crates/gir-error"
| get name
| each { |folder|
    cd $folder
	print -e $"Entering ($folder)"
    gir -o .
}
| ignore
