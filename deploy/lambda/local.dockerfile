FROM public.ecr.aws/lambda/provided:al2
RUN yum install -y gcc openssl-devel

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable
ENV PATH $PATH:/root/.cargo/bin
RUN rustup install 1.63.0

RUN yum install -y python3 python3-devel make && pip3 install aws-sam-cli Jinja2==2.11.3
