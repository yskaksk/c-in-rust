#!/bin/bash

cat <<EOF | gcc -xc -c -o tmp2.o -
int ret3() { return 3; }
int ret5() { return 5; }
int add(int x, int y) { return x+y; }
int sub(int x, int y) { return x-y; }

int add6(int a, int b, int c, int d, int e, int f) {
  return a+b+c+d+e+f;
}
EOF

assert() {
    expected="$1"
    input="$2"

    ./target/debug/lang "$input" > tmp.s

    gcc -static -o tmp tmp.s tmp2.o
    ./tmp
    actual="$?"

    if [ "$actual" = "$expected" ]; then
        echo "$input => $actual"
    else
        echo "$input => $expected expected, but got $actual"
        exit 1
    fi
}

echo building...
cargo build -q

echo testing...
assert 0 "main() {return 0;}"
assert 42 "main() {return 42;}"
assert 21 "main() {return 5+20-4;}"
assert 41 "main() {return  12 + 34 - 5 ;}"
assert 47 "main() {return 5+6*7;}"
assert 15 "main() {return 5*(9-6);}"
assert 4 "main() {return (3+5)/2;}"
assert 10 "main() {return -10+20;}"
assert 10 "main() {return - -10;}"
assert 10 "main() {return - - +10;}"

assert 0 "main() {return 0==1;}"
assert 1 "main() {return 42==42;}"
assert 1 "main() {return 0!=1;}"
assert 0 "main() {return 42!=42;}"

assert 1 "main() {return 0<1;}"
assert 0 "main() {return 1<1;}"
assert 0 "main() {return 2<1;}"
assert 1 "main() {return 0<=1;}"
assert 1 "main() {return 1<=1;}"
assert 0 "main() {return 2<=1;}"

assert 1 "main() {return 1>0;}"
assert 0 "main() {return 1>1;}"
assert 0 "main() {return 1>2;}"
assert 1 "main() {return 1>=0;}"
assert 1 "main() {return 1>=1;}"
assert 0 "main() {return 1>=2;}"

assert 2 "main() {a=1;return a+1;}"
assert 6 "main() {a=2;b=3;return a*b;}"
assert 6 "main() {ab=2;cd=3;return ab*cd;}"
assert 6 "main() {ab=2;cd=3;return ab*cd;}"

assert 1 "main() {a=1;if (a==1) return a;}"
assert 2 "main() {a=1;b=2;if (a!=1) return a; else return b;}"
assert 16 "main() {a=1;while (a<10) a=a*2; return a;}"
assert 8 "main() {a=1;for(c=0;c<3;c=c+1;)a=a*2;return a;}"
assert 8 "main() {a=0;b=1;while(a<3){a=a+1;b=b*2;}return b;}"

assert 3 "main() {return ret3();}"
assert 5 "main() {return ret5();}"
assert 8 "main() {return add(3, 5);}"
assert 2 "main() {return sub(5, 3);}"
assert 21 "main() {return add6(1,2,3,4,5,6);}"

assert 6 "main() {return f(2, 3);} f(x, y) {return x*y;}"
assert 7 "main() {return f(1, 2, 3);} f(x, y, z) {return x+y*z;}"
assert 55 'main() { return fib(9); } fib(x) { if (x<=1) return 1; return fib(x-1) + fib(x-2); }'

echo OK
