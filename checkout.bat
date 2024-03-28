set branch=%1
echo checkouting %branch%
git checkout %branch%
git submodule foreach --recursive git checkout %branch%
