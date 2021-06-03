# selene
BGZ/TBI Reader

## Example

```shell
selene tabix --data-file ~/lunaris/vep/data/all_sites.vep.tsv.gz \
    --input-file ~/lunaris/vep/test/inputs/input_selene.vcf --col-ref Ref --col-alt Alt --output-file tmp/out.tsv \
    --cache-misses-file tmp/misses.tsv
```