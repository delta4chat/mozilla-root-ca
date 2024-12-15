#!/bin/bash

set -e
set -x

type bash

bash update-pem.sh
bash update-der.sh

