#!/bin/bash

outdir="$(realpath .)"
outfile="${outdir}/der.rs"
pem="${outdir}/mozilla.pem"

set -e
set -x

type openssl
type mktemp
type cp
type base64
type mkdir
type rm
type mv

tmp="$(mktemp -d -t mozillaRootCaUpdater.XXXXXXXX)"
trap "rm -rfv $tmp" EXIT
cd $tmp

cp $pem .

mkdir mozilla
cd mozilla

cat ../mozilla.pem | awk 'split_after==1{n++;split_after=0}
   /-----END CERTIFICATE-----/ {split_after=1}
   {if(length($0) > 0) print > "" (1+n) ".pem"}'

echo "use crate::*;" > rs.tmp.out
echo "pub const DER_LIST: &'static [ &'static [u8] ] = &[" >> rs.tmp.out

for cert in $(ls *.pem | sort -g)
do
    echo "/*"
    openssl x509 -in $cert -noout -serial -issuer -dates -sha1 -fingerprint
    openssl x509 -in $cert -noout -sha256 -fingerprint
    echo "*/"
    echo "&b64!(b\"$(openssl x509 -in $cert -outform der | base64 -w 0)\"),"
    echo -e "\n"
done >> rs.tmp.out

echo "];" >> rs.tmp.out

mv rs.tmp.out $outfile
