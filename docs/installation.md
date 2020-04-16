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

    brew install boost llvm@8 ninja z3

    brew cask install mono-mdk
    echo 'export PATH="$HOME/.dotnet/tools:$PATH"' >> ~/.bash_profile
    export PATH="$HOME/.dotnet/tools:$PATH"
    dotnet tool install --global boogie
    dotnet tool install --global Corral

    export PATH="$HOME/homebrew/opt/llvm@8/bin:$PATH"
    export LDFLAGS="-L$HOME/homebrew/opt/llvm@8/lib"
    export CPPFLAGS="-I$HOME/homebrew/opt/llvm@8/include"

    # consider doing this:
    echo 'export PATH="$HOME/homebrew/opt/llvm@8/bin:$PATH"' >> ~/.bash_profile

    # you need a version of the Rust compiler that was built using LLVM 8
    # so install a slightly older version
    rustup toolchain install 1.30.0
    rustup default 1.30.0


### Building SMACK

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


### Testing install

Test using this demo

    smack examples/simple/simple.c

Unfortunately, this currently generates an error in Corral
(apparently while generating an error message).
(This is a [known issue](https://github.com/smackers/smack/issues/521) that
may be due to the version of Z3 being used.)

    LB: Took 0.00 s
    Verifying program while tracking: {assertsPassed}
    Program has a potential bug: False bug
    Verifying program while tracking: {assertsPassed, $CurrAddr}
    Program has a potential bug: False bug
    Verifying program while tracking: {assertsPassed, $CurrAddr, $M.0}
    Unhandled exception. CoreLib.InsufficientDetailsToConstructCexPath: StratifiedInliningErrorReporter: Must find a successor
    at CoreLib.StratifiedInliningErrorReporter.GetAbsyTraceLabels(StratifiedVC svc, IList`1 labels) in /home/travis/build/boogie-org/corral/source/CoreLib/StratifiedInlining.cs:line 4195
    at CoreLib.StratifiedInliningErrorReporter.GetAbsyTrace(StratifiedVC svc, IList`1 labels) in /home/travis/build/boogie-org/corral/source/CoreLib/StratifiedInlining.cs:line 4190
