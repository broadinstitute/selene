#!/bin/bash

vepCmd=$1
input=$2
assembly=$3
cpus=$4
fasta=$5
dirCache=$6
dirPlugins=$7
dbNSFP=$8
output=$9
warnings=${10}

echo "= = = Begin of vep invocation"
cat <<COMMANDLINE
$vepCmd -i $input \
--format vcf \
--fork $cpus \
--force_overwrite \
--no_stats \
--offline \
--fasta $fasta \
--tab \
--cache \
--dir_cache $dirCache \
--dir_plugins $dirPlugins \
--polyphen b \
--sift b \
--ccds \
--canonical \
--appris \
--tsl \
--biotype \
--regulatory \
--assembly $assembly \
--flag_pick_allele \
--pick_order tsl,biotype,appris,rank,ccds,canonical,length \
--domains flags \
--plugin LoF \
--plugin LoFtool \
--plugin dbNSFP,${dbNSFP},ALL \
--symbol \
--output_file $output \
--warning_file $warnings
COMMANDLINE
echo "= = = End of vep invocation"

$vepCmd -i $input \
--format vcf \
--fork $cpus \
--force_overwrite \
--no_stats \
--offline \
--fasta $fasta \
--tab \
--cache \
--dir_cache $dirCache \
--dir_plugins $dirPlugins \
--polyphen b \
--sift b \
--ccds \
--canonical \
--appris \
--tsl \
--biotype \
--regulatory \
--assembly $assembly \
--flag_pick_allele \
--pick_order tsl,biotype,appris,rank,ccds,canonical,length \
--domains flags \
--plugin LoF \
--plugin LoFtool \
--plugin dbNSFP,${dbNSFP},ALL \
--symbol \
--output_file $output \
--warning_file $warnings
