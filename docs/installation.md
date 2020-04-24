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

### Fixing LLDB

On OSX, you need to code-sign lldb so that it can be used
by following the instructions [on this
page](https://opensource.apple.com/source/lldb/lldb-69/docs/code-signing.txt)
and then executing this command.  (You might want to do the same for gdb as well)

```
codesign -s lldb-codesign `which lldb`
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

#### Fixing contract support

(This fixes [issue #557](https://github.com/smackers/smack/issues/557).)

Note: If you plan to use SMACK's support for modular verification using contracts and
loop invariants (see [testsuite
examples](https://github.com/smackers/smack/tree/master/test/c/contracts),
you will soon find that the latest version of `boogie`
no longer supports the flag `/noinfer` that was previously needed.
So either install a version of boogie from before 10th April 2020 or
apply this change to SMACK and reinstall.

```
diff --git a/share/smack/top.py b/share/smack/top.py
index 021437c0..5baca734 100755
--- a/share/smack/top.py
+++ b/share/smack/top.py
@@ -470,7 +470,7 @@ def verify_bpl(args):
   elif args.verifier == 'boogie' or args.modular:
     command = ["boogie"]
     command += [args.bpl_file]
-    command += ["/nologo", "/noinfer", "/doModSetAnalysis"]
+    command += ["/nologo", "/doModSetAnalysis"]
     command += ["/timeLimit:%s" % args.time_limit]
     command += ["/errorLimit:%s" % args.max_violations]
     if not args.modular:
```

#### Fixing replay support

(This is reported as [issue #570](https://github.com/smackers/smack/issues/570).)

Apply the following hack to SMACK.
(I believe that the better fix would be to just change 'message' to 'msg'
but that was not enough on my machine.)

```
diff --git a/share/smack/replay.py b/share/smack/replay.py
index 8064eafd..ae59f005 100644
--- a/share/smack/replay.py
+++ b/share/smack/replay.py
@@ -41,7 +41,7 @@ def replay_error_trace(verifier_output, args):
       print("Error-trace replay failed.")

   except Exception as err:
-    print("Error-trace replay caught", err.message)
+    print("Error-trace replay caught", err)

   return False

@@ -51,7 +51,8 @@ def detect_missing_definitions(bc_file):
   try:
     try_command(['clang', bc_file])
   except Exception as err:
-    for line in err.message.split("\n"):
+    msg = repr(err).replace("\\n","\n")
+    for line in msg.split("\n"):
       m = re.search(r'\"_(.*)\", referenced from:', line) or re.search(r'undefined reference to `(.*)\'', line)
       if m:
         missing.append(m.group(1))
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
