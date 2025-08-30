FROM alpine:3.20
WORKDIR /app
RUN apk add --no-cache bash coreutils util-linux busybox-extras
RUN cp /bin/busybox /bin/busybox.bak
RUN cp /bin/busybox-extras /bin/busybox
ENV blabber_bind=0.0.0.0:80
COPY server.sh add_user.sh .
COPY root root
CMD [ "./server.sh" ]
