FROM public.ecr.aws/lambda/provided:al2
COPY target/release/bootstrap ${LAMBDA_RUNTIME_DIR}/bootstrap
CMD [ "lambda-handler" ]
