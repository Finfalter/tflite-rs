#!/bin/sh

thisDir=$(cd $(dirname $0) && pwd)

echo -n "removing unnecessary files from download directory .."

rm -rf ${thisDir}/tensorflow/tensorflow/lite/tools/make/downloads/googletest
rm -rf ${thisDir}/tensorflow/tensorflow/lite/tools/make/downloads/gemmlowp/todo
rm -rf ${thisDir}/tensorflow/tensorflow/lite/tools/make/downloads/gemmlowp/test
rm -rf ${thisDir}/tensorflow/tensorflow/lite/tools/make/downloads/gemmlowp/meta/generators
rm -rf ${thisDir}/tensorflow/tensorflow/lite/tools/make/downloads/gemmlowp/doc
rm -rf ${thisDir}/tensorflow/tensorflow/lite/tools/make/downloads/flatbuffers/tests
rm -rf ${thisDir}/tensorflow/tensorflow/lite/tools/make/downloads/flatbuffers/samples
rm -rf ${thisDir}/tensorflow/tensorflow/lite/tools/make/downloads/flatbuffers/rust
rm -rf ${thisDir}/tensorflow/tensorflow/lite/tools/make/downloads/flatbuffers/python
rm -rf ${thisDir}/tensorflow/tensorflow/lite/tools/make/downloads/flatbuffers/php
rm -rf ${thisDir}/tensorflow/tensorflow/lite/tools/make/downloads/flatbuffers/net
rm -rf ${thisDir}/tensorflow/tensorflow/lite/tools/make/downloads/flatbuffers/lua
rm -rf ${thisDir}/tensorflow/tensorflow/lite/tools/make/downloads/flatbuffers/java
rm -rf ${thisDir}/tensorflow/tensorflow/lite/tools/make/downloads/flatbuffers/grpc
rm -rf ${thisDir}/tensorflow/tensorflow/lite/tools/make/downloads/flatbuffers/go
rm -rf ${thisDir}/tensorflow/tensorflow/lite/tools/make/downloads/flatbuffers/docs
rm -rf ${thisDir}/tensorflow/tensorflow/lite/tools/make/downloads/flatbuffers/dart
rm -rf ${thisDir}/tensorflow/tensorflow/lite/tools/make/downloads/flatbuffers/android
rm -rf ${thisDir}/tensorflow/tensorflow/lite/tools/make/downloads/farmhash/dev
rm -rf ${thisDir}/tensorflow/tensorflow/lite/tools/make/downloads/farmhash/m4
rm -rf ${thisDir}/tensorflow/tensorflow/lite/tools/make/downloads/eigen/unsupported/test
rm -rf ${thisDir}/tensorflow/tensorflow/lite/tools/make/downloads/eigen/unsupported/doc
rm -rf ${thisDir}/tensorflow/tensorflow/lite/tools/make/downloads/eigen/test
rm -rf ${thisDir}/tensorflow/tensorflow/lite/tools/make/downloads/eigen/failtest
rm -rf ${thisDir}/tensorflow/tensorflow/lite/tools/make/downloads/eigen/doc
rm -rf ${thisDir}/tensorflow/tensorflow/lite/tools/make/downloads/eigen/demos
rm -rf ${thisDir}/tensorflow/tensorflow/lite/tools/make/downloads/eigen/bench
rm -rf ${thisDir}/tensorflow/tensorflow/lite/tools/make/downloads/absl/ci

echo " done"