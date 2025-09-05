FROM python:3.13.7-alpine3.21
WORKDIR /app
ENV blabber_host=0.0.0.0
ENV blabber_port=80
ENV blabber_credfile=../credfile
COPY ./server.py .
WORKDIR /app/state/rooms
CMD [ "/app/server.py", "serve" ]
