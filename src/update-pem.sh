#!/bin/bash

outdir="$(realpath .)"
outfile="${outdir}/pem.rs"
pem_outfile="${outdir}/mozilla.pem"

set -e
set -x

type openssl
type mktemp
type curl
type base64
type mkdir
type ls
type sort
type rm
type mv

export LC_ALL=C

tmp="$(mktemp -d -t mozillaRootCaUpdater.XXXXXXXX)"
trap "rm -rfv $tmp" EXIT
cd $tmp

curl https://curl.se/ca/cacert.pem -v -4 -L -o mozilla.pem
pem="$(realpath mozilla.pem)"

mkdir mozilla
cd mozilla

cat ../mozilla.pem | awk 'split_after==1{n++;split_after=0}
   /-----END CERTIFICATE-----/ {split_after=1}
   {if(length($0) > 0) print > "" (1+n) ".pem"}'

echo "pub const PEM_BUNDLE: &'static str = include_str!(\"mozilla.pem\");" > rs.tmp.out
echo "pub const PEM_LIST: &'static [ &'static str ] = &[" >> rs.tmp.out

for cert in $(ls *.pem | sort -g)
do
    echo "/*"
    openssl x509 -in $cert -noout -serial -issuer -dates -sha1 -fingerprint
    openssl x509 -in $cert -noout -sha256 -fingerprint
    echo "*/"
    echo "\"$(openssl x509 -in $cert -outform pem | xxd -i | sed s/'0x0a'/'0x5c, 0x6e'/g | xxd -r -p)\","
    echo -e "\n"
done >> rs.tmp.out
echo "];" >> rs.tmp.out

mv rs.tmp.out $outfile
mv $pem $pem_outfile
