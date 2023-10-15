#!/bin/bash

NPROC=$(sensors -A | grep "Core" | wc -l)
NSEQ=$(( $NPROC + 1 ))
SEQ=$(seq 2 ${NSEQ})
CS="21 27 33 39 45 51 50 49 48 47 46 82 118 154 190 226 220 214 208 202 196 255"
RS=`tput sgr0`
NID=0

die() {
	echo "Error. Dependency $@ not found."
	exit 1
}

checks() {
	DEPS="jq sensors"

	for DEP in $DEPS; do
		[ "$(command -v ${DEP})" ] || die ${DEP}
	done
}

checks

#This array maps color IDs to order them in a consistent sequence.
declare -a c
for ID in $CS; do
	NID=$(( $NID + 1 ))
	c[$NID]=`tput setaf $(echo "$ID")`
done

#It calculates array variable ID out of temperature value and applies the color.
colorize () {
	if [ $CT -le 38 ]; then
        echo "${c[1]}$CT${RS} °C"
	elif [ $CT -ge 82 ]; then
        echo "${c[21]}$CT${RS} °C"
	else
        MUN=$(( $CT - 40 ))
        MUN=$(( $MUN / 2 ))
        MUN=$(( $MUN + 1 ))
        echo "${c[$MUN]}$CT${RS} °C"
	fi
}

while true; do
SENSORS=$(sensors -Aj | jq '."coretemp-isa-0000"')
	printf "Processor temperature:\n\n"
  for NUM in $SEQ; do
	printf "Core $(($NUM - 1)) - "
	CT=`printf "$SENSORS" | grep temp$NUM\_input | sed -e "s/.*\ //;s/.000//;s/,//"`
	colorize
  done
	sleep 0.1
	clear
done
