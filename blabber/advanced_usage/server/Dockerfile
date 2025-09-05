FROM alpine:3.20
WORKDIR /app
RUN apk add --no-cache python3
ENV blabber_host=0.0.0.0
ENV blabber_port=80
ENV blabber_credfile=../credfile
COPY server.py .
WORKDIR /app/state/rooms
CMD [ "/app/server.py" ]
