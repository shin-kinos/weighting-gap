# weighting-gap

## Description
- A program that calculates gap penalties a site taking account of sequence weighing. 
- It supports two types of weighting, position-based method[1] and distance-based method[2]. 

## Compilation 
You can compile the program with `Cargo`. 🦀📦

[e.g.]

``` 
% cd weighting-gap-main
% cargo build --release
``` 

Then the object file will be generated in `./target/release` directory. 

## Input file format 
Aligned Multi-FASTA format in amino acid sequences.  
See sample files in `demo` directory. 

## Usage 
- `-i` : Input file name, REQUIRED.
- `-o` : Output file name, REQUIRED.
- `-m` : Method of weighting, position-based or distance-based. 

[e.g.]

```
% ./weighting-gap -i input.fasta -o result_out.txt -m hen 
``` 
Type `-h` to see other available options.

## Output 
- Output data are written by `loop` structure, an element of the Self-Defining Text Archive and Retrival (STAR) format, often be used in molecular-structure sciences.
- It has two categories, `_gap_penalty` and `_weighting`.
- If necessary, see the documentation about STAR format (https://pdb101.rcsb.org/learn/guide-to-understanding-pdb-data/beginner%E2%80%99s-guide-to-pdb-structures-and-the-pdbx-mmcif-format).

[e.g.]

```
demo_02.fasta
#
loop_
_gap_penalty.number_site
_gap_penalty.gap_penalty_value
1	1.0000
2	1.0000
3	1.0000
4	0.6833
5	1.0000
#
loop_
_weighting.number_sequence
_weighting.weighting_value
_weighting.title_line
1	0.3167	>NIFE_CLOPA_GAP
2	0.2500	>NIFD_AZOVI_GAP
3	0.1833	>NIFD_BRAJA_GAP
4	0.2500	>NIFK_ANASP_GAP
#

```

## References 

1. Henikoff, Steven, and Jorja G. Henikoff. "Position-based sequence weights." Journal of molecular biology 243.4 (1994): 574-578.
2. Vingron, Martin, and Patrick Argos. "A fast and sensitive multiple sequence alignment algorithm." Bioinformatics 5.2 (1989): 115-121.
