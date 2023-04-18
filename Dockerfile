FROM --platform=$BUILDPLATFORM rust:alpine as builder

WORKDIR /usr/src

# Create blank project
RUN USER=root cargo new --bin sf-express

# We want dependencies cached, so copy those first.
COPY Cargo.toml Cargo.lock /usr/src/sf-express/

# Set the working directory
WORKDIR /usr/src/sf-express

# This is a dummy build to get the dependencies cached.
RUN apk add musl-dev openssl 

RUN cargo build --release

RUN rm src/*.rs

# Now copy in the rest of the sources
COPY src /usr/src/sf-express/src/

# This is the actual application build.
RUN cargo build  --release

### Runtime
FROM --platform=$TARGETPLATFORM alpine:3.16.0 AS runtime

ARG $TIMEZONE 

WORKDIR /

# Copy application binary from builder image
COPY --from=builder /usr/src/sf-express/target/release/sf-express /usr/local/bin

RUN apk --no-cache add tzdata bash \
    && cp "/usr/share/zoneinfo/Asia/Shanghai" /etc/localtime \
    && echo "Asia/Shanghai" > /etc/timezone \
    && echo "5 0 * * * sf-express >> /var/log/sf-express.log" >> /etc/crontabs/root

# Run the application
CMD ["/usr/sbin/crond", "-f", "-d", "0"]]