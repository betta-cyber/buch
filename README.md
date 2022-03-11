# install

Install LLVM 13 and the Linux kernel headers

```
# apt-get update \
  && apt-get -y install \
       wget \
       build-essential \
       software-properties-common \
       lsb-release \
       libelf-dev \
       linux-headers-generic \
       pkg-config \
  && wget https://apt.llvm.org/llvm.sh && chmod +x llvm.sh && ./llvm.sh 13 && rm -f ./llvm.sh
# llvm-config-13 --version | grep 13
```
