FROM public.ecr.aws/lambda/provided:al2
COPY target/release/lambda-container ${LAMBDA_RUNTIME_DIR}/bootstrap
CMD [ "lambda-handler" ]
