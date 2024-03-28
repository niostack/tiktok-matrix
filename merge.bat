echo merging main to current branch
git merge main
git submodule foreach --recursive git merge main
