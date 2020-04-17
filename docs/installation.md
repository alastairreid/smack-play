# Installation

You can get Smack from
[github](https://github.com/smackers/smack/)
and the
[installation
page](https://github.com/smackers/smack/blob/master/docs/installation.md)
is the definitive place to look for install info.

## OSX install

In case it is helpful, here are the actual commands I used
to install on my OSX machine.
(If this doesn't work for you, go back to the
[official installation
page](https://github.com/smackers/smack/blob/master/docs/installation.md)
and consider sending me a pull request to fix the instructions.

### Prerequisites

Most of the prerequisites are easy except that you need to use old
versions of

- LLVM: must be version 8
- Rust compiler: must be a version built against LLVM version 8
- Z3: must use 4.5.0 or earlier to avoid
  a [known issue](https://github.com/smackers/smack/issues/521)
  that affects Corral/Boogie.

```
# based on homebrew formula for Z3 4.8.7
wget https://github.com/Z3Prover/z3/archive/z3-4.5.0.tar.gz
tar zxf z3-4.5.0.tar.gz
cd z3-z3-4.5.0/
python3 scripts/mk_make.py --prefix=$HOME/local --staticlib
make -C build -j
make -C build install

brew install llvm@8

export PATH="$HOME/homebrew/opt/llvm@8/bin:$PATH"
export LDFLAGS="-L$HOME/homebrew/opt/llvm@8/lib"
export CPPFLAGS="-I$HOME/homebrew/opt/llvm@8/include"

# consider doing this:
echo 'export PATH="$HOME/homebrew/opt/llvm@8/bin:$PATH"' >> ~/.bash_profile

# you need a version of the Rust compiler that was built using LLVM 8
# so install a slightly older version
rustup toolchain install 1.30.0
rustup default 1.30.0
```

The remaining prerequisites seem to be fine with the latest releases

```
brew install boost ninja
brew cask install mono-mdk
echo 'export PATH="$HOME/.dotnet/tools:$PATH"' >> ~/.bash_profile
export PATH="$HOME/.dotnet/tools:$PATH"
dotnet tool install --global boogie
dotnet tool install --global Corral
```


### Building SMACK

```
# I probably should have used an official release - but, apparently I did not
# wget https://github.com/smackers/smack/archive/v2.4.1.tar.gz

git clone git@github.com:smackers/smack.git
cd smack

git submodule init
git submodule update
# build SMACK
mkdir build
cd build
cmake -DCMAKE_INSTALL_PREFIX=$HOME/local -DCMAKE_C_COMPILER=clang -DCMAKE_CXX_COMPILER=clang++ -DCMAKE_BUILD_TYPE=Debug .. -G Ninja -Wnodev
ninja install
```


### Testing install

Test using the
[examples/simple/simple.c](https://github.com/smackers/smack/blob/master/examples/simple/simple.c)
demo

```
smack examples/simple/simple.c
```
producing output like this

```
SMACK program verifier version 2.4.1
SMACK found no errors with unroll bound 1.
```

Note: When I first tried this, I got an error in Corral
due to [a known issue with recent Z3 issues](https://github.com/smackers/smack/issues/521).
This is avoided by installing an older version of Z3 as described above.


### Running SMACK regression tests

After installation, run the regression tests like this.
(Varying the number of threads to match your machine and whether you are
doing this in the background during a video conference call :smiley:)

```
pip3 import psutil pyyaml
cd test
./regtest.py --threads 6 --languages c
./regtest.py --threads 6 --languages rust
stty sane # fix terminal after running script
```

The C tests take a while to run and produce this report

```
 ELAPSED TIME [609.86s]
 PASSED count: 209
 FAILED count: 1
 TIMEOUT count: 0
 UNKNOWN count: 25
```

The Rust tests are faster and produce this report

```
 ELAPSED TIME [200.83s]
 PASSED count: 40
 FAILED count: 0
 TIMEOUT count: 0
 UNKNOWN count: 0
```

Note: When I first ran this, I saw around 75% `PASSED` and 25% reported
as `UNKNOWN`.
This was also fixed by installing an older version of Z3.
