# PARTITION 

**Work in progress**

Very efficient implementation of the approximation algorithm solving the [partition](https://en.wikipedia.org/wiki/Partition_problem) problem.

## Building PDF from LaTeX

### Using Docker

1. **Pull the Docker Image:**
   ```bash
   docker pull blang/latex
   ```

2. **Compile the Document:**
   ```bash
   docker run --rm -v $(pwd)/paper:/data blang/latex latexmk -pdf /data/main.tex
   ```
