#!/bin/sh

genuuids(){
    uuidfile=./sample.d/uuids.dat

    test -f "${uuidfile}" && rm "${uuidfile}"

    printf \
        '%s-%s-%s-%s-%s' \
            dafef00d \
            dead \
            beaf \
            face \
            864299792458 |
    xxd \
        -plain \
        -revert |
    cat >> "${uuidfile}"

    printf \
        '%s-%s-%s-%s-%s' \
            cafef00d \
            dead \
            beaf \
            face \
            864299792458 |
    xxd \
        -plain \
        -revert |
    cat >> "${uuidfile}"

    printf \
        '%s-%s-%s-%s-%s' \
            dafef00d \
            dead \
            beaf \
            face \
            864299792458 |
    xxd \
        -plain \
        -revert |
    cat >> "${uuidfile}"

}

genuuids

cat ./sample.d/uuids.dat |
    wazero run ./rs-uniq4uuid.wasm |
    xxd
