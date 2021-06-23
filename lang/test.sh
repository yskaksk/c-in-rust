#!/bin/bash
assert() {
    expected="$1"
    input="$2"

    ./target/debug/lang "$input" > tmp.s

    cc -o tmp tmp.s
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

assert 0 "0;"
assert 42 "42;"
assert 21 "5+20-4;"
assert 41 " 12 + 34 - 5 ;"
assert 47 "5+6*7;"
assert 15 "5*(9-6);"
assert 4 "(3+5)/2;"
assert 10 "-10+20;"
assert 10 "- -10;"
assert 10 "- - +10;"

assert 0 "0==1;"
assert 1 "42==42;"
assert 1 "0!=1;"
assert 0 "42!=42;"

assert 1 "0<1;"
assert 0 "1<1;"
assert 0 "2<1;"
assert 1 "0<=1;"
assert 1 "1<=1;"
assert 0 "2<=1;"

assert 1 "1>0;"
assert 0 "1>1;"
assert 0 "1>2;"
assert 1 "1>=0;"
assert 1 "1>=1;"
assert 0 "1>=2;"

assert 2 "a=1;a+1;"
assert 6 "a=2;b=3;a*b;"

assert 6 "ab=2;cd=3;ab*cd;"

assert 6 "ab=2;cd=3;return ab*cd;"
assert 1 "a=1;if (a==1) return a;"
assert 2 "a=1;b=2;if (a!=1) return a; else return b;"

assert 16 "a=1;while (a<10) a=a*2; return a;"

echo OK
